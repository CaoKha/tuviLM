"""Who is calling: an anonymous owner kept in a signed cookie."""

from __future__ import annotations

from fastapi import Depends, Request, Response

from tuvi_api.auth.settings import AuthSettings
from tuvi_api.auth.tokens import new_anonymous_owner_id, sign_owner_id, unsign_owner_id
from tuvi_api.settings import get_settings

OWNER_COOKIE = "tuvi_owner"

_OWNER_COOKIE_MAX_AGE = 365 * 24 * 3600


def get_auth_settings() -> AuthSettings:
    return get_settings().auth


def read_anonymous_owner_id(request: Request, settings: AuthSettings) -> str | None:
    raw = request.cookies.get(OWNER_COOKIE)
    return unsign_owner_id(raw, settings.secret_key) if raw else None


async def get_owner_id(
    request: Request,
    response: Response,
    settings: AuthSettings = Depends(get_auth_settings),
) -> str:
    """Owner of chart profiles and chats. A guest without a cookie gets a new one."""
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
