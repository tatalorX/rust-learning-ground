# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
FastAPI application with secure authentication.

Sovereign deployment: Qubes-inspired compartmentalized architecture.
See SOVEREIGN_DEPLOY_GUIDE.md.pdf for deployment instructions.
"""

import hmac
import secrets
from contextlib import asynccontextmanager
from datetime import datetime, timezone
from typing import Optional

from fastapi import FastAPI, Depends, HTTPException, status, Request, Response
from fastapi.middleware.cors import CORSMiddleware
from fastapi.middleware.trustedhost import TrustedHostMiddleware

from fastapi.responses import JSONResponse, RedirectResponse
from slowapi import Limiter, _rate_limit_exceeded_handler
from slowapi.util import get_remote_address
from slowapi.errors import RateLimitExceeded
from sqlalchemy.ext.asyncio import AsyncSession

from app.config import get_settings, get_security_headers
from app.database import init_db, close_db, get_db, AsyncSessionLocal
from app.security import (
    get_cookie_settings,
    generate_csrf_token,
    validate_csrf_token,
    get_token_hash,
)
from app.auth_deps import get_current_user, optional_user
from app.auth_service import (
    create_user,
    authenticate_user,
    handle_failed_login,
    create_user_tokens,
    refresh_access_token,
    revoke_refresh_token,
    revoke_all_user_tokens,
    get_or_create_github_user,
    get_user_progress,
    update_user_progress,
    log_audit_event,
    AuthError,
    hash_email_for_log,
)
from app.github_oauth import get_github_auth_url, complete_github_auth
from app.schemas import (
    UserCreate,
    UserLogin,
    UserResponse,
    UserProfileResponse,
    TokenResponse,
    ErrorResponse,
    ProgressResponse,
    ProblemSubmission,
    GitHubAuthUrl,
    PasswordChange,
    GitHubCallback,
)
from app.models import User, AuditLog
from app.models_school import UserSchoolProfile

# Routers
from app.routers import school_profile, school_classroom, school_assignments
from app.routers import auth_extended, admin, gdpr
from app import crud_school as school_crud
from app.cache_service import CacheService
from app.websocket_manager import router as ws_router
from app.logging_config import get_logger, setup_logging
from app.routers import api_v4

settings = get_settings()
logger = get_logger(__name__)

# Rate limiter
limiter = Limiter(key_func=get_remote_address)

# Cache service
cache_service = CacheService(settings)


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan handler."""
    # Initialize logging
    setup_logging()

    # Startup
    await init_db()
    await cache_service.connect()

    # Initialize School Platform data (ranks, titles)
    async with AsyncSessionLocal() as db:
        try:
            await school_crud.initialize_ranks(db)
            await school_crud.initialize_titles(db)
            await db.commit()
            logger.info("School Platform initialized")
        except Exception as e:
            logger.warning(
                "School Platform initialization warning", extra={"error": str(e)}
            )

    # Seed development data
    if settings.DEBUG or settings.ENVIRONMENT == "development":
        try:
            from app.seed_data import seed_database

            await seed_database()
        except Exception as e:
            logger.warning("Seed data warning", extra={"error": str(e)})

    yield
    # Shutdown
    await cache_service.close()
    await close_db()


app = FastAPI(
    title=settings.APP_NAME,
    version="3.0.0",
    docs_url="/api/docs" if settings.DEBUG else None,
    redoc_url="/api/redoc" if settings.DEBUG else None,
    openapi_url="/api/openapi.json" if settings.DEBUG else None,
    lifespan=lifespan,
)

# Rate limiting
app.state.limiter = limiter
app.add_exception_handler(RateLimitExceeded, _rate_limit_exceeded_handler)

# CORS - Dynamic configuration with wildcard support
# Allows ngrok URLs and other dynamic origins
cors_kwargs = {
    "allow_origins": settings.cors_origins_list,
    "allow_credentials": True,
    "allow_methods": ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
    "allow_headers": ["Content-Type", "Authorization", "X-CSRF-Token"],
    "expose_headers": ["X-CSRF-Token"],
}

# Add regex pattern for wildcard origins (e.g., *.ngrok-free.app)
if settings.cors_origins_regex:
    cors_kwargs["allow_origin_regex"] = settings.cors_origins_regex

app.add_middleware(CORSMiddleware, **cors_kwargs)

# Trusted hosts in production
if not settings.DEBUG:
    app.add_middleware(TrustedHostMiddleware, allowed_hosts=settings.allowed_hosts_list)

# ============== Middleware ==============


@app.middleware("http")
async def security_headers_middleware(request: Request, call_next):
    """Add security headers to all responses."""
    response = await call_next(request)
    headers = get_security_headers(settings.DEBUG)
    for header, value in headers.items():
        response.headers[header] = value
    return response


# ============== Dependencies ==============


def get_client_ip(request: Request) -> str:
    """Get client IP address safely."""
    forwarded = request.headers.get("X-Forwarded-For")
    trusted_proxies = settings.TRUSTED_PROXIES

    if forwarded and trusted_proxies > 0:
        ips = [ip.strip() for ip in forwarded.split(",")]
        if len(ips) >= trusted_proxies:
            client_ip_index = len(ips) - trusted_proxies - 1
            if client_ip_index >= 0:
                return ips[client_ip_index][:45]

    return request.client.host if request.client else "unknown"


# ============== Auth Routes ==============


@app.post("/api/auth/register", response_model=TokenResponse)
@limiter.limit(settings.RATE_LIMIT_REGISTER)
async def register(
    request: Request,
    user_data: UserCreate,
    response: Response,
    db: AsyncSession = Depends(get_db),
):
    """Register a new user account."""
    client_ip = get_client_ip(request)

    try:
        user = await create_user(db, user_data)

        # Create tokens
        access_token, refresh_token = await create_user_tokens(
            db, user, ip_address=client_ip, user_agent=request.headers.get("user-agent")
        )

        # Log success (with hashed email for privacy)
        await log_audit_event(
            db,
            "register",
            "success",
            user.id,
            client_ip,
            request.headers.get("user-agent"),
            {"method": "password", "email_hash": hash_email_for_log(user_data.email)},
        )

        # Set cookies
        cookie_settings = get_cookie_settings()
        response.set_cookie(
            key="access_token",
            value=access_token,
            max_age=settings.ACCESS_TOKEN_EXPIRE_MINUTES * 60,
            **cookie_settings,
        )
        response.set_cookie(
            key="refresh_token",
            value=refresh_token,
            max_age=settings.REFRESH_TOKEN_EXPIRE_DAYS * 86400,
            **cookie_settings,
        )

        return TokenResponse(
            access_token=access_token,
            expires_in=settings.ACCESS_TOKEN_EXPIRE_MINUTES * 60,
            user=UserResponse.model_validate(user),
        )

    except AuthError as e:
        await log_audit_event(
            db,
            "register",
            "failure",
            None,
            client_ip,
            request.headers.get("user-agent"),
            {"error": e.code, "email_hash": hash_email_for_log(user_data.email)},
        )
        raise HTTPException(status_code=status.HTTP_400_BAD_REQUEST, detail=e.message)


@app.post("/api/auth/login", response_model=TokenResponse)
@limiter.limit(settings.RATE_LIMIT_LOGIN)
async def login(
    request: Request,
    credentials: UserLogin,
    response: Response,
    db: AsyncSession = Depends(get_db),
):
    """Authenticate user and return tokens."""
    client_ip = get_client_ip(request)

    try:
        user = await authenticate_user(db, credentials.email, credentials.password)

        # Create tokens
        access_token, refresh_token = await create_user_tokens(
            db, user, ip_address=client_ip, user_agent=request.headers.get("user-agent")
        )

        # Log success
        await log_audit_event(
            db,
            "login",
            "success",
            user.id,
            client_ip,
            request.headers.get("user-agent"),
            {"method": "password"},
        )

        # Set cookies
        cookie_settings = get_cookie_settings()
        response.set_cookie(
            key="access_token",
            value=access_token,
            max_age=settings.ACCESS_TOKEN_EXPIRE_MINUTES * 60,
            **cookie_settings,
        )
        response.set_cookie(
            key="refresh_token",
            value=refresh_token,
            max_age=settings.REFRESH_TOKEN_EXPIRE_DAYS * 86400,
            **cookie_settings,
        )

        return TokenResponse(
            access_token=access_token,
            expires_in=settings.ACCESS_TOKEN_EXPIRE_MINUTES * 60,
            user=UserResponse.model_validate(user),
        )

    except AuthError as e:
        # Record failed login for account lockout
        await handle_failed_login(db, credentials.email)

        await log_audit_event(
            db,
            "login",
            "failure",
            None,
            client_ip,
            request.headers.get("user-agent"),
            {"error": e.code, "email_hash": hash_email_for_log(credentials.email)},
        )
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail=e.message)


@app.post("/api/auth/refresh", response_model=TokenResponse)
@limiter.limit(settings.RATE_LIMIT_REFRESH)
async def refresh_token(
    request: Request,
    response: Response,
    db: AsyncSession = Depends(get_db),
):
    """Refresh access token using refresh token."""
    refresh_token = request.cookies.get("refresh_token")
    if not refresh_token:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED, detail="No refresh token"
        )

    try:
        access_token, new_refresh_token, user = await refresh_access_token(
            db, refresh_token, get_client_ip(request)
        )

        # Set new cookies
        cookie_settings = get_cookie_settings()
        response.set_cookie(
            key="access_token",
            value=access_token,
            max_age=settings.ACCESS_TOKEN_EXPIRE_MINUTES * 60,
            **cookie_settings,
        )
        response.set_cookie(
            key="refresh_token",
            value=new_refresh_token,
            max_age=settings.REFRESH_TOKEN_EXPIRE_DAYS * 86400,
            **cookie_settings,
        )

        return TokenResponse(
            access_token=access_token,
            expires_in=settings.ACCESS_TOKEN_EXPIRE_MINUTES * 60,
            user=UserResponse.model_validate(user),
        )

    except AuthError as e:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail=e.message)


@app.post("/api/auth/logout")
async def logout(
    request: Request,
    response: Response,
    db: AsyncSession = Depends(get_db),
    user: Optional[User] = Depends(optional_user),
):
    """Logout user and revoke refresh token."""
    refresh_token = request.cookies.get("refresh_token")

    if refresh_token:
        await revoke_refresh_token(db, refresh_token)

    if user:
        await log_audit_event(
            db,
            "logout",
            "success",
            user.id,
            get_client_ip(request),
            request.headers.get("user-agent"),
        )

    # Clear cookies
    cookie_settings = get_cookie_settings()
    response.delete_cookie(key="access_token", **cookie_settings)
    response.delete_cookie(key="refresh_token", **cookie_settings)

    return {"success": True, "message": "Logged out successfully"}


@app.post("/api/auth/logout-all")
async def logout_all(
    request: Request,
    response: Response,
    db: AsyncSession = Depends(get_db),
    user: User = Depends(get_current_user),
):
    """Logout from all devices."""
    count = await revoke_all_user_tokens(db, user.id)

    await log_audit_event(
        db,
        "logout_all",
        "success",
        user.id,
        get_client_ip(request),
        request.headers.get("user-agent"),
        {"revoked_tokens": count},
    )

    # Clear cookies
    cookie_settings = get_cookie_settings()
    response.delete_cookie(key="access_token", **cookie_settings)
    response.delete_cookie(key="refresh_token", **cookie_settings)

    return {"success": True, "message": f"Logged out from all {count} devices"}


# ============== GitHub OAuth Routes ==============


@app.get("/api/auth/github", response_model=GitHubAuthUrl)
async def github_auth_url(request: Request):
    """Get GitHub OAuth authorization URL."""
    if not settings.GITHUB_CLIENT_ID:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="GitHub OAuth not configured",
        )

    state = generate_csrf_token()
    # Store state in cookie for verification
    response = JSONResponse({"success": True, "auth_url": get_github_auth_url(state)})
    response.set_cookie(
        key="oauth_state",
        value=state,
        max_age=600,
        httponly=True,
        secure=settings.COOKIE_SECURE,
        samesite="lax",
    )
    return response


@app.get("/api/auth/github/callback")
async def github_callback(
    request: Request,
    code: str,
    state: str,
    response: Response,
    db: AsyncSession = Depends(get_db),
):
    """Handle GitHub OAuth callback."""
    client_ip = get_client_ip(request)

    # Verify state to prevent CSRF (using constant-time comparison)
    stored_state = request.cookies.get("oauth_state")
    if not stored_state:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST, detail="Missing state cookie"
        )

    if not hmac.compare_digest(stored_state, state):
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST, detail="Invalid state"
        )

    # Exchange code for user data
    user_data = await complete_github_auth(code)
    if not user_data:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="GitHub authentication failed",
        )

    # Get or create user
    try:
        user = await get_or_create_github_user(
            db,
            github_id=str(user_data["id"]),
            github_username=user_data["login"],
            email=user_data.get("email"),
            avatar_url=user_data.get("avatar_url"),
        )

        # Create tokens
        access_token, refresh_token = await create_user_tokens(
            db, user, ip_address=client_ip, user_agent=request.headers.get("user-agent")
        )

        # Log success
        await log_audit_event(
            db,
            "login",
            "success",
            user.id,
            client_ip,
            request.headers.get("user-agent"),
            {"method": "github", "github_id": user.github_id},
        )

        # Set cookies and redirect
        cookie_settings = get_cookie_settings()
        response.set_cookie(
            key="access_token",
            value=access_token,
            max_age=settings.ACCESS_TOKEN_EXPIRE_MINUTES * 60,
            **cookie_settings,
        )
        response.set_cookie(
            key="refresh_token",
            value=refresh_token,
            max_age=settings.REFRESH_TOKEN_EXPIRE_DAYS * 86400,
            **cookie_settings,
        )
        response.delete_cookie(key="oauth_state")

        # Redirect to frontend
        return RedirectResponse(url=settings.OAUTH_REDIRECT_URL, status_code=302)

    except Exception as e:
        await log_audit_event(
            db,
            "login",
            "failure",
            None,
            client_ip,
            request.headers.get("user-agent"),
            {"method": "github", "error": str(e)},
        )
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail="Authentication failed",
        )


# ============== User Routes ==============


@app.get("/api/me", response_model=UserProfileResponse)
async def get_me(
    user: User = Depends(get_current_user),
    db: AsyncSession = Depends(get_db),
):
    """Get current user profile with progress."""
    progress = await get_user_progress(db, user.id)

    return UserProfileResponse(
        id=user.id,
        email=user.email,
        username=user.username,
        display_name=user.display_name,
        is_active=user.is_active,
        is_verified=user.is_verified,
        created_at=user.created_at,
        last_login=user.last_login,
        github_username=user.github_username,
        github_avatar_url=user.github_avatar_url,
        progress=ProgressResponse(
            xp=progress.xp,
            solved_problems=progress.solved_problems,
            current_streak=progress.current_streak,
            longest_streak=progress.longest_streak,
            last_solved_date=progress.last_solved_date,
        ),
    )


@app.get("/api/stats")
async def get_stats(
    db: AsyncSession = Depends(get_db),
    user: Optional[User] = Depends(optional_user),
):
    """Get user stats - works for both authenticated and anonymous users."""
    if user:
        progress = await get_user_progress(db, user.id)
        return {
            "xp": progress.xp,
            "solved": progress.solved_problems,
            "authenticated": True,
            "username": user.username,
        }
    else:
        # Return empty stats for anonymous users
        return {
            "xp": 0,
            "solved": [],
            "authenticated": False,
        }


# ============== Progress Routes ==============


@app.post("/api/progress/solve")
@limiter.limit(settings.RATE_LIMIT_API)
async def solve_problem(
    request: Request,
    submission: ProblemSubmission,
    db: AsyncSession = Depends(get_db),
    user: User = Depends(get_current_user),
):
    """Mark a problem as solved and update progress."""
    progress = await update_user_progress(
        db, user.id, submission.problem_id, xp_gained=10, code=submission.code
    )

    return {
        "success": True,
        "xp": progress.xp,
        "solved_count": len(progress.solved_problems),
        "newly_solved": submission.problem_id in progress.solved_problems,
    }


@app.get("/api/progress")
async def get_progress(
    db: AsyncSession = Depends(get_db),
    user: User = Depends(get_current_user),
):
    """Get detailed progress."""
    progress = await get_user_progress(db, user.id)
    return ProgressResponse(
        xp=progress.xp,
        solved_problems=progress.solved_problems,
        current_streak=progress.current_streak,
        longest_streak=progress.longest_streak,
        last_solved_date=progress.last_solved_date,
    )


# ============== CSRF Token Route ==============


@app.get("/api/csrf")
@limiter.limit("30/minute")  # SECURITY FIX: Rate limit CSRF token generation (H-05)
async def get_csrf_token(request: Request):
    """Get a CSRF token for state-changing operations."""
    return {"csrf_token": generate_csrf_token()}


# ============== Health Check ==============


@app.get("/api/health")
async def health_check():
    """Health check endpoint."""
    return {
        "status": "healthy",
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "environment": settings.ENVIRONMENT,
        "version": "3.0.0",
    }


# ============== Extended Health Check ==============


@app.get("/api/health/detailed")
async def health_check_detailed(
    admin: User = Depends(require_admin),  # SECURITY FIX: Require admin auth (H-01)
):
    """Detailed health check with service status."""
    from app.database import engine

    # Check database
    db_status = "healthy"
    try:
        async with engine.connect() as conn:
            await conn.execute("SELECT 1")
    except Exception as e:
        # SECURITY FIX: Don't expose detailed error messages (H-02)
        db_status = "unhealthy"

    # Check cache
    cache_status = "healthy" if settings.CACHE_ENABLED else "disabled"

    return {
        "status": "healthy" if db_status == "healthy" else "degraded",
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "environment": settings.ENVIRONMENT,
        "version": "3.0.0",
        "services": {
            "database": db_status,
            "cache": cache_status,
            "email": settings.EMAIL_BACKEND,
        },
    }


# ============== Routers ==============

# School Platform
app.include_router(school_profile.router)
app.include_router(school_classroom.router)
app.include_router(school_assignments.router)
app.include_router(gdpr.router)  # GDPR compliance endpoints
app.include_router(api_v4.router, prefix="/api")  # v4 API (alpha)

# Extended Auth (password reset, email verification)
app.include_router(auth_extended.router)

# Notifications
from app.notification_service import router as notification_router

app.include_router(notification_router)

# WebSocket
app.include_router(ws_router)

# Admin
app.include_router(admin.router)


# ============== Error Handlers ==============


@app.exception_handler(HTTPException)
async def http_exception_handler(request: Request, exc: HTTPException):
    """Custom HTTP exception handler."""
    return JSONResponse(
        status_code=exc.status_code,
        content=ErrorResponse(
            success=False,
            message=exc.detail,
        ).model_dump(),
    )


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8000)
