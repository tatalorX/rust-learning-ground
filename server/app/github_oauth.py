# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
GitHub OAuth integration.
"""
import secrets
from typing import Optional, Dict, Any

import httpx

from app.config import get_settings

settings = get_settings()

# GitHub OAuth endpoints
GITHUB_AUTH_URL = "https://github.com/login/oauth/authorize"
GITHUB_TOKEN_URL = "https://github.com/login/oauth/access_token"
GITHUB_USER_URL = "https://api.github.com/user"
GITHUB_EMAILS_URL = "https://api.github.com/user/emails"


def get_github_auth_url(state: str) -> str:
    """Generate GitHub OAuth authorization URL."""
    if not settings.GITHUB_CLIENT_ID:
        raise ValueError("GitHub OAuth not configured")
    
    params = {
        "client_id": settings.GITHUB_CLIENT_ID,
        "redirect_uri": None,  # Use default from GitHub app settings
        "scope": "read:user user:email",
        "state": state,
    }
    
    # Build URL manually to handle None values
    url = f"{GITHUB_AUTH_URL}?client_id={settings.GITHUB_CLIENT_ID}&scope=read:user+user:email&state={state}"
    return url


async def exchange_code_for_token(code: str) -> Optional[str]:
    """Exchange OAuth code for access token."""
    if not settings.GITHUB_CLIENT_ID or not settings.GITHUB_CLIENT_SECRET:
        return None
    
    async with httpx.AsyncClient() as client:
        response = await client.post(
            GITHUB_TOKEN_URL,
            headers={"Accept": "application/json"},
            data={
                "client_id": settings.GITHUB_CLIENT_ID,
                "client_secret": settings.GITHUB_CLIENT_SECRET,
                "code": code,
            },
        )
        
        if response.status_code != 200:
            return None
        
        data = response.json()
        return data.get("access_token")


async def get_github_user(access_token: str) -> Optional[Dict[str, Any]]:
    """Get GitHub user info using access token."""
    async with httpx.AsyncClient() as client:
        # Get user info
        user_response = await client.get(
            GITHUB_USER_URL,
            headers={
                "Authorization": f"Bearer {access_token}",
                "Accept": "application/vnd.github.v3+json",
            },
        )
        
        if user_response.status_code != 200:
            return None
        
        user_data = user_response.json()
        
        # Get primary email (may be private)
        email_response = await client.get(
            GITHUB_EMAILS_URL,
            headers={
                "Authorization": f"Bearer {access_token}",
                "Accept": "application/vnd.github.v3+json",
            },
        )
        
        if email_response.status_code == 200:
            emails = email_response.json()
            # Find primary verified email
            for email_info in emails:
                if email_info.get("primary") and email_info.get("verified"):
                    user_data["email"] = email_info.get("email")
                    break
            # Fallback to any email
            if "email" not in user_data or not user_data["email"]:
                for email_info in emails:
                    if email_info.get("verified"):
                        user_data["email"] = email_info.get("email")
                        break
        
        return user_data


async def complete_github_auth(code: str) -> Optional[Dict[str, Any]]:
    """
    Complete GitHub OAuth flow.
    Returns user data or None on failure.
    """
    # Exchange code for token
    access_token = await exchange_code_for_token(code)
    if not access_token:
        return None
    
    # Get user info
    user_data = await get_github_user(access_token)
    return user_data
