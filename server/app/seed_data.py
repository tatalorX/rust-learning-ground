# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Seed data script for development environment.
Creates sample users, classrooms, and assignments.
"""
import asyncio
from datetime import datetime, timezone, timedelta

from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select

from app.database import AsyncSessionLocal
from app.auth_service import hash_password
from app.models import User, UserProgress
from app.models_school import (
    UserSchoolProfile, Classroom, ClassroomMember, Assignment,
    UserRole, GradeLevel, RankTier
)
from app.crud_school import generate_classroom_code


async def create_sample_users(db: AsyncSession) -> list[User]:
    """Create sample users for development."""
    users = []
    
    # Admin user
    admin = User(
        email="admin@rustlearning.local",
        username="admin",
        hashed_password=hash_password("admin123"),
        display_name="Admin User",
        is_active=True,
        is_verified=True,
        is_admin=True
    )
    db.add(admin)
    await db.flush()
    users.append(admin)
    
    # Teacher user
    teacher = User(
        email="teacher@rustlearning.local",
        username="teacher",
        hashed_password=hash_password("teacher123"),
        display_name="John Teacher",
        is_active=True,
        is_verified=True,
        is_admin=False
    )
    db.add(teacher)
    await db.flush()
    users.append(teacher)
    
    # Student users
    student_names = [
        ("alice", "Alice Student"),
        ("bob", "Bob Learner"),
        ("charlie", "Charlie Coder"),
        ("diana", "Diana Developer"),
        ("eve", "Eve Engineer"),
        ("frank", "Frank Rustacean"),
        ("grace", "Grace Hacker"),
        ("henry", "Henry Programmer"),
    ]
    
    for username, display_name in student_names:
        student = User(
            email=f"{username}@rustlearning.local",
            username=username,
            hashed_password=hash_password(f"{username}123"),
            display_name=display_name,
            is_active=True,
            is_verified=True,
            is_admin=False
        )
        db.add(student)
        await db.flush()
        users.append(student)
    
    return users


async def create_user_profiles(db: AsyncSession, users: list[User]):
    """Create school profiles for users."""
    profiles = []
    
    # Admin profile
    admin_profile = UserSchoolProfile(
        user_id=users[0].id,
        role=UserRole.ADMIN,
        grade_level=None,
        total_xp=50000,
        current_rank=RankTier.LEGEND,
        treats=500,
        problems_solved=250
    )
    db.add(admin_profile)
    profiles.append(admin_profile)
    
    # Teacher profile
    teacher_profile = UserSchoolProfile(
        user_id=users[1].id,
        role=UserRole.TEACHER,
        grade_level=GradeLevel.HIGH_10,
        total_xp=25000,
        current_rank=RankTier.GRANDMASTER,
        treats=250,
        problems_solved=150
    )
    db.add(teacher_profile)
    profiles.append(teacher_profile)
    
    # Student profiles with varying XP
    grade_levels = [
        GradeLevel.MIDDLE_7,
        GradeLevel.MIDDLE_8,
        GradeLevel.HIGH_9,
        GradeLevel.HIGH_10,
        GradeLevel.HIGH_11,
        GradeLevel.COLLEGE,
        GradeLevel.HIGH_12,
        GradeLevel.MIDDLE_6
    ]
    
    xp_values = [500, 1500, 3500, 8000, 12000, 18000, 22000, 800]
    
    for i, user in enumerate(users[2:]):  # Skip admin and teacher
        profile = UserSchoolProfile(
            user_id=user.id,
            role=UserRole.STUDENT,
            grade_level=grade_levels[i % len(grade_levels)],
            total_xp=xp_values[i % len(xp_values)],
            current_rank=RankTier.NOVICE,  # Will be updated
            treats=i * 5,
            problems_solved=(i + 1) * 10,
            current_streak=i + 1,
            longest_streak=(i + 1) * 2
        )
        profile.update_rank()  # Calculate correct rank
        db.add(profile)
        profiles.append(profile)
    
    return profiles


async def create_user_progress(db: AsyncSession, users: list[User]):
    """Create progress records for users."""
    for i, user in enumerate(users):
        solved = [f"{j:03d}" for j in range(1, min((i + 1) * 10 + 1, 100))]
        
        progress = UserProgress(
            user_id=user.id,
            xp=(i + 1) * 100,
            solved_problems=solved,
            current_streak=i + 1,
            longest_streak=(i + 1) * 2,
            last_solved_date=datetime.now(timezone.utc) - timedelta(days=i)
        )
        db.add(progress)


async def create_classrooms(db: AsyncSession, teacher: User) -> list[Classroom]:
    """Create sample classrooms."""
    classrooms = []
    
    classroom_data = [
        {
            "name": "Introduction to Rust",
            "description": "Learn the basics of Rust programming",
            "subject": "Computer Science",
            "grade_level": GradeLevel.HIGH_9,
            "max_students": 30
        },
        {
            "name": "Advanced Rust Concepts",
            "description": "Deep dive into ownership, lifetimes, and unsafe Rust",
            "subject": "Computer Science",
            "grade_level": GradeLevel.COLLEGE,
            "max_students": 25
        },
        {
            "name": "Rust for Web Development",
            "description": "Building web applications with Rust",
            "subject": "Web Development",
            "grade_level": GradeLevel.HIGH_11,
            "max_students": 20
        }
    ]
    
    for data in classroom_data:
        classroom = Classroom(
            teacher_id=teacher.id,
            name=data["name"],
            description=data["description"],
            subject=data["subject"],
            code=generate_classroom_code(),
            grade_level=data["grade_level"],
            max_students=data["max_students"],
            is_active=True,
            is_archived=False
        )
        db.add(classroom)
        await db.flush()
        classrooms.append(classroom)
    
    return classrooms


async def add_students_to_classrooms(
    db: AsyncSession,
    classrooms: list[Classroom],
    students: list[User]
):
    """Add students to classrooms."""
    # Distribute students across classrooms
    for i, student in enumerate(students):
        classroom = classrooms[i % len(classrooms)]
        
        member = ClassroomMember(
            classroom_id=classroom.id,
            user_id=student.id,
            is_active=True,
            problems_solved_in_class=(i + 1) * 5,
            total_xp_earned=(i + 1) * 50
        )
        db.add(member)


async def create_assignments(
    db: AsyncSession,
    classrooms: list[Classroom],
    teacher: User
) -> list[Assignment]:
    """Create sample assignments."""
    assignments = []
    
    assignment_data = [
        {
            "title": "Hello World",
            "description": "Your first Rust program",
            "exercise_ids": ["001", "002", "003"],
            "xp_reward": 50,
            "treat_reward": 5,
            "status": "published"
        },
        {
            "title": "Variables and Mutability",
            "description": "Learn about let, mut, and constants",
            "exercise_ids": ["004", "005", "006", "007"],
            "xp_reward": 75,
            "treat_reward": 8,
            "status": "published"
        },
        {
            "title": "Functions",
            "description": "Creating and using functions",
            "exercise_ids": ["008", "009", "010"],
            "xp_reward": 100,
            "treat_reward": 10,
            "status": "draft"
        }
    ]
    
    for classroom in classrooms:
        for data in assignment_data:
            assignment = Assignment(
                classroom_id=classroom.id,
                created_by=teacher.id,
                title=data["title"],
                description=data["description"],
                exercise_ids=data["exercise_ids"],
                xp_reward=data["xp_reward"],
                treat_reward=data["treat_reward"],
                status=data["status"],
                published_at=datetime.now(timezone.utc) if data["status"] == "published" else None
            )
            db.add(assignment)
            await db.flush()
            assignments.append(assignment)
    
    return assignments


async def seed_database():
    """Main seed function."""
    print("🌱 Seeding database with development data...")
    
    async with AsyncSessionLocal() as db:
        try:
            # Check if data already exists
            result = await db.execute(select(User).where(User.username == "admin"))
            if result.scalar_one_or_none():
                print("⚠️  Database already seeded. Skipping...")
                return
            
            # Create users
            print("👤 Creating users...")
            users = await create_sample_users(db)
            
            # Create profiles
            print("📚 Creating school profiles...")
            await create_user_profiles(db, users)
            
            # Create progress
            print("📊 Creating progress records...")
            await create_user_progress(db, users)
            
            # Create classrooms
            print("🏫 Creating classrooms...")
            classrooms = await create_classrooms(db, users[1])  # Teacher
            
            # Add students to classrooms
            print("👥 Adding students to classrooms...")
            await add_students_to_classrooms(db, classrooms, users[2:])
            
            # Create assignments
            print("📝 Creating assignments...")
            await create_assignments(db, classrooms, users[1])
            
            # Commit all changes
            await db.commit()
            
            print("✅ Database seeded successfully!")
            print("")
            print("Sample accounts:")
            print("  Admin:    admin@rustlearning.local / admin123")
            print("  Teacher:  teacher@rustlearning.local / teacher123")
            print("  Students: alice@rustlearning.local / alice123")
            print("            bob@rustlearning.local / bob123")
            print("            (and 6 more...)")
            
        except Exception as e:
            await db.rollback()
            print(f"❌ Seeding failed: {e}")
            raise


if __name__ == "__main__":
    asyncio.run(seed_database())
