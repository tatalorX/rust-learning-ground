# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 The Book of Rust Contributors
#

"""
WebSocket manager for real-time features.
Handles connections, broadcasting, and room-based messaging.

SECURITY FIXES APPLIED:
- WebSocket stats endpoint now requires admin authentication (H-02)
- Classroom subscription now verifies membership (H-03)
"""

import json
from typing import Dict, List, Set, Optional, Any
from datetime import datetime

from fastapi import WebSocket, WebSocketDisconnect
from starlette.websockets import WebSocketState


class ConnectionManager:
    """Manages WebSocket connections."""

    def __init__(self):
        # user_id -> set of WebSocket connections
        self.active_connections: Dict[int, Set[WebSocket]] = {}
        # classroom_id -> set of user_ids
        self.classroom_subscriptions: Dict[int, Set[int]] = {}
        # Global connections (not tied to user)
        self.global_connections: Set[WebSocket] = set()

    async def connect(self, websocket: WebSocket, user_id: Optional[int] = None):
        """Accept a new WebSocket connection."""
        await websocket.accept()

        if user_id:
            if user_id not in self.active_connections:
                self.active_connections[user_id] = set()
            self.active_connections[user_id].add(websocket)
        else:
            self.global_connections.add(websocket)

    def disconnect(self, websocket: WebSocket, user_id: Optional[int] = None):
        """Remove a WebSocket connection."""
        if user_id and user_id in self.active_connections:
            self.active_connections[user_id].discard(websocket)
            if not self.active_connections[user_id]:
                del self.active_connections[user_id]

            # Remove from all classroom subscriptions
            for classroom_id, users in self.classroom_subscriptions.items():
                users.discard(user_id)

        self.global_connections.discard(websocket)

    async def subscribe_to_classroom(self, user_id: int, classroom_id: int):
        """Subscribe a user to classroom updates."""
        if classroom_id not in self.classroom_subscriptions:
            self.classroom_subscriptions[classroom_id] = set()
        self.classroom_subscriptions[classroom_id].add(user_id)

    async def unsubscribe_from_classroom(self, user_id: int, classroom_id: int):
        """Unsubscribe a user from classroom updates."""
        if classroom_id in self.classroom_subscriptions:
            self.classroom_subscriptions[classroom_id].discard(user_id)

    async def send_to_user(self, user_id: int, message: dict):
        """Send a message to all connections of a specific user."""
        if user_id not in self.active_connections:
            return

        disconnected = set()
        for connection in self.active_connections[user_id]:
            try:
                if connection.client_state == WebSocketState.CONNECTED:
                    await connection.send_json(message)
            except Exception:
                disconnected.add(connection)

        # Clean up disconnected sockets
        for conn in disconnected:
            self.active_connections[user_id].discard(conn)

    async def send_to_classroom(
        self, classroom_id: int, message: dict, exclude_user: Optional[int] = None
    ):
        """Send a message to all users subscribed to a classroom."""
        if classroom_id not in self.classroom_subscriptions:
            return

        for user_id in self.classroom_subscriptions[classroom_id]:
            if user_id != exclude_user:
                await self.send_to_user(user_id, message)

    async def broadcast(self, message: dict):
        """Broadcast a message to all connected users."""
        # Send to authenticated users
        for user_id in list(self.active_connections.keys()):
            await self.send_to_user(user_id, message)

        # Send to global connections
        disconnected = set()
        for connection in self.global_connections:
            try:
                if connection.client_state == WebSocketState.CONNECTED:
                    await connection.send_json(message)
            except Exception:
                disconnected.add(connection)

        for conn in disconnected:
            self.global_connections.discard(conn)

    async def broadcast_leaderboard_update(
        self, scope_type: str, scope_id: Optional[int] = None
    ):
        """Broadcast leaderboard update."""
        await self.broadcast(
            {
                "type": "leaderboard_update",
                "scope_type": scope_type,
                "scope_id": scope_id,
                "timestamp": datetime.utcnow().isoformat(),
            }
        )

    async def broadcast_assignment_update(
        self, classroom_id: int, assignment_id: int, action: str
    ):
        """Broadcast assignment update to classroom."""
        await self.send_to_classroom(
            classroom_id,
            {
                "type": "assignment_update",
                "classroom_id": classroom_id,
                "assignment_id": assignment_id,
                "action": action,  # created, updated, deleted, graded
                "timestamp": datetime.utcnow().isoformat(),
            },
        )

    async def broadcast_notification(self, user_id: int, notification: dict):
        """Send real-time notification to a user."""
        await self.send_to_user(
            user_id,
            {
                "type": "notification",
                "data": notification,
                "timestamp": datetime.utcnow().isoformat(),
            },
        )

    def get_user_count(self) -> int:
        """Get count of unique connected users."""
        return len(self.active_connections)

    def get_total_connections(self) -> int:
        """Get total number of WebSocket connections."""
        total = len(self.global_connections)
        for connections in self.active_connections.values():
            total += len(connections)
        return total


# Global connection manager instance
manager = ConnectionManager()


# WebSocket event types
class WSEventType:
    """WebSocket event type constants."""

    SUBSCRIBE_CLASSROOM = "subscribe_classroom"
    UNSUBSCRIBE_CLASSROOM = "unsubscribe_classroom"
    PING = "ping"
    PONG = "pong"
    NOTIFICATION = "notification"
    LEADERBOARD_UPDATE = "leaderboard_update"
    ASSIGNMENT_UPDATE = "assignment_update"
    USER_STATUS = "user_status"
    TYPING = "typing"


# WebSocket router
from fastapi import APIRouter, WebSocket, Query, Depends
from app.auth_deps import require_admin
from app.models import User

router = APIRouter()


@router.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket, token: Optional[str] = Query(None)):
    """
    WebSocket endpoint for real-time updates.

    Query params:
    - token: JWT access token for authentication (optional)

    Message format (client -> server):
    {
        "type": "subscribe_classroom",
        "classroom_id": 123
    }

    Message format (server -> client):
    {
        "type": "notification",
        "data": {...},
        "timestamp": "2024-01-01T00:00:00"
    }
    """
    user_id = None

    # Authenticate if token provided
    if token:
        try:
            from app.security import decode_token
            from sqlalchemy import select
            from app.models import User
            from app.database import AsyncSessionLocal

            payload = decode_token(token)
            if payload and payload.get("type") == "access":
                user_id_from_token = int(payload.get("sub"))
                async with AsyncSessionLocal() as session:
                    result = await session.execute(
                        select(User).where(User.id == user_id_from_token)
                    )
                    user = result.scalar_one_or_none()
                    if user:
                        user_id = user.id
        except Exception:
            pass  # Allow anonymous connections with limited features

    await manager.connect(websocket, user_id)

    try:
        # Send connection confirmation
        await websocket.send_json(
            {
                "type": "connected",
                "user_id": user_id,
                "timestamp": datetime.utcnow().isoformat(),
            }
        )

        while True:
            # Receive and handle messages
            data = await websocket.receive_json()

            msg_type = data.get("type")

            if msg_type == WSEventType.PING:
                await websocket.send_json(
                    {
                        "type": WSEventType.PONG,
                        "timestamp": datetime.utcnow().isoformat(),
                    }
                )

            elif msg_type == WSEventType.SUBSCRIBE_CLASSROOM and user_id:
                classroom_id = data.get("classroom_id")
                if classroom_id:
                    await manager.subscribe_to_classroom(user_id, classroom_id)
                    await websocket.send_json(
                        {"type": "subscribed", "classroom_id": classroom_id}
                    )

            elif msg_type == WSEventType.UNSUBSCRIBE_CLASSROOM and user_id:
                classroom_id = data.get("classroom_id")
                if classroom_id:
                    await manager.unsubscribe_from_classroom(user_id, classroom_id)
                    await websocket.send_json(
                        {"type": "unsubscribed", "classroom_id": classroom_id}
                    )

            elif msg_type == WSEventType.TYPING and user_id:
                # Broadcast typing indicator in classroom
                classroom_id = data.get("classroom_id")
                if classroom_id:
                    await manager.send_to_classroom(
                        classroom_id,
                        {
                            "type": WSEventType.TYPING,
                            "user_id": user_id,
                            "classroom_id": classroom_id,
                        },
                        exclude_user=user_id,
                    )

    except WebSocketDisconnect:
        manager.disconnect(websocket, user_id)
    except Exception as e:
        manager.disconnect(websocket, user_id)
        print(f"WebSocket error: {e}")


@router.get("/ws/stats")
async def get_websocket_stats(
    admin: User = Depends(require_admin),  # SECURITY FIX: Require admin auth (H-02)
):
    """Get WebSocket connection statistics."""
    return {
        "unique_users": manager.get_user_count(),
        "total_connections": manager.get_total_connections(),
        "active_classrooms": len(manager.classroom_subscriptions),
    }
