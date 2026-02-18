# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#
"""
Feature Flags System

Provides kill switches and feature toggles for the application.
Allows disabling features without deploying new code.

Usage:
    from app.feature_flags import feature_flags
    
    if feature_flags.is_enabled("code_execution"):
        run_code()
    else:
        return {"error": "Code execution temporarily disabled"}
"""

import os
from typing import Dict, Any, Optional
from functools import lru_cache
from dataclasses import dataclass, field
from datetime import datetime, timezone
import json
import logging

logger = logging.getLogger(__name__)


@dataclass
class FeatureFlag:
    """Represents a feature flag with its configuration."""
    name: str
    enabled: bool = True
    description: str = ""
    kill_switch: bool = False  # If True, this is a kill switch (safety critical)
    updated_at: Optional[datetime] = field(default_factory=lambda: datetime.now(timezone.utc))
    updated_by: Optional[str] = None
    reason: Optional[str] = None  # Reason for current state


class FeatureFlags:
    """
    Feature flags manager with support for environment variables and runtime updates.
    
    Kill switches (safety critical features):
    - code_execution: Disable all code compilation/execution
    - user_registration: Disable new user registration
    - classroom_creation: Disable creating new classrooms
    - assignment_creation: Disable creating new assignments
    
    Regular features:
    - github_oauth: Enable GitHub OAuth login
    - email_notifications: Send email notifications
    - leaderboard: Show leaderboard features
    - caching: Enable Redis caching
    - websockets: Enable WebSocket connections
    """
    
    DEFAULT_FLAGS = {
        # Kill switches - safety critical
        "code_execution": FeatureFlag(
            name="code_execution",
            enabled=True,
            description="Enable/disable all code compilation and execution",
            kill_switch=True
        ),
        "user_registration": FeatureFlag(
            name="user_registration",
            enabled=True,
            description="Enable/disable new user registration",
            kill_switch=True
        ),
        "classroom_creation": FeatureFlag(
            name="classroom_creation",
            enabled=True,
            description="Enable/disable creating new classrooms",
            kill_switch=True
        ),
        "assignment_creation": FeatureFlag(
            name="assignment_creation",
            enabled=True,
            description="Enable/disable creating new assignments",
            kill_switch=True
        ),
        "login": FeatureFlag(
            name="login",
            enabled=True,
            description="Enable/disable user login",
            kill_switch=True
        ),
        
        # Regular features
        "github_oauth": FeatureFlag(
            name="github_oauth",
            enabled=False,
            description="Enable GitHub OAuth authentication",
            kill_switch=False
        ),
        "email_notifications": FeatureFlag(
            name="email_notifications",
            enabled=False,
            description="Send email notifications",
            kill_switch=False
        ),
        "leaderboard": FeatureFlag(
            name="leaderboard",
            enabled=True,
            description="Show leaderboard features",
            kill_switch=False
        ),
        "caching": FeatureFlag(
            name="caching",
            enabled=False,
            description="Enable Redis caching",
            kill_switch=False
        ),
        "websockets": FeatureFlag(
            name="websockets",
            enabled=False,
            description="Enable WebSocket connections",
            kill_switch=False
        ),
        "public_leaderboard": FeatureFlag(
            name="public_leaderboard",
            enabled=True,
            description="Make leaderboard visible to non-logged-in users",
            kill_switch=False
        ),
    }
    
    def __init__(self):
        self._flags: Dict[str, FeatureFlag] = {}
        self._load_flags()
    
    def _load_flags(self):
        """Load feature flags from defaults and environment."""
        # Start with defaults
        for name, flag in self.DEFAULT_FLAGS.items():
            self._flags[name] = flag
        
        # Override from environment variables
        # Format: FEATURE_CODE_EXECUTION=false
        for name in self._flags:
            env_var = f"FEATURE_{name.upper()}"
            env_value = os.getenv(env_var)
            if env_value is not None:
                enabled = env_value.lower() in ('true', '1', 'yes', 'on')
                self._flags[name].enabled = enabled
                self._flags[name].updated_at = datetime.now(timezone.utc)
                self._flags[name].reason = f"Set via environment variable {env_var}"
                logger.info(f"Feature flag '{name}' set to {enabled} from {env_var}")
        
        # Load from JSON file if exists (for runtime updates)
        self._load_from_file()
    
    def _load_from_file(self):
        """Load feature flags from a JSON file."""
        flags_file = os.getenv("FEATURE_FLAGS_FILE", "feature_flags.json")
        if os.path.exists(flags_file):
            try:
                with open(flags_file, 'r') as f:
                    data = json.load(f)
                
                for name, flag_data in data.items():
                    if name in self._flags:
                        self._flags[name].enabled = flag_data.get('enabled', True)
                        self._flags[name].updated_at = datetime.now(timezone.utc)
                        self._flags[name].reason = flag_data.get('reason', 'Loaded from file')
                        logger.info(f"Feature flag '{name}' loaded from file: {self._flags[name].enabled}")
            except Exception as e:
                logger.error(f"Failed to load feature flags from file: {e}")
    
    def _save_to_file(self):
        """Save current feature flags to a JSON file."""
        flags_file = os.getenv("FEATURE_FLAGS_FILE", "feature_flags.json")
        try:
            data = {
                name: {
                    'enabled': flag.enabled,
                    'updated_at': flag.updated_at.isoformat() if flag.updated_at else None,
                    'updated_by': flag.updated_by,
                    'reason': flag.reason
                }
                for name, flag in self._flags.items()
            }
            with open(flags_file, 'w') as f:
                json.dump(data, f, indent=2)
        except Exception as e:
            logger.error(f"Failed to save feature flags to file: {e}")
    
    def is_enabled(self, name: str) -> bool:
        """Check if a feature flag is enabled."""
        if name not in self._flags:
            logger.warning(f"Unknown feature flag: {name}")
            return True  # Default to enabled for unknown flags
        return self._flags[name].enabled
    
    def is_disabled(self, name: str) -> bool:
        """Check if a feature flag is disabled."""
        return not self.is_enabled(name)
    
    def get_flag(self, name: str) -> Optional[FeatureFlag]:
        """Get the full feature flag configuration."""
        return self._flags.get(name)
    
    def set_flag(self, name: str, enabled: bool, updated_by: str = None, reason: str = None):
        """
        Update a feature flag at runtime.
        Only works for flags loaded from file, not environment variables.
        """
        if name not in self._flags:
            raise ValueError(f"Unknown feature flag: {name}")
        
        flag = self._flags[name]
        old_value = flag.enabled
        flag.enabled = enabled
        flag.updated_at = datetime.now(timezone.utc)
        flag.updated_by = updated_by
        flag.reason = reason
        
        action = "ENABLED" if enabled else "DISABLED"
        if flag.kill_switch:
            action = f"KILLSWITCH_{action}"
        
        logger.warning(
            f"{action}: Feature flag '{name}' changed from {old_value} to {enabled} "
            f"by {updated_by or 'unknown'}: {reason or 'No reason given'}"
        )
        
        self._save_to_file()
    
    def get_all_flags(self) -> Dict[str, FeatureFlag]:
        """Get all feature flags."""
        return dict(self._flags)
    
    def get_kill_switches(self) -> Dict[str, FeatureFlag]:
        """Get all kill switches (safety critical flags)."""
        return {name: flag for name, flag in self._flags.items() if flag.kill_switch}
    
    def check_kill_switches(self) -> list:
        """Check if any kill switches are activated. Returns list of disabled kill switches."""
        disabled = []
        for name, flag in self._flags.items():
            if flag.kill_switch and not flag.enabled:
                disabled.append(flag)
        return disabled
    
    def require_feature(self, name: str, error_message: str = None):
        """
        Decorator for requiring a feature flag to be enabled.
        
        Usage:
            @feature_flags.require_feature("code_execution")
            async def run_code():
                ...
        """
        def decorator(func):
            async def wrapper(*args, **kwargs):
                if self.is_disabled(name):
                    from fastapi import HTTPException, status
                    raise HTTPException(
                        status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
                        detail=error_message or f"Feature '{name}' is currently disabled"
                    )
                return await func(*args, **kwargs)
            return wrapper
        return decorator


# Global instance
feature_flags = FeatureFlags()


# Convenience functions for common checks
def can_execute_code() -> bool:
    """Check if code execution is enabled."""
    return feature_flags.is_enabled("code_execution")


def can_register_users() -> bool:
    """Check if user registration is enabled."""
    return feature_flags.is_enabled("user_registration")


def can_create_classrooms() -> bool:
    """Check if classroom creation is enabled."""
    return feature_flags.is_enabled("classroom_creation")


def can_create_assignments() -> bool:
    """Check if assignment creation is enabled."""
    return feature_flags.is_enabled("assignment_creation")


def can_login() -> bool:
    """Check if login is enabled."""
    return feature_flags.is_enabled("login")
