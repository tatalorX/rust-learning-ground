# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Pydantic schemas for School Platform API.
"""
from datetime import datetime
from typing import Optional, List, Dict, Any
from pydantic import BaseModel, Field, ConfigDict

from app.models_school import UserRole, GradeLevel, RankTier


# ============================================================================
# SHARED BASE SCHEMAS
# ============================================================================

class TimestampedSchema(BaseModel):
    """Base schema with timestamps."""
    model_config = ConfigDict(from_attributes=True)
    
    created_at: datetime
    updated_at: datetime


# ============================================================================
# USER SCHOOL PROFILE SCHEMAS
# ============================================================================

class UserSchoolProfileBase(BaseModel):
    """Base school profile schema."""
    role: UserRole = UserRole.STUDENT
    grade_level: Optional[GradeLevel] = None


class UserSchoolProfileCreate(UserSchoolProfileBase):
    """Schema for creating school profile."""
    pass


class UserSchoolProfileUpdate(BaseModel):
    """Schema for updating school profile."""
    role: Optional[UserRole] = None
    grade_level: Optional[GradeLevel] = None
    current_title: Optional[str] = None


class UserSchoolProfileResponse(UserSchoolProfileBase, TimestampedSchema):
    """Full school profile response."""
    id: int
    user_id: int
    
    # Gamification
    total_xp: int
    current_rank: RankTier
    current_title: Optional[str] = None
    treats: int
    
    # Achievements
    achievements: List[str] = []
    badges: List[str] = []
    
    # Stats
    problems_solved_total: int
    problems_solved_by_difficulty: Dict[str, int]
    streak_days: int
    longest_streak_days: int
    
    # Computed fields
    rank_progress: Dict[str, Any]
    
    @staticmethod
    def calculate_rank_progress(total_xp: int, rank: RankTier) -> Dict[str, Any]:
        """Calculate progress within current rank."""
        rank_thresholds = {
            RankTier.NOVICE: (0, 1000),
            RankTier.APPRENTICE: (1000, 3000),
            RankTier.JOURNEYMAN: (3000, 6000),
            RankTier.ADEPT: (6000, 10000),
            RankTier.EXPERT: (10000, 15000),
            RankTier.MASTER: (15000, 21000),
            RankTier.GRANDMASTER: (21000, 30000),
            RankTier.LEGEND: (30000, 50000),
        }
        
        min_xp, max_xp = rank_thresholds.get(rank, (0, 1000))
        progress = total_xp - min_xp
        required = max_xp - min_xp
        percentage = min(100, (progress / required * 100)) if required > 0 else 100
        
        return {
            "current_xp": total_xp,
            "min_xp": min_xp,
            "max_xp": max_xp,
            "progress": progress,
            "required": required,
            "percentage": round(percentage, 1),
            "xp_to_next_rank": max(0, max_xp - total_xp)
        }


class UserPublicProfile(BaseModel):
    """Public profile info for other users to see."""
    model_config = ConfigDict(from_attributes=True)
    
    id: int
    username: str
    display_name: Optional[str] = None
    
    # School info
    role: UserRole
    grade_level: Optional[GradeLevel] = None
    current_rank: RankTier
    current_title: Optional[str] = None
    
    # Stats
    total_xp: int
    problems_solved_total: int
    streak_days: int
    badges: List[str] = []


# ============================================================================
# CLASSROOM SCHEMAS
# ============================================================================

class ClassroomBase(BaseModel):
    """Base classroom schema."""
    name: str = Field(..., min_length=1, max_length=200)
    description: Optional[str] = None
    grade_level: Optional[GradeLevel] = None
    max_students: int = Field(default=50, ge=1, le=200)
    color_theme: str = "blue"
    icon: str = "📚"


class ClassroomCreate(ClassroomBase):
    """Schema for creating a classroom."""
    pass


class ClassroomUpdate(BaseModel):
    """Schema for updating a classroom."""
    name: Optional[str] = Field(None, min_length=1, max_length=200)
    description: Optional[str] = None
    grade_level: Optional[GradeLevel] = None
    max_students: Optional[int] = Field(None, ge=1, le=200)
    color_theme: Optional[str] = None
    icon: Optional[str] = None
    is_active: Optional[bool] = None


class ClassroomResponse(ClassroomBase, TimestampedSchema):
    """Full classroom response."""
    model_config = ConfigDict(from_attributes=True)
    
    id: int
    code: str
    teacher_id: int
    
    is_active: bool
    is_archived: bool
    archived_at: Optional[datetime] = None
    
    # Member count
    member_count: int = 0
    
    # Teacher info
    teacher_username: Optional[str] = None
    teacher_display_name: Optional[str] = None


class ClassroomListResponse(BaseModel):
    """Simplified classroom for list views."""
    model_config = ConfigDict(from_attributes=True)
    
    id: int
    name: str
    code: str
    icon: str
    color_theme: str
    grade_level: Optional[GradeLevel] = None
    member_count: int
    is_active: bool
    created_at: datetime


class ClassroomJoinRequest(BaseModel):
    """Request to join a classroom."""
    code: str = Field(..., min_length=4, max_length=20)


class ClassroomMemberResponse(BaseModel):
    """Classroom member info."""
    model_config = ConfigDict(from_attributes=True)
    
    id: int
    user_id: int
    username: str
    display_name: Optional[str] = None
    
    role: UserRole
    grade_level: Optional[GradeLevel] = None
    
    joined_at: datetime
    is_active: bool
    
    # Progress
    xp_earned: int
    problems_solved_count: int
    current_rank: RankTier


# ============================================================================
# ASSIGNMENT SCHEMAS
# ============================================================================

class AssignmentBase(BaseModel):
    """Base assignment schema."""
    title: str = Field(..., min_length=1, max_length=200)
    description: Optional[str] = None
    problem_ids: List[str] = []  # e.g., ["001", "002", "150"]
    due_at: Optional[datetime] = None
    allow_late_submission: bool = True
    xp_bonus: int = Field(default=0, ge=0)


class AssignmentCreate(AssignmentBase):
    """Schema for creating an assignment."""
    classroom_id: int


class AssignmentUpdate(BaseModel):
    """Schema for updating an assignment."""
    title: Optional[str] = Field(None, min_length=1, max_length=200)
    description: Optional[str] = None
    problem_ids: Optional[List[str]] = None
    due_at: Optional[datetime] = None
    allow_late_submission: Optional[bool] = None
    is_published: Optional[bool] = None
    xp_bonus: Optional[int] = Field(None, ge=0)


class AssignmentResponse(AssignmentBase, TimestampedSchema):
    """Full assignment response."""
    model_config = ConfigDict(from_attributes=True)
    
    id: int
    classroom_id: int
    assigned_at: datetime
    is_published: bool
    
    # Statistics
    total_students: int = 0
    completed_count: int = 0
    completion_percentage: float = 0.0
    
    # Student-specific fields (populated when viewing as student)
    is_completed: Optional[bool] = None
    problems_completed: Optional[List[str]] = None


class AssignmentSubmissionResponse(BaseModel):
    """Assignment submission response."""
    model_config = ConfigDict(from_attributes=True)
    
    id: int
    assignment_id: int
    student_id: int
    student_username: Optional[str] = None
    student_display_name: Optional[str] = None
    
    problems_completed: List[str]
    completed_at: Optional[datetime] = None
    xp_earned: int
    
    created_at: datetime
    updated_at: datetime


# ============================================================================
# GAMIFICATION - RANK SCHEMAS
# ============================================================================

class RankDefinitionResponse(BaseModel):
    """Rank definition response."""
    model_config = ConfigDict(from_attributes=True)
    
    rank_tier: RankTier
    min_xp: int
    max_xp: Optional[int]
    
    name: str
    description: Optional[str] = None
    icon: str
    color: str
    
    treats_reward: int
    title_unlocked: Optional[str] = None


class RankProgressResponse(BaseModel):
    """User's rank progress response."""
    current_rank: RankTier
    current_xp: int
    min_xp: int
    max_xp: int
    progress: int
    required: int
    percentage: float
    xp_to_next_rank: int
    
    next_rank: Optional[RankTier] = None
    next_rank_name: Optional[str] = None


# ============================================================================
# GAMIFICATION - TITLE SCHEMAS
# ============================================================================

class TitleDefinitionResponse(BaseModel):
    """Title definition response."""
    model_config = ConfigDict(from_attributes=True)
    
    key: str
    name: str
    description: Optional[str] = None
    category: str
    
    unlock_conditions: Dict[str, Any]
    icon: Optional[str] = None
    color: Optional[str] = None
    
    is_rare: bool
    is_active: bool


class UserTitleResponse(BaseModel):
    """User's earned title response."""
    model_config = ConfigDict(from_attributes=True)
    
    id: int
    title_key: str
    title_name: str
    title_description: Optional[str] = None
    title_icon: Optional[str] = None
    title_color: Optional[str] = None
    title_category: Optional[str] = None
    is_rare: bool = False
    
    earned_at: datetime
    is_equipped: bool


class EquipTitleRequest(BaseModel):
    """Request to equip/unequip a title."""
    title_key: Optional[str] = None  # None = unequip


# ============================================================================
# GAMIFICATION - TREAT SCHEMAS
# ============================================================================

class TreatTransactionResponse(BaseModel):
    """Treat transaction response."""
    model_config = ConfigDict(from_attributes=True)
    
    id: int
    amount: int
    balance_after: int
    transaction_type: str
    description: Optional[str] = None
    created_at: datetime


class TreatBalanceResponse(BaseModel):
    """User's treat balance."""
    balance: int
    total_earned: int
    total_spent: int


class TreatRewardResponse(BaseModel):
    """Treat reward definition."""
    model_config = ConfigDict(from_attributes=True)
    
    action_type: str
    treats_amount: int
    description: Optional[str] = None
    cooldown_seconds: int


# ============================================================================
# LEADERBOARD SCHEMAS
# ============================================================================

class LeaderboardEntryResponse(BaseModel):
    """Leaderboard entry response."""
    model_config = ConfigDict(from_attributes=True)
    
    rank: int
    user_id: int
    username: str
    display_name: Optional[str] = None
    
    current_rank: RankTier
    current_title: Optional[str] = None
    
    xp_total: int
    problems_solved: int
    
    # Is this the current user?
    is_current_user: bool = False


class LeaderboardResponse(BaseModel):
    """Full leaderboard response."""
    scope_type: str
    scope_id: Optional[int] = None
    scope_name: Optional[str] = None
    period: str
    
    entries: List[LeaderboardEntryResponse]
    
    # Pagination
    total_count: int
    current_user_rank: Optional[int] = None


# ============================================================================
# PROGRESS & STATISTICS SCHEMAS
# ============================================================================

class StudentProgressResponse(BaseModel):
    """Detailed student progress for teacher view."""
    model_config = ConfigDict(from_attributes=True)
    
    user_id: int
    username: str
    display_name: Optional[str] = None
    grade_level: Optional[GradeLevel] = None
    
    # Overall stats
    total_xp: int
    current_rank: RankTier
    problems_solved_total: int
    streak_days: int
    
    # Classroom-specific
    classroom_id: int
    joined_at: datetime
    xp_earned_in_class: int
    problems_solved_in_class: int
    
    # Assignment progress
    assignments_completed: int
    assignments_total: int
    completion_percentage: float


class ClassStatisticsResponse(BaseModel):
    """Statistics for a classroom."""
    classroom_id: int
    classroom_name: str
    
    total_students: int
    active_students: int
    
    # Aggregated stats
    total_problems_solved: int
    average_problems_per_student: float
    total_xp_earned: int
    
    # Rank distribution
    rank_distribution: Dict[str, int]
    
    # Activity (last 7 days)
    recent_activity: List[Dict[str, Any]]
    
    # Top performers
    top_students: List[Dict[str, Any]]


# ============================================================================
# GRADE CONTENT MAPPING SCHEMAS
# ============================================================================

class GradeContentMappingResponse(BaseModel):
    """Content mapping for grade levels."""
    model_config = ConfigDict(from_attributes=True)
    
    id: int
    grade_level: GradeLevel
    exercise_id: str
    is_recommended: bool
    difficulty_modifier: int
    required_prior_exercises: List[str]


class RecommendedExercisesResponse(BaseModel):
    """Recommended exercises for a user."""
    grade_level: GradeLevel
    
    exercises: List[Dict[str, Any]]
    
    # Progress stats for this grade level
    total_recommended: int
    completed: int
    percentage: float
