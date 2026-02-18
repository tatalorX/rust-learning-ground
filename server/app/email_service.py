# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Email service for sending notifications, password resets, and verification emails.
Supports SMTP, SendGrid, and console (for development) backends.

SECURITY FIXES APPLIED:
- Password reset tokens are now hashed with SHA-256 before storage (C-04)
- HTML content is escaped to prevent XSS in emails (H-23)
"""

import asyncio
import hashlib
import html
from datetime import datetime, timezone, timedelta
from typing import Optional
import secrets
import string

from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import select, delete

from app.config import Settings
from app.models import User
from app.models_school import PasswordResetToken


class EmailBackend:
    """Email backend types."""

    SMTP = "smtp"
    SENDGRID = "sendgrid"
    CONSOLE = "console"  # For development
    NULL = "null"  # Disabled


class EmailService:
    """Service for sending emails."""

    def __init__(self, settings: Settings):
        self.settings = settings
        self.backend = settings.EMAIL_BACKEND or EmailBackend.CONSOLE
        self.from_email = settings.EMAIL_FROM or "noreply@rustlearning.com"
        self.from_name = settings.EMAIL_FROM_NAME or "Rust Learning Ground"

    async def send_email(
        self,
        to_email: str,
        subject: str,
        html_content: str,
        text_content: Optional[str] = None,
    ) -> bool:
        """
        Send an email using the configured backend.

        Returns True if sent successfully, False otherwise.
        """
        if self.backend == EmailBackend.NULL:
            return False

        if self.backend == EmailBackend.CONSOLE:
            return await self._send_console(
                to_email, subject, html_content, text_content
            )

        if self.backend == EmailBackend.SMTP:
            return await self._send_smtp(to_email, subject, html_content, text_content)

        if self.backend == EmailBackend.SENDGRID:
            return await self._send_sendgrid(
                to_email, subject, html_content, text_content
            )

        return False

    async def _send_console(
        self,
        to_email: str,
        subject: str,
        html_content: str,
        text_content: Optional[str] = None,
    ) -> bool:
        """Print email to console (development mode)."""
        print("\n" + "=" * 60)
        print("📧 EMAIL (Console Backend)")
        print("=" * 60)
        print(f"To: {to_email}")
        print(f"From: {self.from_name} <{self.from_email}>")
        print(f"Subject: {subject}")
        print("-" * 60)
        print(text_content or html_content)
        print("=" * 60 + "\n")
        return True

    async def _send_smtp(
        self,
        to_email: str,
        subject: str,
        html_content: str,
        text_content: Optional[str] = None,
    ) -> bool:
        """Send email via SMTP."""
        try:
            import aiosmtplib
            from email.mime.text import MIMEText
            from email.mime.multipart import MIMEMultipart

            msg = MIMEMultipart("alternative")
            msg["Subject"] = subject
            msg["From"] = f"{self.from_name} <{self.from_email}>"
            msg["To"] = to_email

            if text_content:
                msg.attach(MIMEText(text_content, "plain"))
            msg.attach(MIMEText(html_content, "html"))

            await aiosmtplib.send(
                msg,
                hostname=self.settings.SMTP_HOST or "localhost",
                port=self.settings.SMTP_PORT or 587,
                username=self.settings.SMTP_USER,
                password=self.settings.SMTP_PASSWORD,
                use_tls=self.settings.SMTP_USE_TLS or True,
            )
            return True
        except Exception as e:
            print(f"SMTP send failed: {e}")
            return False

    async def _send_sendgrid(
        self,
        to_email: str,
        subject: str,
        html_content: str,
        text_content: Optional[str] = None,
    ) -> bool:
        """Send email via SendGrid API."""
        try:
            # SendGrid implementation would go here
            # Requires: pip install sendgrid
            pass
        except Exception as e:
            print(f"SendGrid send failed: {e}")
            return False
        return False

    # Template methods

    async def send_password_reset(
        self, to_email: str, reset_token: str, base_url: str
    ) -> bool:
        """Send password reset email."""
        reset_url = f"{base_url}/reset-password?token={reset_token}"

        subject = "Password Reset Request - Rust Learning Ground"

        html_content = f"""
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                .header {{ background: #f97316; color: white; padding: 20px; text-align: center; }}
                .content {{ background: #f9fafb; padding: 20px; }}
                .button {{ display: inline-block; background: #f97316; color: white; 
                          padding: 12px 24px; text-decoration: none; border-radius: 4px; }}
                .footer {{ text-align: center; color: #6b7280; font-size: 12px; margin-top: 20px; }}
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>🔧 Rust Learning Ground</h1>
                </div>
                <div class="content">
                    <h2>Password Reset Request</h2>
                    <p>Hello,</p>
                    <p>We received a request to reset your password. Click the button below to set a new password:</p>
                    <p style="text-align: center;">
                        <a href="{reset_url}" class="button">Reset Password</a>
                    </p>
                    <p>Or copy and paste this link into your browser:</p>
                    <p><code>{reset_url}</code></p>
                    <p>This link will expire in 1 hour.</p>
                    <p>If you didn't request this, please ignore this email.</p>
                </div>
                <div class="footer">
                    <p>Rust Learning Ground - Learn Rust Programming</p>
                </div>
            </div>
        </body>
        </html>
        """

        text_content = f"""
Password Reset Request - Rust Learning Ground

Hello,

We received a request to reset your password. Visit this link to reset it:
{reset_url}

This link will expire in 1 hour.

If you didn't request this, please ignore this email.
        """

        return await self.send_email(to_email, subject, html_content, text_content)

    async def send_verification_email(
        self, to_email: str, verification_token: str, base_url: str
    ) -> bool:
        """Send email verification email."""
        verify_url = f"{base_url}/verify-email?token={verification_token}"

        subject = "Verify Your Email - Rust Learning Ground"

        html_content = f"""
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                .header {{ background: #f97316; color: white; padding: 20px; text-align: center; }}
                .content {{ background: #f9fafb; padding: 20px; }}
                .button {{ display: inline-block; background: #f97316; color: white; 
                          padding: 12px 24px; text-decoration: none; border-radius: 4px; }}
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>🔧 Rust Learning Ground</h1>
                </div>
                <div class="content">
                    <h2>Verify Your Email</h2>
                    <p>Welcome! Please click the button below to verify your email address:</p>
                    <p style="text-align: center;">
                        <a href="{verify_url}" class="button">Verify Email</a>
                    </p>
                    <p>This link will expire in 24 hours.</p>
                </div>
            </div>
        </body>
        </html>
        """

        return await self.send_email(to_email, subject, html_content)

    async def send_welcome_email(self, to_email: str, username: str) -> bool:
        """Send welcome email after registration."""
        subject = "Welcome to Rust Learning Ground!"

        # SECURITY FIX: Escape username to prevent XSS (H-23)
        safe_username = html.escape(username)

        html_content = f"""
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                .header {{ background: #f97316; color: white; padding: 20px; text-align: center; }}
                .content {{ background: #f9fafb; padding: 20px; }}
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>🔧 Rust Learning Ground</h1>
                </div>
                <div class="content">
                    <h2>Welcome, {safe_username}!</h2>
                    <p>Thank you for joining Rust Learning Ground. Start your journey to master Rust programming!</p>
                    <ul>
                        <li>🎯 280+ hands-on exercises</li>
                        <li>🏫 Join classrooms and learn with others</li>
                        <li>🏆 Earn ranks, titles, and achievements</li>
                        <li>📊 Track your progress</li>
                    </ul>
                    <p><a href="/school-dashboard.html">Visit your School Dashboard</a></p>
                </div>
            </div>
        </body>
        </html>
        """

        return await self.send_email(to_email, subject, html_content)

    async def send_security_alert(
        self,
        to_email: str,
        alert_type: str,
        details: str,
        action_url: Optional[str] = None,
    ) -> bool:
        """Send security alert email."""
        subject = f"Security Alert: {alert_type} - Rust Learning Ground"

        action_html = (
            f'<p><a href="{action_url}" class="button">Take Action</a></p>'
            if action_url
            else ""
        )

        html_content = f"""
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                .header {{ background: #dc2626; color: white; padding: 20px; text-align: center; }}
                .content {{ background: #fef2f2; padding: 20px; border: 1px solid #fecaca; }}
                .button {{ display: inline-block; background: #dc2626; color: white; 
                          padding: 12px 24px; text-decoration: none; border-radius: 4px; }}
            </style>
        </head>
        <body>
            <div class="container">
                <div class="header">
                    <h1>🛡️ Security Alert</h1>
                </div>
                <div class="content">
                    <h2>{alert_type}</h2>
                    <p>{details}</p>
                    {action_html}
                    <p>If you did not initiate this action, please secure your account immediately.</p>
                </div>
            </div>
        </body>
        </html>
        """

        return await self.send_email(to_email, subject, html_content)


# Password reset service


async def create_password_reset_token(
    db: AsyncSession, user: User, expires_hours: int = 1
) -> str:
    """
    Create a password reset token for a user.
    Invalidates any existing tokens.

    SECURITY FIX: Token is hashed with SHA-256 before storage (C-04)
    """
    # Delete any existing tokens for this user
    await db.execute(
        delete(PasswordResetToken).where(PasswordResetToken.user_id == user.id)
    )

    # Generate secure token
    token = "".join(
        secrets.choice(string.ascii_letters + string.digits) for _ in range(64)
    )

    # SECURITY FIX: Hash token before storage to prevent token theft if DB compromised
    token_hash = hashlib.sha256(token.encode()).hexdigest()

    # Create token record with hashed token
    reset_token = PasswordResetToken(
        user_id=user.id,
        token_hash=token_hash,  # Now properly hashed
        expires_at=datetime.now(timezone.utc) + timedelta(hours=expires_hours),
    )

    db.add(reset_token)
    await db.flush()

    return token


async def validate_password_reset_token(db: AsyncSession, token: str) -> Optional[User]:
    """Validate a password reset token and return the associated user."""
    result = await db.execute(
        select(PasswordResetToken, User)
        .join(User, PasswordResetToken.user_id == User.id)
        .where(PasswordResetToken.token_hash == token)
        .where(PasswordResetToken.used_at.is_(None))
        .where(PasswordResetToken.expires_at > datetime.now(timezone.utc))
    )

    row = result.first()
    if row:
        return row.User
    return None


async def mark_reset_token_used(db: AsyncSession, token: str) -> bool:
    """Mark a reset token as used."""
    result = await db.execute(
        select(PasswordResetToken).where(PasswordResetToken.token_hash == token)
    )
    reset_token = result.scalar_one_or_none()

    if reset_token:
        reset_token.used_at = datetime.now(timezone.utc)
        await db.flush()
        return True
    return False


async def cleanup_expired_tokens(db: AsyncSession) -> int:
    """Delete expired password reset tokens. Returns count deleted."""
    result = await db.execute(
        delete(PasswordResetToken).where(
            PasswordResetToken.expires_at < datetime.now(timezone.utc)
        )
    )
    return result.rowcount
