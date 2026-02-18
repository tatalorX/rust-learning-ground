# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
CRUD operations for School Platform.
"""
import random
import string
from datetime import datetime, timezone, timedelta
from typing import Optional, List, Dict, Any

from sqlalchemy import select, update, delete, func, and_
from sqlalchemy.ext.asyncio import AsyncSession

from app.models_school import (
    UserSchoolProfile, Classroom, ClassroomMember, Assignment, AssignmentSubmission,
    RankDefinition, TitleDefinition, UserTitle, TreatTransaction, TreatReward,
    LeaderboardEntry, GradeContentMapping, ActivityLog,
    UserRole, GradeLevel, RankTier
)
from app.schemas_school import (
    UserSchoolProfileCreate, UserSchoolProfileUpdate,
    ClassroomCreate, ClassroomUpdate,
    AssignmentCreate, AssignmentUpdate
)


# ============================================================================
# UTILITY FUNCTIONS
# ============================================================================

def generate_classroom_code(length: int = 8) -> str:
    """Generate a random classroom join code."""
    chars = string.ascii_uppercase + string.digits
    return ''.join(random.choices(chars, k=length))


# ============================================================================
# USER SCHOOL PROFILE CRUD
# ============================================================================

async def get_user_school_profile(db: AsyncSession, user_id: int, auto_create: bool = True) -> Optional[UserSchoolProfile]:
    """Get or create user school profile."""
    result = await db.execute(
        select(UserSchoolProfile).where(UserSchoolProfile.user_id == user_id)
    )
    profile = result.scalar_one_or_none()
    
    if not profile and auto_create:
        # Auto-create profile if it doesn't exist (caller must commit)
        profile = UserSchoolProfile(user_id=user_id)
        db.add(profile)
        await db.flush()  # Flush to get the ID, but don't commit
        await db.refresh(profile)
    
    return profile


async def create_user_school_profile(
    db: AsyncSession, user_id: int, data: UserSchoolProfileCreate
) -> UserSchoolProfile:
    """Create a new school profile for user."""
    profile = UserSchoolProfile(
        user_id=user_id,
        role=data.role,
        grade_level=data.grade_level
    )
    db.add(profile)
    await db.flush()
    await db.refresh(profile)
    return profile


async def update_user_school_profile(
    db: AsyncSession, user_id: int, data: UserSchoolProfileUpdate
) -> Optional[UserSchoolProfile]:
    """Update user's school profile."""
    profile = await get_user_school_profile(db, user_id)
    if not profile:
        return None
    
    # Update fields
    if data.grade_level is not None:
        profile.grade_level = data.grade_level
    if data.display_title is not None:
        profile.display_title = data.display_title
    if data.profile_public is not None:
        profile.profile_public = data.profile_public
    
    await db.flush()
    await db.refresh(profile)
    return profile


# ============================================================================
# XP & RANK CRUD
# ============================================================================

async def add_xp(db: AsyncSession, user_id: int, amount: int, source: str = "") -> UserSchoolProfile:
    """Add XP to user and check for rank up."""
    profile = await get_user_school_profile(db, user_id)
    
    old_rank = profile.current_rank
    profile.total_xp += amount
    profile.update_rank()
    
    # Award treats on rank up
    if profile.current_rank != old_rank:
        rank_treats = {
            RankTier.APPRENTICE: 10,
            RankTier.JOURNEYMAN: 20,
            RankTier.ADEPT: 35,
            RankTier.EXPERT: 50,
            RankTier.MASTER: 75,
            RankTier.GRANDMASTER: 100,
            RankTier.LEGEND: 150
        }
        treats = rank_treats.get(profile.current_rank, 0)
        if treats > 0:
            profile.treats += treats
            # Log treat transaction
            transaction = TreatTransaction(
                user_id=user_id,
                amount=treats,
                balance_after=profile.treats,
                transaction_type="rank_up",
                description=f"Rank up to {profile.current_rank.value}"
            )
            db.add(transaction)
    
    # Log XP transaction
    transaction = TreatTransaction(
        user_id=user_id,
        amount=0,  # XP is tracked separately
        balance_after=profile.treats,
        transaction_type="xp_gain",
        description=f"Gained {amount} XP from {source}"
    )
    db.add(transaction)
    
    await db.flush()
    await db.refresh(profile)
    return profile


async def get_rank_definitions(db: AsyncSession) -> List[RankDefinition]:
    """Get all rank definitions ordered by min_xp."""
    result = await db.execute(
        select(RankDefinition).order_by(RankDefinition.min_xp)
    )
    return list(result.scalars().all())


# ============================================================================
# CLASSROOM CRUD
# ============================================================================

async def create_classroom(
    db: AsyncSession, teacher_id: int, data: ClassroomCreate
) -> Classroom:
    """Create a new classroom."""
    # Generate unique code
    code = generate_classroom_code()
    
    classroom = Classroom(
        teacher_id=teacher_id,
        name=data.name,
        description=data.description,
        subject=data.subject,
        code=code,
        grade_level=data.grade_level,
        max_students=data.max_students or 30
    )
    db.add(classroom)
    await db.flush()
    await db.refresh(classroom)
    
    # Log activity
    await log_activity(
        db,
        user_id=teacher_id,
        action="classroom_created",
        target_type="classroom",
        target_id=classroom.id,
        classroom_id=classroom.id,
        details={"name": data.name}
    )
    
    return classroom


async def get_classroom(db: AsyncSession, classroom_id: int) -> Optional[Classroom]:
    """Get classroom by ID."""
    result = await db.execute(
        select(Classroom).where(Classroom.id == classroom_id)
    )
    return result.scalar_one_or_none()


async def get_classroom_by_code(db: AsyncSession, code: str) -> Optional[Classroom]:
    """Get classroom by join code."""
    result = await db.execute(
        select(Classroom).where(
            and_(Classroom.code == code, Classroom.is_active == True)
        )
    )
    return result.scalar_one_or_none()


async def list_teacher_classrooms(db: AsyncSession, teacher_id: int) -> List[Classroom]:
    """Get all classrooms taught by a teacher."""
    result = await db.execute(
        select(Classroom)
        .where(Classroom.teacher_id == teacher_id)
        .where(Classroom.is_archived == False)
        .order_by(Classroom.created_at.desc())
    )
    return list(result.scalars().all())


async def list_student_classrooms(db: AsyncSession, student_id: int) -> List[Dict[str, Any]]:
    """Get all classrooms a student is a member of."""
    result = await db.execute(
        select(Classroom, ClassroomMember)
        .join(ClassroomMember, Classroom.id == ClassroomMember.classroom_id)
        .where(ClassroomMember.user_id == student_id)
        .where(ClassroomMember.is_active == True)
        .where(Classroom.is_active == True)
        .order_by(ClassroomMember.joined_at.desc())
    )
    
    classrooms = []
    for classroom, member in result.all():
        classrooms.append({
            "classroom": classroom,
            "joined_at": member.joined_at,
            "problems_solved": member.problems_solved_in_class,
            "xp_earned": member.total_xp_earned
        })
    
    return classrooms


async def update_classroom(
    db: AsyncSession, classroom_id: int, data: ClassroomUpdate
) -> Optional[Classroom]:
    """Update classroom details."""
    classroom = await get_classroom(db, classroom_id)
    if not classroom:
        return None
    
    if data.name is not None:
        classroom.name = data.name
    if data.description is not None:
        classroom.description = data.description
    if data.subject is not None:
        classroom.subject = data.subject
    if data.grade_level is not None:
        classroom.grade_level = data.grade_level
    if data.max_students is not None:
        classroom.max_students = data.max_students
    
    await db.flush()
    await db.refresh(classroom)
    
    # Log activity
    await log_activity(
        db,
        user_id=classroom.teacher_id,
        action="classroom_updated",
        target_type="classroom",
        target_id=classroom_id,
        classroom_id=classroom_id
    )
    
    return classroom


async def archive_classroom(db: AsyncSession, classroom_id: int) -> bool:
    """Archive a classroom (soft delete)."""
    classroom = await get_classroom(db, classroom_id)
    if not classroom:
        return False
    
    classroom.is_archived = True
    classroom.is_active = False
    classroom.archived_at = datetime.now(timezone.utc)
    
    await db.flush()
    
    # Log activity
    await log_activity(
        db,
        user_id=classroom.teacher_id,
        action="classroom_archived",
        target_type="classroom",
        target_id=classroom_id,
        classroom_id=classroom_id
    )
    
    return True


async def is_classroom_teacher(db: AsyncSession, user_id: int, classroom_id: int) -> bool:
    """Check if user is the teacher of a classroom."""
    result = await db.execute(
        select(Classroom).where(
            and_(
                Classroom.id == classroom_id,
                Classroom.teacher_id == user_id,
                Classroom.is_archived == False
            )
        )
    )
    return result.scalar_one_or_none() is not None


async def is_classroom_member(db: AsyncSession, user_id: int, classroom_id: int) -> bool:
    """Check if user is a member of a classroom."""
    result = await db.execute(
        select(ClassroomMember).where(
            and_(
                ClassroomMember.classroom_id == classroom_id,
                ClassroomMember.user_id == user_id,
                ClassroomMember.is_active == True
            )
        )
    )
    return result.scalar_one_or_none() is not None


# ============================================================================
# CLASSROOM MEMBERSHIP CRUD
# ============================================================================

async def join_classroom(db: AsyncSession, user_id: int, classroom_id: int) -> bool:
    """Add a student to a classroom."""
    # Check if already a member
    existing = await db.execute(
        select(ClassroomMember).where(
            and_(
                ClassroomMember.classroom_id == classroom_id,
                ClassroomMember.user_id == user_id
            )
        )
    )
    if existing.scalar_one_or_none():
        return False
    
    member = ClassroomMember(
        classroom_id=classroom_id,
        user_id=user_id
    )
    db.add(member)
    await db.flush()
    
    # Log activity
    await log_activity(
        db,
        user_id=user_id,
        action="joined_classroom",
        target_type="classroom",
        target_id=classroom_id,
        classroom_id=classroom_id
    )
    
    return True


async def leave_classroom(db: AsyncSession, user_id: int, classroom_id: int) -> bool:
    """Remove a student from a classroom."""
    result = await db.execute(
        delete(ClassroomMember).where(
            and_(
                ClassroomMember.classroom_id == classroom_id,
                ClassroomMember.user_id == user_id
            )
        )
    )
    
    if result.rowcount > 0:
        # Log activity
        await log_activity(
            db,
            user_id=user_id,
            action="left_classroom",
            target_type="classroom",
            target_id=classroom_id,
            classroom_id=classroom_id
        )
        return True
    
    return False


async def remove_classroom_member(
    db: AsyncSession, classroom_id: int, member_id: int, teacher_id: int
) -> bool:
    """Remove a member from classroom (teacher only)."""
    # Verify teacher owns classroom
    if not await is_classroom_teacher(db, teacher_id, classroom_id):
        return False
    
    result = await db.execute(
        delete(ClassroomMember).where(
            and_(
                ClassroomMember.classroom_id == classroom_id,
                ClassroomMember.user_id == member_id
            )
        )
    )
    
    if result.rowcount > 0:
        # Log activity
        await log_activity(
            db,
            user_id=teacher_id,
            action="removed_member",
            target_type="classroom_member",
            target_id=member_id,
            classroom_id=classroom_id,
            details={"removed_user_id": member_id}
        )
        return True
    
    return False


async def get_classroom_members(db: AsyncSession, classroom_id: int) -> List[Dict[str, Any]]:
    """Get all members of a classroom with their profiles."""
    from app.models import User
    
    result = await db.execute(
        select(ClassroomMember, User, UserSchoolProfile)
        .join(User, ClassroomMember.user_id == User.id)
        .outerjoin(UserSchoolProfile, User.id == UserSchoolProfile.user_id)
        .where(ClassroomMember.classroom_id == classroom_id)
        .where(ClassroomMember.is_active == True)
    )
    
    members = []
    for member, user, profile in result.all():
        members.append({
            "user_id": user.id,
            "username": user.username,
            "display_name": user.display_name or user.username,
            "email": user.email,
            "joined_at": member.joined_at,
            "is_active": member.is_active,
            "problems_solved_count": member.problems_solved_in_class,
            "xp_earned": member.total_xp_earned,
            "current_rank": profile.current_rank if profile else None,
            "total_xp": profile.total_xp if profile else 0,
            "avatar_url": user.github_avatar_url
        })
    
    return members


# ============================================================================
# ACTIVITY LOGGING
# ============================================================================

async def log_activity(
    db: AsyncSession,
    user_id: Optional[int],
    action: str,
    target_type: str,
    target_id: Optional[int] = None,
    classroom_id: Optional[int] = None,
    details: Optional[Dict[str, Any]] = None,
    ip_address: Optional[str] = None
) -> ActivityLog:
    """Log an activity in the system."""
    activity = ActivityLog(
        user_id=user_id,
        action=action,
        target_type=target_type,
        target_id=target_id,
        classroom_id=classroom_id,
        details=details or {},
        ip_address=ip_address
    )
    db.add(activity)
    await db.flush()
    return activity


async def get_recent_activity(
    db: AsyncSession,
    classroom_id: int,
    limit: int = 20,
    hours: int = 168  # 1 week
) -> List[Dict[str, Any]]:
    """Get recent activity for a classroom."""
    from app.models import User
    
    since = datetime.now(timezone.utc) - timedelta(hours=hours)
    
    result = await db.execute(
        select(ActivityLog, User)
        .outerjoin(User, ActivityLog.user_id == User.id)
        .where(ActivityLog.classroom_id == classroom_id)
        .where(ActivityLog.created_at >= since)
        .order_by(ActivityLog.created_at.desc())
        .limit(limit)
    )
    
    activities = []
    for activity, user in result.all():
        activities.append({
            "id": activity.id,
            "user_id": activity.user_id,
            "username": user.username if user else "System",
            "display_name": user.display_name if user else "System",
            "action": activity.action,
            "target_type": activity.target_type,
            "target_id": activity.target_id,
            "details": activity.details,
            "created_at": activity.created_at.isoformat() if activity.created_at else None
        })
    
    return activities


# ============================================================================
# ASSIGNMENT CRUD
# ============================================================================

async def create_assignment(
    db: AsyncSession, classroom_id: int, created_by: int, data: AssignmentCreate
) -> Assignment:
    """Create a new assignment."""
    assignment = Assignment(
        classroom_id=classroom_id,
        created_by=created_by,
        title=data.title,
        description=data.description,
        exercise_ids=data.exercise_ids,
        due_date=data.due_date,
        xp_reward=data.xp_reward or 0,
        treat_reward=data.treat_reward or 0,
        status=data.status or "draft"
    )
    db.add(assignment)
    await db.flush()
    await db.refresh(assignment)
    
    # Log activity
    await log_activity(
        db,
        user_id=created_by,
        action="assignment_created",
        target_type="assignment",
        target_id=assignment.id,
        classroom_id=classroom_id,
        details={"title": data.title}
    )
    
    return assignment


async def get_assignment(db: AsyncSession, assignment_id: int) -> Optional[Assignment]:
    """Get assignment by ID."""
    result = await db.execute(
        select(Assignment).where(Assignment.id == assignment_id)
    )
    return result.scalar_one_or_none()


async def list_classroom_assignments(db: AsyncSession, classroom_id: int) -> List[Assignment]:
    """Get all assignments for a classroom."""
    result = await db.execute(
        select(Assignment)
        .where(Assignment.classroom_id == classroom_id)
        .order_by(Assignment.created_at.desc())
    )
    return list(result.scalars().all())


async def update_assignment(
    db: AsyncSession, assignment_id: int, data: AssignmentUpdate
) -> Optional[Assignment]:
    """Update an assignment."""
    assignment = await get_assignment(db, assignment_id)
    if not assignment:
        return None
    
    if data.title is not None:
        assignment.title = data.title
    if data.description is not None:
        assignment.description = data.description
    if data.exercise_ids is not None:
        assignment.exercise_ids = data.exercise_ids
    if data.due_date is not None:
        assignment.due_date = data.due_date
    if data.status is not None:
        assignment.status = data.status
        if data.status == "published" and not assignment.published_at:
            assignment.published_at = datetime.now(timezone.utc)
    if data.xp_reward is not None:
        assignment.xp_reward = data.xp_reward
    if data.treat_reward is not None:
        assignment.treat_reward = data.treat_reward
    
    await db.flush()
    await db.refresh(assignment)
    
    # Log activity
    await log_activity(
        db,
        user_id=assignment.created_by,
        action="assignment_updated",
        target_type="assignment",
        target_id=assignment_id,
        classroom_id=assignment.classroom_id
    )
    
    return assignment


async def delete_assignment(db: AsyncSession, assignment_id: int) -> bool:
    """Delete an assignment."""
    assignment = await get_assignment(db, assignment_id)
    if not assignment:
        return False
    
    classroom_id = assignment.classroom_id
    created_by = assignment.created_by
    
    await db.delete(assignment)
    await db.flush()
    
    # Log activity
    await log_activity(
        db,
        user_id=created_by,
        action="assignment_deleted",
        target_type="assignment",
        target_id=assignment_id,
        classroom_id=classroom_id
    )
    
    return True


# ============================================================================
# SUBMISSION CRUD
# ============================================================================

async def get_or_create_submission(
    db: AsyncSession, assignment_id: int, user_id: int
) -> AssignmentSubmission:
    """Get existing submission or create new one."""
    result = await db.execute(
        select(AssignmentSubmission).where(
            and_(
                AssignmentSubmission.assignment_id == assignment_id,
                AssignmentSubmission.user_id == user_id
            )
        )
    )
    submission = result.scalar_one_or_none()
    
    if not submission:
        submission = AssignmentSubmission(
            assignment_id=assignment_id,
            user_id=user_id,
            status="not_started"
        )
        db.add(submission)
        await db.flush()
    
    return submission


async def submit_exercise(
    db: AsyncSession,
    assignment_id: int,
    user_id: int,
    exercise_id: str,
    code: Optional[str] = None
) -> AssignmentSubmission:
    """Record an exercise submission."""
    submission = await get_or_create_submission(db, assignment_id, user_id)
    assignment = await get_assignment(db, assignment_id)
    
    # Mark exercise as completed
    if exercise_id not in submission.completed_exercises:
        submission.completed_exercises.append(exercise_id)
    
    # Update status
    total_exercises = len(assignment.exercise_ids) if assignment else 0
    completed = len(submission.completed_exercises)
    
    if completed >= total_exercises and total_exercises > 0:
        submission.status = "submitted"
        submission.submitted_at = datetime.now(timezone.utc)
        
        # Award XP and treats
        if assignment:
            profile = await get_user_school_profile(db, user_id)
            if assignment.xp_reward > 0:
                profile.total_xp += assignment.xp_reward
                profile.update_rank()
            if assignment.treat_reward > 0:
                profile.treats += assignment.treat_reward
            
            # Update classroom member stats
            await db.execute(
                update(ClassroomMember)
                .where(
                    and_(
                        ClassroomMember.classroom_id == assignment.classroom_id,
                        ClassroomMember.user_id == user_id
                    )
                )
                .values(
                    problems_solved_in_class=ClassroomMember.problems_solved_in_class + 1,
                    total_xp_earned=ClassroomMember.total_xp_earned + assignment.xp_reward
                )
            )
            
            submission.xp_earned = assignment.xp_reward
            submission.treats_earned = assignment.treat_reward
    else:
        submission.status = "in_progress"
    
    if not submission.started_at:
        submission.started_at = datetime.now(timezone.utc)
    
    await db.flush()
    await db.refresh(submission)
    
    # Log activity
    await log_activity(
        db,
        user_id=user_id,
        action="exercise_submitted",
        target_type="exercise",
        target_id=int(exercise_id) if exercise_id.isdigit() else None,
        classroom_id=assignment.classroom_id if assignment else None,
        details={"assignment_id": assignment_id, "exercise_id": exercise_id}
    )
    
    return submission


async def grade_submission(
    db: AsyncSession,
    assignment_id: int,
    user_id: int,
    score: int,
    feedback: Optional[str] = None
) -> AssignmentSubmission:
    """Grade a submission."""
    submission = await get_or_create_submission(db, assignment_id, user_id)
    
    submission.score = score
    submission.feedback = feedback
    submission.status = "graded"
    submission.graded_at = datetime.now(timezone.utc)
    
    await db.flush()
    await db.refresh(submission)
    
    # Log activity
    assignment = await get_assignment(db, assignment_id)
    if assignment:
        await log_activity(
            db,
            user_id=assignment.created_by,
            action="submission_graded",
            target_type="submission",
            target_id=submission.id,
            classroom_id=assignment.classroom_id,
            details={"student_id": user_id, "score": score}
        )
    
    return submission


async def get_assignment_submissions(
    db: AsyncSession, assignment_id: int
) -> List[Dict[str, Any]]:
    """Get all submissions for an assignment."""
    from app.models import User
    
    result = await db.execute(
        select(AssignmentSubmission, User)
        .join(User, AssignmentSubmission.user_id == User.id)
        .where(AssignmentSubmission.assignment_id == assignment_id)
        .order_by(AssignmentSubmission.submitted_at.desc())
    )
    
    submissions = []
    for submission, user in result.all():
        submissions.append({
            "id": submission.id,
            "user_id": user.id,
            "username": user.username,
            "display_name": user.display_name or user.username,
            "status": submission.status,
            "completed_exercises": submission.completed_exercises,
            "score": submission.score,
            "feedback": submission.feedback,
            "xp_earned": submission.xp_earned,
            "treats_earned": submission.treats_earned,
            "started_at": submission.started_at,
            "submitted_at": submission.submitted_at,
            "graded_at": submission.graded_at
        })
    
    return submissions


# ============================================================================
# LEADERBOARD CRUD
# ============================================================================

async def get_global_leaderboard(
    db: AsyncSession, limit: int = 100
) -> List[Dict[str, Any]]:
    """Get global leaderboard."""
    from app.models import User
    
    result = await db.execute(
        select(UserSchoolProfile, User)
        .join(User, UserSchoolProfile.user_id == User.id)
        .where(User.is_active == True)
        .order_by(UserSchoolProfile.total_xp.desc())
        .limit(limit)
    )
    
    leaderboard = []
    rank = 1
    for profile, user in result.all():
        leaderboard.append({
            "rank": rank,
            "user_id": user.id,
            "username": user.username,
            "display_name": user.display_name or user.username,
            "avatar_url": user.github_avatar_url,
            "total_xp": profile.total_xp,
            "current_rank": profile.current_rank.value,
            "problems_solved": profile.problems_solved,
            "current_streak": profile.current_streak,
            "longest_streak": profile.longest_streak
        })
        rank += 1
    
    return leaderboard


async def get_classroom_leaderboard(
    db: AsyncSession, classroom_id: int, limit: int = 50
) -> List[Dict[str, Any]]:
    """Get leaderboard for a classroom."""
    from app.models import User
    
    result = await db.execute(
        select(ClassroomMember, User, UserSchoolProfile)
        .join(User, ClassroomMember.user_id == User.id)
        .outerjoin(UserSchoolProfile, User.id == UserSchoolProfile.user_id)
        .where(ClassroomMember.classroom_id == classroom_id)
        .where(ClassroomMember.is_active == True)
        .order_by(ClassroomMember.total_xp_earned.desc())
        .limit(limit)
    )
    
    leaderboard = []
    rank = 1
    for member, user, profile in result.all():
        leaderboard.append({
            "rank": rank,
            "user_id": user.id,
            "username": user.username,
            "display_name": user.display_name or user.username,
            "avatar_url": user.github_avatar_url,
            "xp_earned_in_class": member.total_xp_earned,
            "problems_solved_in_class": member.problems_solved_in_class,
            "current_rank": profile.current_rank.value if profile else "novice",
            "total_xp": profile.total_xp if profile else 0
        })
        rank += 1
    
    return leaderboard


# ============================================================================
# INITIALIZATION
# ============================================================================

async def initialize_ranks(db: AsyncSession):
    """Initialize default rank definitions."""
    ranks = [
        {
            "rank_tier": RankTier.NOVICE,
            "name": "Novice",
            "description": "Just starting your Rust journey",
            "icon": "🌱",
            "min_xp": 0,
            "color_hex": "#6b7280",
            "treats_reward": 0
        },
        {
            "rank_tier": RankTier.APPRENTICE,
            "name": "Apprentice",
            "description": "Learning the basics of Rust",
            "icon": "📚",
            "min_xp": 1000,
            "color_hex": "#22c55e",
            "treats_reward": 10
        },
        {
            "rank_tier": RankTier.JOURNEYMAN,
            "name": "Journeyman",
            "description": "Building your Rust skills",
            "icon": "🔨",
            "min_xp": 3000,
            "color_hex": "#3b82f6",
            "treats_reward": 20
        },
        {
            "rank_tier": RankTier.ADEPT,
            "name": "Adept",
            "description": "Becoming proficient in Rust",
            "icon": "⚡",
            "min_xp": 6000,
            "color_hex": "#8b5cf6",
            "treats_reward": 35
        },
        {
            "rank_tier": RankTier.EXPERT,
            "name": "Expert",
            "description": "Mastering advanced Rust concepts",
            "icon": "🎯",
            "min_xp": 10000,
            "color_hex": "#f59e0b",
            "treats_reward": 50
        },
        {
            "rank_tier": RankTier.MASTER,
            "name": "Master",
            "description": "A true Rust master",
            "icon": "👑",
            "min_xp": 15000,
            "color_hex": "#ef4444",
            "treats_reward": 75
        },
        {
            "rank_tier": RankTier.GRANDMASTER,
            "name": "Grandmaster",
            "description": "Elite Rust programmer",
            "icon": "🌟",
            "min_xp": 21000,
            "color_hex": "#ec4899",
            "treats_reward": 100
        },
        {
            "rank_tier": RankTier.LEGEND,
            "name": "Legend",
            "description": "A legend in the Rust community",
            "icon": "🔥",
            "min_xp": 30000,
            "color_hex": "#f97316",
            "treats_reward": 150
        }
    ]
    
    for rank_data in ranks:
        existing = await db.execute(
            select(RankDefinition).where(
                RankDefinition.rank_tier == rank_data["rank_tier"]
            )
        )
        if not existing.scalar_one_or_none():
            rank = RankDefinition(**rank_data)
            db.add(rank)
    
    # Transaction committed by caller


async def initialize_titles(db: AsyncSession):
    """Initialize default titles."""
    titles = [
        # Problem-solving titles
        {
            "key": "first_steps",
            "name": "First Steps",
            "description": "Solve your first problem",
            "category": "achievement",
            "requirement_type": "problems_solved",
            "requirement_value": 1,
            "requirement_data": {},
            "icon": "👣",
            "color": "#22c55e",
            "is_hidden": False
        },
        {
            "key": "getting_warm",
            "name": "Getting Warm",
            "description": "Solve 10 problems",
            "category": "achievement",
            "requirement_type": "problems_solved",
            "requirement_value": 10,
            "requirement_data": {},
            "icon": "🔥",
            "color": "#f59e0b",
            "is_hidden": False
        },
        {
            "key": "problem_solver",
            "name": "Problem Solver",
            "description": "Solve 50 problems",
            "category": "achievement",
            "requirement_type": "problems_solved",
            "requirement_value": 50,
            "requirement_data": {},
            "icon": "🧩",
            "color": "#3b82f6",
            "is_hidden": False
        },
        {
            "key": "code_master",
            "name": "Code Master",
            "description": "Solve 100 problems",
            "category": "achievement",
            "requirement_type": "problems_solved",
            "requirement_value": 100,
            "requirement_data": {},
            "icon": "💻",
            "color": "#8b5cf6",
            "is_hidden": False
        },
        {
            "key": "rust_legend",
            "name": "Rust Legend",
            "description": "Solve 250 problems",
            "category": "achievement",
            "requirement_type": "problems_solved",
            "requirement_value": 250,
            "requirement_data": {},
            "icon": "🏆",
            "color": "#f97316",
            "is_hidden": False
        },
        # Streak titles
        {
            "key": "week_warrior",
            "name": "Week Warrior",
            "description": "Maintain a 7-day streak",
            "category": "achievement",
            "requirement_type": "streak",
            "requirement_value": 7,
            "requirement_data": {},
            "icon": "📅",
            "color": "#22c55e",
            "is_hidden": False
        },
        {
            "key": "month_master",
            "name": "Month Master",
            "description": "Maintain a 30-day streak",
            "category": "achievement",
            "requirement_type": "streak",
            "requirement_value": 30,
            "requirement_data": {},
            "icon": "🗓️",
            "color": "#3b82f6",
            "is_hidden": False
        },
        {
            "key": "centurion",
            "name": "Centurion",
            "description": "Maintain a 100-day streak",
            "category": "achievement",
            "requirement_type": "streak",
            "requirement_value": 100,
            "requirement_data": {},
            "icon": "💯",
            "color": "#ef4444",
            "is_hidden": False
        },
        # Rank titles
        {
            "key": "apprentice_rank",
            "name": "The Apprentice",
            "description": "Reach Apprentice rank",
            "category": "rank",
            "requirement_type": "rank",
            "requirement_value": 1,
            "requirement_data": {"tier": "apprentice"},
            "icon": "📚",
            "color": "#22c55e",
            "is_hidden": False
        },
        {
            "key": "expert_rank",
            "name": "The Expert",
            "description": "Reach Expert rank",
            "category": "rank",
            "requirement_type": "rank",
            "requirement_value": 1,
            "requirement_data": {"tier": "expert"},
            "icon": "🎯",
            "color": "#f59e0b",
            "is_hidden": False
        },
        {
            "key": "legend_rank",
            "name": "The Legend",
            "description": "Reach Legend rank",
            "category": "rank",
            "requirement_type": "rank",
            "requirement_value": 1,
            "requirement_data": {"tier": "legend"},
            "icon": "🔥",
            "color": "#f97316",
            "is_hidden": False
        }
    ]
    
    for title_data in titles:
        existing = await db.execute(
            select(TitleDefinition).where(TitleDefinition.key == title_data["key"])
        )
        if not existing.scalar_one_or_none():
            title = TitleDefinition(**title_data)
            db.add(title)
    
    # Transaction committed by caller
