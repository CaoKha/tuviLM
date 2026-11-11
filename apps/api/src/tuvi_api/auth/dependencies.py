"""Who is calling: a logged-in user, or an anonymous owner kept in a signed cookie."""

from __future__ import annotations

from datetime import UTC, datetime

from fastapi import Depends, Request, Response

from tuvi_api.auth.models import AuthStore, User
from tuvi_api.auth.settings import AuthSettings
from tuvi_api.auth.tokens import (
    hash_session_token,
    new_anonymous_owner_id,
    sign_owner_id,
    unsign_owner_id,
)
from tuvi_api.settings import get_settings

SESSION_COOKIE = "tuvi_session"
OWNER_COOKIE = "tuvi_owner"

_OWNER_COOKIE_MAX_AGE = 365 * 24 * 3600


def get_auth_settings() -> AuthSettings:
    return get_settings().auth


def get_auth_store(request: Request) -> AuthStore:
    return request.app.state.api_state.auth_store


async def get_current_user(
    request: Request,
    store: AuthStore = Depends(get_auth_store),
) -> User | None:
    token = request.cookies.get(SESSION_COOKIE)
    if not token:
        return None
    return await store.get_session_user(hash_session_token(token), now=datetime.now(UTC))


def read_anonymous_owner_id(request: Request, settings: AuthSettings) -> str | None:
    raw = request.cookies.get(OWNER_COOKIE)
    return unsign_owner_id(raw, settings.secret_key) if raw else None


async def get_owner_id(
    request: Request,
    response: Response,
    user: User | None = Depends(get_current_user),
    settings: AuthSettings = Depends(get_auth_settings),
) -> str:
    """Owner of chart profiles and chats. A guest without a cookie gets a new one."""
    if user is not None:
        return user.owner_id
    owner_id = read_anonymous_owner_id(request, settings)
    if owner_id is None:
        owner_id = new_anonymous_owner_id()
        set_owner_cookie(response, owner_id, settings)
    return owner_id


def set_owner_cookie(response: Response, owner_id: str, settings: AuthSettings) -> None:
    response.set_cookie(
        OWNER_COOKIE,
        sign_owner_id(owner_id, settings.secret_key),
        max_age=_OWNER_COOKIE_MAX_AGE,
        httponly=True,
        secure=settings.cookie_secure,
        samesite="lax",
        path="/",
    )


def set_session_cookie(response: Response, token: str, settings: AuthSettings) -> None:
    response.set_cookie(
        SESSION_COOKIE,
        token,
        max_age=settings.session_days * 24 * 3600,
        httponly=True,
        secure=settings.cookie_secure,
        samesite="lax",
        path="/",
    )


def clear_cookie(response: Response, name: str, settings: AuthSettings) -> None:
    response.delete_cookie(
        name,
        httponly=True,
        secure=settings.cookie_secure,
        samesite="lax",
        path="/",
    )
