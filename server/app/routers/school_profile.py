# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
API routes for user school profiles and gamification.
"""
from typing import List, Optional

from fastapi import APIRouter, Depends, HTTPException, status
from sqlalchemy.ext.asyncio import AsyncSession

from app.database import get_db
from app.auth import get_current_user
from app.models import User, UserProgress
from app import crud_school as crud
from app import schemas_school as schemas
from app.models_school import UserRole, GradeLevel, RankTier
from app.models_school import RankDefinition, TitleDefinition, TreatTransaction, GradeContentMapping
from sqlalchemy import select

router = APIRouter(prefix="/api/school", tags=["School Platform"])


# ============================================================================
# USER SCHOOL PROFILE
# ============================================================================

@router.get("/profile", response_model=schemas.UserSchoolProfileResponse)
async def get_my_school_profile(
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get current user's school profile."""
    profile = await crud.get_user_school_profile(db, current_user.id)
    
    # Calculate rank progress
    rank_progress = schemas.UserSchoolProfileResponse.calculate_rank_progress(
        profile.total_xp, profile.current_rank
    )
    
    # Create response
    response_data = {
        **{k: v for k, v in vars(profile).items() if not k.startswith('_')},
        "rank_progress": rank_progress
    }
    
    return schemas.UserSchoolProfileResponse(**response_data)


@router.post("/profile", response_model=schemas.UserSchoolProfileResponse)
async def create_school_profile(
    data: schemas.UserSchoolProfileCreate,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Create school profile for current user."""
    existing = await crud.get_user_school_profile(db, current_user.id)
    if existing and existing.id:  # Profile already exists
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="School profile already exists"
        )
    
    profile = await crud.create_user_school_profile(db, current_user.id, data)
    
    rank_progress = schemas.UserSchoolProfileResponse.calculate_rank_progress(
        profile.total_xp, profile.current_rank
    )
    
    response_data = {
        **{k: v for k, v in vars(profile).items() if not k.startswith('_')},
        "rank_progress": rank_progress
    }
    
    return schemas.UserSchoolProfileResponse(**response_data)


@router.patch("/profile", response_model=schemas.UserSchoolProfileResponse)
async def update_school_profile(
    data: schemas.UserSchoolProfileUpdate,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Update current user's school profile."""
    profile = await crud.update_user_school_profile(db, current_user.id, data)
    if not profile:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="School profile not found"
        )
    
    rank_progress = schemas.UserSchoolProfileResponse.calculate_rank_progress(
        profile.total_xp, profile.current_rank
    )
    
    response_data = {
        **{k: v for k, v in vars(profile).items() if not k.startswith('_')},
        "rank_progress": rank_progress
    }
    
    return schemas.UserSchoolProfileResponse(**response_data)


@router.get("/profile/{user_id}", response_model=schemas.UserPublicProfile)
async def get_public_profile(
    user_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get public profile of another user."""
    from app.models import User as UserModel
    
    result = await db.execute(
        select(UserModel).where(UserModel.id == user_id)
    )
    user = result.scalar_one_or_none()
    
    if not user:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="User not found"
        )
    
    profile = await crud.get_user_school_profile(db, user_id)
    
    return schemas.UserPublicProfile(
        id=user.id,
        username=user.username,
        display_name=user.display_name,
        role=profile.role if profile else UserRole.STUDENT,
        grade_level=profile.grade_level if profile else None,
        current_rank=profile.current_rank if profile else RankTier.NOVICE,
        current_title=profile.display_title if profile else None,
        total_xp=profile.total_xp if profile else 0,
        problems_solved_total=profile.problems_solved if profile else 0,
        streak_days=profile.current_streak if profile else 0,
        badges=getattr(profile, "badges", []) if profile else []
    )


# ============================================================================
# GRADE LEVELS
# ============================================================================

@router.get("/grade-levels", response_model=List[dict])
async def get_grade_levels():
    """Get all available grade levels."""
    grade_info = []
    
    grade_mapping = {
        GradeLevel.PRIMARY_1: {"name": "Primary 1", "age": "6-7", "category": "primary"},
        GradeLevel.PRIMARY_2: {"name": "Primary 2", "age": "7-8", "category": "primary"},
        GradeLevel.PRIMARY_3: {"name": "Primary 3", "age": "8-9", "category": "primary"},
        GradeLevel.PRIMARY_4: {"name": "Primary 4", "age": "9-10", "category": "primary"},
        GradeLevel.PRIMARY_5: {"name": "Primary 5", "age": "10-11", "category": "primary"},
        GradeLevel.MIDDLE_6: {"name": "Middle 6", "age": "11-12", "category": "middle"},
        GradeLevel.MIDDLE_7: {"name": "Middle 7", "age": "12-13", "category": "middle"},
        GradeLevel.MIDDLE_8: {"name": "Middle 8", "age": "13-14", "category": "middle"},
        GradeLevel.HIGH_9: {"name": "High 9", "age": "14-15", "category": "high"},
        GradeLevel.HIGH_10: {"name": "High 10", "age": "15-16", "category": "high"},
        GradeLevel.HIGH_11: {"name": "High 11", "age": "16-17", "category": "high"},
        GradeLevel.HIGH_12: {"name": "High 12", "age": "17-18", "category": "high"},
        GradeLevel.COLLEGE: {"name": "College", "age": "18+", "category": "college"},
    }
    
    for level, info in grade_mapping.items():
        grade_info.append({
            "value": level.value,
            "name": info["name"],
            "age_range": info["age"],
            "category": info["category"]
        })
    
    return grade_info


# ============================================================================
# RANKS & PROGRESS
# ============================================================================

@router.get("/ranks", response_model=List[schemas.RankDefinitionResponse])
async def get_rank_definitions(
    db: AsyncSession = Depends(get_db)
):
    """Get all rank definitions."""
    ranks = await crud.get_rank_definitions(db)
    return ranks


@router.get("/ranks/progress", response_model=schemas.RankProgressResponse)
async def get_my_rank_progress(
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get current user's rank progress."""
    profile = await crud.get_user_school_profile(db, current_user.id)
    
    # Get next rank
    rank_order = [
        RankTier.NOVICE, RankTier.APPRENTICE, RankTier.JOURNEYMAN,
        RankTier.ADEPT, RankTier.EXPERT, RankTier.MASTER,
        RankTier.GRANDMASTER, RankTier.LEGEND
    ]
    
    current_idx = rank_order.index(profile.current_rank)
    next_rank = rank_order[current_idx + 1] if current_idx < len(rank_order) - 1 else None
    
    rank_progress = schemas.UserSchoolProfileResponse.calculate_rank_progress(
        profile.total_xp, profile.current_rank
    )
    
    # Get next rank name
    next_rank_name = None
    if next_rank:
        result = await db.execute(
            select(RankDefinition.name).where(RankDefinition.rank_tier == next_rank)
        )
        next_rank_name = result.scalar()
    
    return schemas.RankProgressResponse(
        **rank_progress,
        next_rank=next_rank,
        next_rank_name=next_rank_name
    )


# ============================================================================
# TITLES
# ============================================================================

@router.get("/titles", response_model=List[schemas.TitleDefinitionResponse])
async def get_all_titles(
    db: AsyncSession = Depends(get_db)
):
    """Get all available title definitions."""
    result = await db.execute(
        select(TitleDefinition).where(TitleDefinition.is_active == True)
    )
    titles = result.scalars().all()
    return titles


@router.get("/titles/my", response_model=List[schemas.UserTitleResponse])
async def get_my_titles(
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get titles earned by current user."""
    titles = await crud.get_user_titles(db, current_user.id)
    return titles


@router.post("/titles/equip")
async def equip_title(
    request: schemas.EquipTitleRequest,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Equip or unequip a title."""
    success = await crud.equip_title(db, current_user.id, request.title_key)
    if not success:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Title not owned by user"
        )
    return {"message": "Title equipped successfully"}


# ============================================================================
# TREATS
# ============================================================================

@router.get("/treats/balance", response_model=schemas.TreatBalanceResponse)
async def get_treat_balance(
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get current user's treat balance."""
    profile = await crud.get_user_school_profile(db, current_user.id)
    
    # Calculate total earned/spent
    result = await db.execute(
        select(TreatTransaction).where(
            TreatTransaction.user_id == current_user.id
        )
    )
    transactions = result.scalars().all()
    
    total_earned = sum(t.amount for t in transactions if t.amount > 0)
    total_spent = abs(sum(t.amount for t in transactions if t.amount < 0))
    
    return schemas.TreatBalanceResponse(
        balance=profile.treats,
        total_earned=total_earned,
        total_spent=total_spent
    )


@router.get("/treats/history", response_model=List[schemas.TreatTransactionResponse])
async def get_treat_history(
    limit: int = 20,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get treat transaction history."""
    result = await db.execute(
        select(TreatTransaction)
        .where(TreatTransaction.user_id == current_user.id)
        .order_by(TreatTransaction.created_at.desc())
        .limit(limit)
    )
    transactions = result.scalars().all()
    return transactions


# ============================================================================
# LEADERBOARDS
# ============================================================================

@router.get("/leaderboard/global", response_model=schemas.LeaderboardResponse)
async def get_global_leaderboard(
    period: str = "all_time",
    limit: int = 50,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get global leaderboard."""
    from app.models import User
    from app.models_school import UserSchoolProfile
    
    # Get top users by XP
    result = await db.execute(
        select(User, UserSchoolProfile)
        .join(UserSchoolProfile, User.id == UserSchoolProfile.user_id)
        .order_by(UserSchoolProfile.total_xp.desc())
        .limit(limit)
    )
    
    entries = []
    current_user_rank = None
    
    for rank, (user, profile) in enumerate(result.all(), 1):
        entry = schemas.LeaderboardEntryResponse(
            rank=rank,
            user_id=user.id,
            username=user.username,
            display_name=user.display_name,
            current_rank=profile.current_rank,
            current_title=profile.display_title,
            xp_total=profile.total_xp,
            problems_solved=profile.problems_solved,
            is_current_user=(user.id == current_user.id)
        )
        entries.append(entry)
        
        if user.id == current_user.id:
            current_user_rank = rank
    
    return schemas.LeaderboardResponse(
        scope_type="global",
        scope_id=None,
        scope_name="Global",
        period=period,
        entries=entries,
        total_count=len(entries),
        current_user_rank=current_user_rank
    )


@router.get("/leaderboard/classroom/{classroom_id}", response_model=schemas.LeaderboardResponse)
async def get_classroom_leaderboard(
    classroom_id: int,
    limit: int = 50,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get leaderboard for a specific classroom."""
    # Check if user is member
    is_member = await crud.is_classroom_member(db, current_user.id, classroom_id)
    if not is_member:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Not a member of this classroom"
        )
    
    classroom = await crud.get_classroom(db, classroom_id)
    members = await crud.get_classroom_members(db, classroom_id)
    
    # Sort by XP earned in classroom
    members.sort(key=lambda m: m["xp_earned"], reverse=True)
    
    entries = []
    current_user_rank = None
    
    for rank, member in enumerate(members[:limit], 1):
        entry = schemas.LeaderboardEntryResponse(
            rank=rank,
            user_id=member["user_id"],
            username=member["username"],
            display_name=member["display_name"],
            current_rank=member["current_rank"],
            current_title=None,
            xp_total=member["xp_earned"],
            problems_solved=member["problems_solved_count"],
            is_current_user=(member["user_id"] == current_user.id)
        )
        entries.append(entry)
        
        if member["user_id"] == current_user.id:
            current_user_rank = rank
    
    return schemas.LeaderboardResponse(
        scope_type="classroom",
        scope_id=classroom_id,
        scope_name=classroom.name,
        period="all_time",
        entries=entries,
        total_count=len(members),
        current_user_rank=current_user_rank
    )


# ============================================================================
# PROGRESS TRACKING
# ============================================================================

@router.post("/progress/problem-solved/{exercise_id}")
async def record_problem_solved(
    exercise_id: str,
    difficulty: int = 1,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Record a solved problem and award XP."""
    profile = await crud.record_problem_solved(db, current_user.id, exercise_id, difficulty)
    
    # Check for new titles
    new_titles = await crud.check_and_award_titles(db, current_user.id)
    
    return {
        "message": "Problem recorded",
        "xp_gained": {1: 10, 2: 20, 3: 35, 4: 50, 5: 75}.get(difficulty, 10),
        "total_xp": profile.total_xp,
        "current_rank": profile.current_rank.value,
        "new_titles": new_titles
    }


@router.get("/progress/recommended", response_model=schemas.RecommendedExercisesResponse)
async def get_recommended_exercises(
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user)
):
    """Get recommended exercises based on grade level."""
    profile = await crud.get_user_school_profile(db, current_user.id)
    
    if not profile.grade_level:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Please set your grade level first"
        )
    
    # Get recommended exercises for grade
    result = await db.execute(
        select(GradeContentMapping)
        .where(
            GradeContentMapping.grade_level == profile.grade_level,
            GradeContentMapping.is_recommended == True
        )
    )
    mappings = result.scalars().all()
    
    # Format exercise data
    exercises = []
    for mapping in mappings:
        exercises.append({
            "exercise_id": mapping.exercise_id,
            "difficulty_modifier": mapping.difficulty_modifier,
            "required_prior": mapping.required_prior_exercises
        })
    
    # Get completion stats from user's progress
    result = await db.execute(
        select(UserProgress).where(UserProgress.user_id == current_user.id)
    )
    progress = result.scalar_one_or_none()
    solved = set(progress.solved_problems) if progress else set()
    
    completed = len([e for e in exercises if e["exercise_id"] in solved])
    
    return schemas.RecommendedExercisesResponse(
        grade_level=profile.grade_level,
        exercises=exercises,
        total_recommended=len(exercises),
        completed=completed,
        percentage=(completed / len(exercises) * 100) if exercises else 0
    )
