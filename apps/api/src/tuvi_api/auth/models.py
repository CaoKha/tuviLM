from __future__ import annotations

from datetime import datetime
from typing import Protocol

from pydantic import BaseModel

USER_OWNER_PREFIX = "user_"
ANONYMOUS_OWNER_PREFIX = "anon_"


class User(BaseModel):
    id: str
    email: str
    display_name: str | None = None
    created_at: datetime

    @property
    def owner_id(self) -> str:
        """The owner id that chart profiles and chats use for this account."""
        return f"{USER_OWNER_PREFIX}{self.id}"


class AuthError(Exception):
    """Base class for login and registration errors."""


class EmailTakenError(AuthError):
    """Raised when an account with this email already exists."""


class InvalidCredentialsError(AuthError):
    """Raised when the email or password is wrong."""


class AuthStore(Protocol):
    async def create_user(
        self,
        *,
        email: str,
        password_hash: str | None,
        display_name: str | None,
    ) -> User: ...

    async def get_user(self, user_id: str) -> User | None: ...

    async def get_password_hash(self, email: str) -> tuple[User, str] | None:
        """Return the user and password hash, or None if no password account exists."""
        ...

    async def upsert_google_user(
        self,
        *,
        google_sub: str,
        email: str,
        display_name: str | None,
    ) -> User: ...

    async def create_session(
        self,
        *,
        user_id: str,
        token_hash: str,
        expires_at: datetime,
    ) -> None: ...

    async def get_session_user(self, token_hash: str, *, now: datetime) -> User | None: ...

    async def delete_session(self, token_hash: str) -> None: ...
