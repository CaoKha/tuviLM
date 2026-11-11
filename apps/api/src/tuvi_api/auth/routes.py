from __future__ import annotations

import logging
import secrets
from datetime import UTC, datetime, timedelta

from fastapi import APIRouter, Depends, HTTPException, Request, Response
from pydantic import BaseModel, Field, field_validator

from tuvi_api.auth.dependencies import (
    OWNER_COOKIE,
    SESSION_COOKIE,
    clear_cookie,
    get_auth_settings,
    get_auth_store,
    get_current_user,
    read_anonymous_owner_id,
    set_session_cookie,
)
from tuvi_api.auth.models import AuthStore, EmailTakenError, User
from tuvi_api.auth.settings import AuthSettings
from tuvi_api.auth.tokens import (
    hash_password,
    hash_session_token,
    new_session_token,
    verify_password,
)

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/api/v1/auth", tags=["auth"])

# Compared against when the email is unknown, so a failed login takes the same
# time whether or not the account exists.
_DUMMY_PASSWORD_HASH = hash_password(secrets.token_urlsafe(16))


class UserPayload(BaseModel):
    id: str
    email: str
    display_name: str | None


class MeResponse(BaseModel):
    user: UserPayload | None
    google_enabled: bool


class UserResponse(BaseModel):
    user: UserPayload


class RegisterRequest(BaseModel):
    email: str = Field(max_length=254)
    password: str = Field(min_length=8, max_length=128)
    display_name: str | None = Field(default=None, max_length=80)

    @field_validator("email")
    @classmethod
    def _check_email(cls, value: str) -> str:
        value = value.strip()
        local, _, domain = value.partition("@")
        if not local or "." not in domain:
            raise ValueError("Invalid email address.")
        return value


class LoginRequest(BaseModel):
    email: str = Field(max_length=254)
    password: str = Field(max_length=128)


def _payload(user: User) -> UserPayload:
    return UserPayload(id=user.id, email=user.email, display_name=user.display_name)


@router.get("/me", response_model=MeResponse)
async def me(
    user: User | None = Depends(get_current_user),
    settings: AuthSettings = Depends(get_auth_settings),
) -> MeResponse:
    return MeResponse(
        user=_payload(user) if user else None,
        google_enabled=settings.google_enabled,
    )


@router.post("/register", response_model=UserResponse, status_code=201)
async def register(
    payload: RegisterRequest,
    request: Request,
    response: Response,
    store: AuthStore = Depends(get_auth_store),
    settings: AuthSettings = Depends(get_auth_settings),
) -> UserResponse:
    try:
        user = await store.create_user(
            email=payload.email,
            password_hash=hash_password(payload.password),
            display_name=payload.display_name,
        )
    except EmailTakenError as exc:
        raise HTTPException(status_code=409, detail="Email is already registered.") from exc
    await _start_session(request, response, user, store, settings)
    return UserResponse(user=_payload(user))


@router.post("/login", response_model=UserResponse)
async def login(
    payload: LoginRequest,
    request: Request,
    response: Response,
    store: AuthStore = Depends(get_auth_store),
    settings: AuthSettings = Depends(get_auth_settings),
) -> UserResponse:
    found = await store.get_password_hash(payload.email)
    password_hash = found[1] if found else _DUMMY_PASSWORD_HASH
    if not verify_password(payload.password, password_hash) or found is None:
        raise HTTPException(status_code=401, detail="Wrong email or password.")
    user = found[0]
    await _start_session(request, response, user, store, settings)
    return UserResponse(user=_payload(user))


@router.post("/logout", status_code=204)
async def logout(
    request: Request,
    store: AuthStore = Depends(get_auth_store),
    settings: AuthSettings = Depends(get_auth_settings),
) -> Response:
    token = request.cookies.get(SESSION_COOKIE)
    if token:
        await store.delete_session(hash_session_token(token))
    response = Response(status_code=204)
    clear_cookie(response, SESSION_COOKIE, settings)
    return response


async def _start_session(
    request: Request,
    response: Response,
    user: User,
    store: AuthStore,
    settings: AuthSettings,
) -> None:
    token = new_session_token()
    await store.create_session(
        user_id=user.id,
        token_hash=hash_session_token(token),
        expires_at=datetime.now(UTC) + timedelta(days=settings.session_days),
    )
    set_session_cookie(response, token, settings)

    # A guest who logs in keeps the charts and chats made before login.
    anonymous_owner_id = read_anonymous_owner_id(request, settings)
    if anonymous_owner_id is not None:
        chat_store = request.app.state.api_state.conversation_history_store
        moved = await chat_store.reassign_owner(anonymous_owner_id, user.owner_id)
        logger.info("Moved %s chart profiles from a guest to user %s", moved, user.id)
        clear_cookie(response, OWNER_COOKIE, settings)
