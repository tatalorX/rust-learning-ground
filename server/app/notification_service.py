# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Notification service for user notifications.
Supports in-app and email notifications.
"""
from datetime import datetime, timezone, timedelta
from typing import Optional, List, Dict, Any

from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, desc, update, func

from app.models_school import Notification, NotificationType
from app.email_service import EmailService
from app.config import Settings


class NotificationService:
    """Service for managing user notifications."""
    
    def __init__(self, db: AsyncSession, settings: Settings):
        self.db = db
        self.settings = settings
        self.email_service = EmailService(settings)
    
    async def create_notification(
        self,
        user_id: int,
        type: str,
        title: str,
        message: str,
        link_url: Optional[str] = None,
        reference_type: Optional[str] = None,
        reference_id: Optional[int] = None,
        send_email: bool = False,
        email_address: Optional[str] = None,
        expires_hours: Optional[int] = None
    ) -> Notification:
        """Create a new notification for a user."""
        notification = Notification(
            user_id=user_id,
            type=type,
            title=title,
            message=message,
            link_url=link_url,
            reference_type=reference_type,
            reference_id=reference_id,
            expires_at=datetime.now(timezone.utc) + timedelta(hours=expires_hours) if expires_hours else None
        )
        
        self.db.add(notification)
        await self.db.flush()
        
        # Send email if requested
        if send_email and email_address:
            await self.email_service.send_email(
                to_email=email_address,
                subject=title,
                html_content=f"""
                <h2>{title}</h2>
                <p>{message}</p>
                {f'<p><a href="{link_url}">View Details</a></p>' if link_url else ''}
                """,
                text_content=f"{title}\n\n{message}"
            )
            notification.email_sent = True
            await self.db.flush()
        
        return notification
    
    async def get_user_notifications(
        self,
        user_id: int,
        unread_only: bool = False,
        limit: int = 50,
        offset: int = 0
    ) -> List[Notification]:
        """Get notifications for a user."""
        query = select(Notification).where(Notification.user_id == user_id)
        
        if unread_only:
            query = query.where(Notification.is_read == False)
        
        # Filter out expired notifications
        query = query.where(
            (Notification.expires_at.is_(None)) | 
            (Notification.expires_at > datetime.now(timezone.utc))
        )
        
        result = await self.db.execute(
            query.order_by(desc(Notification.created_at))
            .limit(limit)
            .offset(offset)
        )
        return list(result.scalars().all())
    
    async def get_unread_count(self, user_id: int) -> int:
        """Get count of unread notifications for a user."""
        result = await self.db.execute(
            select(func.count(Notification.id))
            .where(Notification.user_id == user_id)
            .where(Notification.is_read == False)
            .where(
                (Notification.expires_at.is_(None)) | 
                (Notification.expires_at > datetime.now(timezone.utc))
            )
        )
        return result.scalar() or 0
    
    async def mark_as_read(self, notification_id: int, user_id: int) -> bool:
        """Mark a notification as read."""
        result = await self.db.execute(
            update(Notification)
            .where(Notification.id == notification_id)
            .where(Notification.user_id == user_id)
            .where(Notification.is_read == False)
            .values(is_read=True, read_at=datetime.now(timezone.utc))
        )
        return result.rowcount > 0
    
    async def mark_all_as_read(self, user_id: int) -> int:
        """Mark all notifications as read for a user. Returns count updated."""
        result = await self.db.execute(
            update(Notification)
            .where(Notification.user_id == user_id)
            .where(Notification.is_read == False)
            .values(is_read=True, read_at=datetime.now(timezone.utc))
        )
        return result.rowcount
    
    async def delete_notification(self, notification_id: int, user_id: int) -> bool:
        """Delete a notification."""
        from sqlalchemy import delete
        
        result = await self.db.execute(
            delete(Notification)
            .where(Notification.id == notification_id)
            .where(Notification.user_id == user_id)
        )
        return result.rowcount > 0
    
    # Helper methods for common notification types
    
    async def notify_assignment_due(
        self,
        user_id: int,
        assignment_title: str,
        due_date: datetime,
        classroom_name: str,
        link_url: Optional[str] = None
    ) -> Notification:
        """Notify user that an assignment is due soon."""
        time_left = due_date - datetime.now(timezone.utc)
        hours_left = int(time_left.total_seconds() / 3600)
        
        return await self.create_notification(
            user_id=user_id,
            type=NotificationType.ASSIGNMENT_DUE,
            title="Assignment Due Soon",
            message=f"'{assignment_title}' in {classroom_name} is due in {hours_left} hours.",
            link_url=link_url,
            reference_type="assignment",
            expires_at=due_date
        )
    
    async def notify_assignment_graded(
        self,
        user_id: int,
        assignment_title: str,
        score: int,
        classroom_name: str,
        link_url: Optional[str] = None
    ) -> Notification:
        """Notify user that their assignment has been graded."""
        return await self.create_notification(
            user_id=user_id,
            type=NotificationType.ASSIGNMENT_GRADED,
            title="Assignment Graded",
            message=f"Your submission for '{assignment_title}' in {classroom_name} has been graded. Score: {score}%",
            link_url=link_url,
            reference_type="assignment"
        )
    
    async def notify_rank_up(
        self,
        user_id: int,
        new_rank: str,
        treats_rewarded: int,
        link_url: Optional[str] = None
    ) -> Notification:
        """Notify user that they've ranked up."""
        return await self.create_notification(
            user_id=user_id,
            type=NotificationType.RANK_UP,
            title=f"🎉 Rank Up: {new_rank}!",
            message=f"Congratulations! You've reached the rank of {new_rank}. You earned {treats_rewarded} treats!",
            link_url=link_url
        )
    
    async def notify_title_earned(
        self,
        user_id: int,
        title_name: str,
        link_url: Optional[str] = None
    ) -> Notification:
        """Notify user that they've earned a new title."""
        return await self.create_notification(
            user_id=user_id,
            type=NotificationType.TITLE_EARNED,
            title="🏆 New Title Unlocked!",
            message=f"You've earned the title '{title_name}'! Equip it in your profile.",
            link_url=link_url
        )
    
    async def notify_streak_warning(
        self,
        user_id: int,
        current_streak: int,
        hours_left: int,
        link_url: Optional[str] = None
    ) -> Notification:
        """Warn user that their streak is about to expire."""
        return await self.create_notification(
            user_id=user_id,
            type=NotificationType.STREAK_WARNING,
            title="⚠️ Streak Expiring Soon!",
            message=f"Your {current_streak}-day streak expires in {hours_left} hours. Solve a problem to keep it going!",
            link_url=link_url,
            expires_hours=hours_left
        )
    
    async def notify_classroom_invite(
        self,
        user_id: int,
        classroom_name: str,
        teacher_name: str,
        link_url: Optional[str] = None
    ) -> Notification:
        """Notify user of a classroom invitation."""
        return await self.create_notification(
            user_id=user_id,
            type=NotificationType.CLASSROOM_INVITE,
            title="📚 Classroom Invitation",
            message=f"{teacher_name} invited you to join '{classroom_name}'.",
            link_url=link_url,
            reference_type="classroom"
        )
    
    async def notify_achievement(
        self,
        user_id: int,
        achievement_name: str,
        description: str,
        link_url: Optional[str] = None
    ) -> Notification:
        """Notify user of an achievement."""
        return await self.create_notification(
            user_id=user_id,
            type=NotificationType.ACHIEVEMENT,
            title=f"🌟 Achievement Unlocked: {achievement_name}",
            message=description,
            link_url=link_url
        )


# Notification router
from fastapi import APIRouter, Depends, HTTPException, status
from app.database import get_db
from app.config import get_settings
from app.auth_deps import get_current_user

router = APIRouter(prefix="/api/notifications", tags=["notifications"])


@router.get("/", response_model=List[Dict[str, Any]])
async def get_my_notifications(
    unread_only: bool = False,
    limit: int = 50,
    offset: int = 0,
    current_user = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """Get current user's notifications."""
    service = NotificationService(db, get_settings())
    notifications = await service.get_user_notifications(
        user_id=current_user.id,
        unread_only=unread_only,
        limit=limit,
        offset=offset
    )
    
    return [
        {
            "id": n.id,
            "type": n.type,
            "title": n.title,
            "message": n.message,
            "link_url": n.link_url,
            "is_read": n.is_read,
            "created_at": n.created_at.isoformat() if n.created_at else None,
            "reference_type": n.reference_type,
            "reference_id": n.reference_id,
        }
        for n in notifications
    ]


@router.get("/unread-count")
async def get_unread_notification_count(
    current_user = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """Get count of unread notifications."""
    service = NotificationService(db, get_settings())
    count = await service.get_unread_count(current_user.id)
    return {"unread_count": count}


@router.post("/{notification_id}/read")
async def mark_notification_read(
    notification_id: int,
    current_user = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """Mark a notification as read."""
    service = NotificationService(db, get_settings())
    success = await service.mark_as_read(notification_id, current_user.id)
    
    if not success:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Notification not found or already read"
        )
    
    return {"message": "Notification marked as read"}


@router.post("/read-all")
async def mark_all_notifications_read(
    current_user = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """Mark all notifications as read."""
    service = NotificationService(db, get_settings())
    count = await service.mark_all_as_read(current_user.id)
    return {"message": f"{count} notifications marked as read"}


@router.delete("/{notification_id}")
async def delete_notification_endpoint(
    notification_id: int,
    current_user = Depends(get_current_user),
    db: AsyncSession = Depends(get_db)
):
    """Delete a notification."""
    service = NotificationService(db, get_settings())
    success = await service.delete_notification(notification_id, current_user.id)
    
    if not success:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Notification not found"
        )
    
    return {"message": "Notification deleted"}



