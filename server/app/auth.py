# SPDX-License-Identifier: MIT
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
Authentication exports for convenient imports.
"""
from app.auth_deps import get_current_user, optional_user

__all__ = ["get_current_user", "optional_user"]
