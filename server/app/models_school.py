# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
School platform models for gamification, classrooms, and progress tracking.
"""

from datetime import datetime, timezone
from enum import Enum as PyEnum
from typing import Optional, List

from sqlalchemy import (
    Column,
    Integer,
    String,
    DateTime,
    Boolean,
    ForeignKey,
    Text,
    Enum,
    Index,
    JSON,
    func,
)
from sqlalchemy.orm import relationship, validates, foreign

from app.database import Base


# ============================================================================
# ENUMS
# ============================================================================


class UserRole(PyEnum):
    """User roles in the school platform."""

    STUDENT = "student"
    TEACHER = "teacher"
    ADMIN = "admin"


class GradeLevel(PyEnum):
    """Grade levels supported by the platform."""

    PRIMARY_1 = "primary_1"  # Ages 6-7
    PRIMARY_2 = "primary_2"  # Ages 7-8
    PRIMARY_3 = "primary_3"  # Ages 8-9
    PRIMARY_4 = "primary_4"  # Ages 9-10
    PRIMARY_5 = "primary_5"  # Ages 10-11
    MIDDLE_6 = "middle_6"  # Ages 11-12
    MIDDLE_7 = "middle_7"  # Ages 12-13
    MIDDLE_8 = "middle_8"  # Ages 13-14
    HIGH_9 = "high_9"  # Ages 14-15
    HIGH_10 = "high_10"  # Ages 15-16
    HIGH_11 = "high_11"  # Ages 16-17
    HIGH_12 = "high_12"  # Ages 17-18
    COLLEGE = "college"  # Adult learners


class RankTier(PyEnum):
    """Rank tiers for gamification."""

    NOVICE = "novice"  # 0 XP
    APPRENTICE = "apprentice"  # 1,000 XP
    JOURNEYMAN = "journeyman"  # 3,000 XP
    ADEPT = "adept"  # 6,000 XP
    EXPERT = "expert"  # 10,000 XP
    MASTER = "master"  # 15,000 XP
    GRANDMASTER = "grandmaster"  # 21,000 XP
    LEGEND = "legend"  # 30,000 XP


class AssignmentStatus(PyEnum):
    """Status of an assignment."""

    DRAFT = "draft"
    PUBLISHED = "published"
    CLOSED = "closed"


class SubmissionStatus(PyEnum):
    """Status of an assignment submission."""

    NOT_STARTED = "not_started"
    IN_PROGRESS = "in_progress"
    SUBMITTED = "submitted"
    GRADED = "graded"
    LATE = "late"


# ============================================================================
# USER PROFILE
# ============================================================================


class UserSchoolProfile(Base):
    """Extended profile for school platform users."""

    __tablename__ = "user_school_profiles"

    id = Column(Integer, primary_key=True, index=True)
    user_id = Column(
        Integer, ForeignKey("users.id"), unique=True, nullable=False, index=True
    )

    # Role and grade
    role = Column(Enum(UserRole), default=UserRole.STUDENT, nullable=False)
    grade_level = Column(Enum(GradeLevel), nullable=True)

    # Gamification
    total_xp = Column(Integer, default=0, nullable=False)
    current_rank = Column(Enum(RankTier), default=RankTier.NOVICE, nullable=False)
    treats = Column(Integer, default=0, nullable=False)  # Special currency

    # Streaks
    current_streak = Column(Integer, default=0, nullable=False)
    longest_streak = Column(Integer, default=0, nullable=False)
    last_activity_date = Column(DateTime(timezone=True), nullable=True)

    # Stats
    problems_solved = Column(Integer, default=0, nullable=False)
    problems_attempted = Column(Integer, default=0, nullable=False)
    total_time_spent_minutes = Column(Integer, default=0, nullable=False)

    # Settings
    display_title = Column(String(50), nullable=True)  # Currently equipped title
    profile_public = Column(Boolean, default=True, nullable=False)

    # Timestamps
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    updated_at = Column(
        DateTime(timezone=True),
        server_default=func.now(),
        onupdate=func.now(),
        nullable=False,
    )

    # Relationships
    user = relationship("User", back_populates="school_profile")

    def __repr__(self) -> str:
        return f"<UserSchoolProfile(user_id={self.user_id}, role={self.role}, rank={self.current_rank})>"

    @validates("total_xp")
    def validate_xp(self, key, value):
        return max(0, value)

    @validates("treats")
    def validate_treats(self, key, value):
        return max(0, value)

    def calculate_rank(self) -> RankTier:
        """Calculate rank based on total XP."""
        xp = self.total_xp
        if xp >= 30000:
            return RankTier.LEGEND
        elif xp >= 21000:
            return RankTier.GRANDMASTER
        elif xp >= 15000:
            return RankTier.MASTER
        elif xp >= 10000:
            return RankTier.EXPERT
        elif xp >= 6000:
            return RankTier.ADEPT
        elif xp >= 3000:
            return RankTier.JOURNEYMAN
        elif xp >= 1000:
            return RankTier.APPRENTICE
        return RankTier.NOVICE

    def update_rank(self) -> bool:
        """Update rank based on XP. Returns True if rank changed."""
        new_rank = self.calculate_rank()
        if new_rank != self.current_rank:
            self.current_rank = new_rank
            return True
        return False

    @property
    def xp_to_next_rank(self) -> int:
        """Get XP needed for next rank."""
        rank_thresholds = {
            RankTier.NOVICE: 1000,
            RankTier.APPRENTICE: 3000,
            RankTier.JOURNEYMAN: 6000,
            RankTier.ADEPT: 10000,
            RankTier.EXPERT: 15000,
            RankTier.MASTER: 21000,
            RankTier.GRANDMASTER: 30000,
            RankTier.LEGEND: 30000,  # Max
        }
        return max(0, rank_thresholds.get(self.current_rank, 1000) - self.total_xp)


# ============================================================================
# CLASSROOM
# ============================================================================


class Classroom(Base):
    """A classroom/group of students led by a teacher."""

    __tablename__ = "classrooms"

    id = Column(Integer, primary_key=True, index=True)
    teacher_id = Column(Integer, ForeignKey("users.id"), nullable=False, index=True)

    # Basic info
    name = Column(String(100), nullable=False)
    description = Column(Text, nullable=True)
    subject = Column(String(50), nullable=True)  # e.g., "Computer Science"

    # Join code for students
    code = Column(String(10), unique=True, nullable=False, index=True)

    # Settings
    grade_level = Column(Enum(GradeLevel), nullable=True)
    max_students = Column(Integer, default=30, nullable=False)
    is_active = Column(Boolean, default=True, nullable=False)
    is_archived = Column(Boolean, default=False, nullable=False)

    # Timestamps
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    updated_at = Column(
        DateTime(timezone=True),
        server_default=func.now(),
        onupdate=func.now(),
        nullable=False,
    )
    archived_at = Column(DateTime(timezone=True), nullable=True)

    # Relationships
    teacher = relationship("User", foreign_keys="Classroom.teacher_id")
    members = relationship(
        "ClassroomMember",
        back_populates="classroom",
        lazy="dynamic",
        cascade="all, delete-orphan",
    )
    assignments = relationship(
        "Assignment",
        back_populates="classroom",
        lazy="dynamic",
        cascade="all, delete-orphan",
    )

    def __repr__(self) -> str:
        return f"<Classroom(id={self.id}, name={self.name}, code={self.code})>"

    @property
    def member_count(self) -> int:
        """Get current number of members."""
        return self.members.count()

    @property
    def is_full(self) -> bool:
        """Check if classroom is at capacity."""
        return self.member_count >= self.max_students


class ClassroomMember(Base):
    """Student membership in a classroom."""

    __tablename__ = "classroom_members"

    id = Column(Integer, primary_key=True, index=True)
    classroom_id = Column(
        Integer, ForeignKey("classrooms.id"), nullable=False, index=True
    )
    user_id = Column(Integer, ForeignKey("users.id"), nullable=False, index=True)

    # Status
    joined_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    is_active = Column(Boolean, default=True, nullable=False)

    # Progress in this classroom
    problems_solved_in_class = Column(Integer, default=0, nullable=False)
    total_xp_earned = Column(Integer, default=0, nullable=False)

    # Indexes
    __table_args__ = (
        Index("ix_classroom_member_unique", "classroom_id", "user_id", unique=True),
    )

    # Relationships
    classroom = relationship("Classroom", back_populates="members")
    student_profile = relationship(
        "UserSchoolProfile",
        foreign_keys=[user_id],
        primaryjoin="UserSchoolProfile.user_id == ClassroomMember.user_id",
        viewonly=True,
    )


# ============================================================================
# ASSIGNMENTS
# ============================================================================


class Assignment(Base):
    """An assignment given to a classroom."""

    __tablename__ = "assignments"

    id = Column(Integer, primary_key=True, index=True)
    classroom_id = Column(
        Integer, ForeignKey("classrooms.id"), nullable=False, index=True
    )
    created_by = Column(Integer, ForeignKey("users.id"), nullable=False)

    # Content
    title = Column(String(100), nullable=False)
    description = Column(Text, nullable=True)
    exercise_ids = Column(JSON, default=list, nullable=False)  # List of exercise IDs

    # Schedule
    status = Column(
        Enum(AssignmentStatus), default=AssignmentStatus.DRAFT, nullable=False
    )
    due_date = Column(DateTime(timezone=True), nullable=True)

    # Gamification
    xp_reward = Column(Integer, default=0, nullable=False)  # Bonus XP for completion
    treat_reward = Column(Integer, default=0, nullable=False)  # Bonus treats

    # Timestamps
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    updated_at = Column(
        DateTime(timezone=True),
        server_default=func.now(),
        onupdate=func.now(),
        nullable=False,
    )
    published_at = Column(DateTime(timezone=True), nullable=True)

    # Relationships
    classroom = relationship("Classroom", back_populates="assignments")
    submissions = relationship(
        "AssignmentSubmission",
        back_populates="assignment",
        lazy="dynamic",
        cascade="all, delete-orphan",
    )

    def __repr__(self) -> str:
        return f"<Assignment(id={self.id}, title={self.title}, classroom_id={self.classroom_id})>"


class AssignmentSubmission(Base):
    """Student submission for an assignment."""

    __tablename__ = "assignment_submissions"

    id = Column(Integer, primary_key=True, index=True)
    assignment_id = Column(
        Integer, ForeignKey("assignments.id"), nullable=False, index=True
    )
    user_id = Column(Integer, ForeignKey("users.id"), nullable=False, index=True)

    # Progress tracking
    status = Column(
        Enum(SubmissionStatus), default=SubmissionStatus.NOT_STARTED, nullable=False
    )
    completed_exercises = Column(
        JSON, default=list, nullable=False
    )  # List of completed exercise IDs

    # Results
    submitted_at = Column(DateTime(timezone=True), nullable=True)
    graded_at = Column(DateTime(timezone=True), nullable=True)
    score = Column(Integer, nullable=True)  # Percentage or points
    feedback = Column(Text, nullable=True)

    # XP earned from this submission
    xp_earned = Column(Integer, default=0, nullable=False)
    treats_earned = Column(Integer, default=0, nullable=False)

    # Timestamps
    started_at = Column(DateTime(timezone=True), nullable=True)
    updated_at = Column(
        DateTime(timezone=True),
        server_default=func.now(),
        onupdate=func.now(),
        nullable=False,
    )

    # Indexes
    __table_args__ = (
        Index("ix_submission_unique", "assignment_id", "user_id", unique=True),
    )

    # Relationships
    assignment = relationship("Assignment", back_populates="submissions")
    student_profile = relationship(
        "UserSchoolProfile",
        foreign_keys=[user_id],
        primaryjoin="UserSchoolProfile.user_id == AssignmentSubmission.user_id",
        viewonly=True,
    )


# ============================================================================
# GAMIFICATION - RANKS & TITLES
# ============================================================================


class RankDefinition(Base):
    """Definition of ranks and their requirements."""

    __tablename__ = "rank_definitions"

    id = Column(Integer, primary_key=True, index=True)

    rank_tier = Column(Enum(RankTier), unique=True, nullable=False)
    name = Column(String(50), nullable=False)
    description = Column(String(255), nullable=True)
    icon = Column(String(10), nullable=True)  # Emoji or icon code

    # Requirements
    min_xp = Column(Integer, nullable=False)
    color_hex = Column(String(7), default="#000000", nullable=False)  # For UI theming

    # Rewards for achieving this rank
    treats_reward = Column(Integer, default=0, nullable=False)
    title_unlocked = Column(String(50), nullable=True)  # Optional title granted


class TitleDefinition(Base):
    """Achievable titles that users can display."""

    __tablename__ = "title_definitions"

    id = Column(Integer, primary_key=True, index=True)

    key = Column(String(50), unique=True, nullable=False, index=True)
    name = Column(String(100), nullable=False)
    description = Column(String(255), nullable=True)

    # Category
    category = Column(
        String(50), nullable=False
    )  # achievement, rank, special, classroom

    # Requirements (stored as JSON for flexibility)
    requirement_type = Column(
        String(50), nullable=False
    )  # problems_solved, streak_days, rank_reached, etc.
    requirement_value = Column(Integer, nullable=False)  # threshold value
    requirement_data = Column(
        JSON, default=dict, nullable=False
    )  # Additional requirements

    # Rewards
    treats_bonus = Column(Integer, default=0, nullable=False)
    xp_bonus = Column(Integer, default=0, nullable=False)

    # Visual
    icon = Column(String(10), nullable=True)
    color = Column(String(7), default="#f97316", nullable=False)

    is_hidden = Column(Boolean, default=False, nullable=False)  # Secret achievement?


class UserTitle(Base):
    """Titles earned by users."""

    __tablename__ = "user_titles"

    id = Column(Integer, primary_key=True, index=True)
    user_id = Column(Integer, ForeignKey("users.id"), nullable=False, index=True)
    title_key = Column(
        String(50), ForeignKey("title_definitions.key"), nullable=False, index=True
    )

    earned_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    is_equipped = Column(Boolean, default=False, nullable=False)  # Currently displayed

    # Indexes
    __table_args__ = (
        Index("ix_user_titles_unique", "user_id", "title_key", unique=True),
    )

    # Relationships
    student_profile = relationship(
        "UserSchoolProfile",
        foreign_keys=[user_id],
        primaryjoin="UserSchoolProfile.user_id == UserTitle.user_id",
        viewonly=True,
    )
    title_def = relationship("TitleDefinition", foreign_keys=[title_key])


# ============================================================================
# GAMIFICATION - TREATS & REWARDS
# ============================================================================


class TreatTransaction(Base):
    """Transaction log for treats (earned/spent)."""

    __tablename__ = "treat_transactions"

    id = Column(Integer, primary_key=True, index=True)
    user_id = Column(Integer, ForeignKey("users.id"), nullable=False, index=True)

    # Transaction details
    amount = Column(Integer, nullable=False)  # Positive = earned, Negative = spent
    balance_after = Column(Integer, nullable=False)

    # Source/Reason
    transaction_type = Column(
        String(50), nullable=False
    )  # rank_up, assignment_complete, streak_bonus, purchase, etc.
    description = Column(String(255), nullable=True)
    reference_id = Column(
        Integer, nullable=True
    )  # Optional reference to related record

    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )


class TreatReward(Base):
    """Definition of how treats can be earned."""

    __tablename__ = "treat_rewards"

    id = Column(Integer, primary_key=True, index=True)

    action_type = Column(
        String(50), unique=True, nullable=False
    )  # problem_solved, streak_maintained, rank_up, etc.
    treats_amount = Column(Integer, nullable=False)
    description = Column(String(255), nullable=True)

    # Cooldown (in seconds) - 0 for no cooldown
    cooldown_seconds = Column(Integer, default=0, nullable=False)

    is_active = Column(Boolean, default=True, nullable=False)


# ============================================================================
# LEADERBOARD & COMPETITION
# ============================================================================


class LeaderboardEntry(Base):
    """Cached leaderboard entries for performance."""

    __tablename__ = "leaderboard_entries"

    id = Column(Integer, primary_key=True, index=True)

    # Scope
    scope_type = Column(
        String(50), nullable=False
    )  # global, classroom, grade_level, school
    scope_id = Column(Integer, nullable=True)  # classroom_id, grade_level_id, etc.

    user_id = Column(Integer, ForeignKey("users.id"), nullable=False, index=True)

    # Ranking data
    rank = Column(Integer, nullable=False, index=True)
    xp_total = Column(Integer, nullable=False)
    problems_solved = Column(Integer, default=0, nullable=False)

    # Period
    period = Column(
        String(20), default="all_time", nullable=False
    )  # daily, weekly, monthly, all_time

    # Timestamp
    calculated_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )

    # Indexes
    __table_args__ = (
        Index(
            "ix_leaderboard_scope_period", "scope_type", "scope_id", "period", "rank"
        ),
    )


# ============================================================================
# GRADE LEVEL CONTENT MAPPING
# ============================================================================


class GradeContentMapping(Base):
    """Maps exercises to appropriate grade levels."""

    __tablename__ = "grade_content_mappings"

    id = Column(Integer, primary_key=True, index=True)

    grade_level = Column(Enum(GradeLevel), nullable=False, index=True)
    exercise_id = Column(String(20), nullable=False, index=True)  # e.g., "001", "150"

    # Is this exercise recommended for this grade?
    is_recommended = Column(Boolean, default=True, nullable=False)

    # Difficulty adjustment (if any)
    difficulty_modifier = Column(Integer, default=0, nullable=False)  # -2 to +2

    # Prerequisites for this grade
    required_prior_exercises = Column(JSON, default=list, nullable=False)

    # Indexes
    __table_args__ = (
        Index("ix_grade_content_unique", "grade_level", "exercise_id", unique=True),
    )


# ============================================================================
# PASSWORD RESET
# ============================================================================


class PasswordResetToken(Base):
    """Password reset token storage."""

    __tablename__ = "password_reset_tokens"

    id = Column(Integer, primary_key=True, index=True)
    user_id = Column(Integer, ForeignKey("users.id"), nullable=False, index=True)

    # Token (hashed in production)
    token_hash = Column(String(255), unique=True, nullable=False, index=True)

    # Expiration
    expires_at = Column(DateTime(timezone=True), nullable=False)
    used_at = Column(DateTime(timezone=True), nullable=True)

    # Timestamps
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )

    # Indexes
    __table_args__ = (Index("ix_reset_token_user", "user_id", "expires_at"),)


# ============================================================================
# NOTIFICATIONS
# ============================================================================


class NotificationType:
    """Notification type constants."""

    ASSIGNMENT_DUE = "assignment_due"
    ASSIGNMENT_GRADED = "assignment_graded"
    CLASSROOM_INVITE = "classroom_invite"
    RANK_UP = "rank_up"
    TITLE_EARNED = "title_earned"
    STREAK_WARNING = "streak_warning"
    ACHIEVEMENT = "achievement"
    SYSTEM = "system"


class Notification(Base):
    """User notifications."""

    __tablename__ = "notifications"

    id = Column(Integer, primary_key=True, index=True)
    user_id = Column(Integer, ForeignKey("users.id"), nullable=False, index=True)

    # Content
    type = Column(String(50), nullable=False, index=True)
    title = Column(String(100), nullable=False)
    message = Column(Text, nullable=False)

    # Link to related resource
    link_url = Column(String(500), nullable=True)
    reference_type = Column(String(50), nullable=True)  # assignment, classroom, etc.
    reference_id = Column(Integer, nullable=True)

    # Status
    is_read = Column(Boolean, default=False, nullable=False)
    read_at = Column(DateTime(timezone=True), nullable=True)

    # Email notification sent?
    email_sent = Column(Boolean, default=False, nullable=False)

    # Timestamps
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    expires_at = Column(DateTime(timezone=True), nullable=True)  # Optional expiration

    # Indexes
    __table_args__ = (
        Index("ix_notifications_user_unread", "user_id", "is_read", "created_at"),
    )


# ============================================================================
# ACTIVITY LOG
# ============================================================================


class ActivityLog(Base):
    """Detailed activity tracking for classrooms and users."""

    __tablename__ = "activity_logs"

    id = Column(Integer, primary_key=True, index=True)

    # Actor
    user_id = Column(
        Integer, ForeignKey("users.id"), nullable=True, index=True
    )  # NULL for system

    # Target (what was acted upon)
    target_type = Column(
        String(50), nullable=False
    )  # classroom, assignment, exercise, etc.
    target_id = Column(Integer, nullable=True)

    # Action details
    action = Column(
        String(50), nullable=False
    )  # created, updated, completed, joined, etc.
    details = Column(JSON, default=dict, nullable=False)  # Additional context

    # Context
    classroom_id = Column(
        Integer, ForeignKey("classrooms.id"), nullable=True, index=True
    )
    ip_address = Column(String(45), nullable=True)

    # Timestamps
    created_at = Column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )

    # Indexes
    __table_args__ = (
        Index("ix_activity_classroom", "classroom_id", "created_at"),
        Index("ix_activity_user", "user_id", "created_at"),
    )
