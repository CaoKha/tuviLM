from __future__ import annotations

from datetime import UTC, datetime

from beanie import Document, PydanticObjectId
from bson.errors import InvalidId
from pydantic import Field
from pymongo import ASCENDING, IndexModel
from pymongo.errors import DuplicateKeyError

from tuvi_api.auth.models import EmailTakenError, User


def _utc_now() -> datetime:
    # MongoDB stores milliseconds. Round here so a user returned from create_user
    # equals the same user read back later.
    now = datetime.now(UTC)
    return now.replace(microsecond=(now.microsecond // 1000) * 1000)


class UserDocument(Document):
    email: str
    display_name: str | None = None
    password_hash: str | None = None
    google_sub: str | None = None
    created_at: datetime = Field(default_factory=_utc_now)

    class Settings:
        name = "users"
        indexes = [
            IndexModel([("email", ASCENDING)], unique=True),
            IndexModel(
                [("google_sub", ASCENDING)],
                unique=True,
                partialFilterExpression={"google_sub": {"$type": "string"}},
            ),
        ]


class AuthSessionDocument(Document):
    user_id: str
    token_hash: str
    created_at: datetime = Field(default_factory=_utc_now)
    expires_at: datetime

    class Settings:
        name = "auth_sessions"
        indexes = [
            IndexModel([("token_hash", ASCENDING)], unique=True),
            # MongoDB deletes the document once expires_at has passed.
            IndexModel([("expires_at", ASCENDING)], expireAfterSeconds=0),
        ]


AUTH_DOCUMENT_MODELS = [UserDocument, AuthSessionDocument]


def _user(document: UserDocument) -> User:
    return User(
        id=str(document.id),
        email=document.email,
        display_name=document.display_name,
        created_at=document.created_at,
    )


def normalize_email(email: str) -> str:
    return email.strip().lower()


class MongoAuthStore:
    """Auth storage on Beanie. Needs init_beanie with AUTH_DOCUMENT_MODELS first."""

    async def create_user(
        self,
        *,
        email: str,
        password_hash: str | None,
        display_name: str | None,
    ) -> User:
        document = UserDocument(
            email=normalize_email(email),
            password_hash=password_hash,
            display_name=display_name,
        )
        try:
            await document.insert()
        except DuplicateKeyError as exc:
            raise EmailTakenError(email) from exc
        return _user(document)

    async def get_user(self, user_id: str) -> User | None:
        try:
            object_id = PydanticObjectId(user_id)
        except (InvalidId, TypeError):
            return None
        document = await UserDocument.get(object_id)
        return _user(document) if document else None

    async def get_password_hash(self, email: str) -> tuple[User, str] | None:
        document = await UserDocument.find_one(UserDocument.email == normalize_email(email))
        if document is None or document.password_hash is None:
            return None
        return _user(document), document.password_hash

    async def upsert_google_user(
        self,
        *,
        google_sub: str,
        email: str,
        display_name: str | None,
    ) -> User:
        document = await UserDocument.find_one(UserDocument.google_sub == google_sub)
        if document is None:
            # Link to an existing password account with the same verified email.
            document = await UserDocument.find_one(
                UserDocument.email == normalize_email(email)
            )
        if document is None:
            document = UserDocument(
                email=normalize_email(email),
                display_name=display_name,
                google_sub=google_sub,
            )
            await document.insert()
            return _user(document)
        document.google_sub = google_sub
        if document.display_name is None:
            document.display_name = display_name
        await document.save()
        return _user(document)

    async def create_session(
        self,
        *,
        user_id: str,
        token_hash: str,
        expires_at: datetime,
    ) -> None:
        await AuthSessionDocument(
            user_id=user_id,
            token_hash=token_hash,
            expires_at=expires_at,
        ).insert()

    async def get_session_user(self, token_hash: str, *, now: datetime) -> User | None:
        session = await AuthSessionDocument.find_one(
            AuthSessionDocument.token_hash == token_hash
        )
        # The TTL index removes expired sessions only about once a minute.
        if session is None or _aware(session.expires_at) <= now:
            return None
        return await self.get_user(session.user_id)

    async def delete_session(self, token_hash: str) -> None:
        await AuthSessionDocument.find(
            AuthSessionDocument.token_hash == token_hash
        ).delete()


def _aware(value: datetime) -> datetime:
    return value if value.tzinfo else value.replace(tzinfo=UTC)
