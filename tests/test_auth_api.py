from __future__ import annotations

from datetime import UTC, datetime, timedelta
from uuid import uuid4

import httpx
import pytest

from tests.test_conversation_history_api import FakeConversationHistoryStore
from tuvi_api.auth.dependencies import OWNER_COOKIE, SESSION_COOKIE
from tuvi_api.auth.models import EmailTakenError, User
from tuvi_api.auth.settings import AuthSettings
from tuvi_api.auth.tokens import sign_owner_id, unsign_owner_id
from tuvi_api.main import ApiState, app
from tuvi_api.settings import get_settings
from tuvi_agent.deps import TuviAgentDeps


pytestmark = pytest.mark.anyio


@pytest.fixture(scope="session")
def anyio_backend() -> str:
    return "asyncio"


class FakeAuthStore:
    def __init__(self) -> None:
        self.users: dict[str, tuple[User, str | None, str | None]] = {}
        self.sessions: dict[str, tuple[str, datetime]] = {}

    async def create_user(self, *, email, password_hash, display_name) -> User:
        email = email.strip().lower()
        if any(user.email == email for user, _, _ in self.users.values()):
            raise EmailTakenError(email)
        user = User(
            id=uuid4().hex,
            email=email,
            display_name=display_name,
            created_at=datetime.now(UTC),
        )
        self.users[user.id] = (user, password_hash, None)
        return user

    async def get_user(self, user_id):
        entry = self.users.get(user_id)
        return entry[0] if entry else None

    async def get_password_hash(self, email):
        for user, password_hash, _ in self.users.values():
            if user.email == email.strip().lower() and password_hash:
                return user, password_hash
        return None

    async def upsert_google_user(self, *, google_sub, email, display_name) -> User:
        for user, password_hash, sub in self.users.values():
            if sub == google_sub or user.email == email.lower():
                self.users[user.id] = (user, password_hash, google_sub)
                return user
        user = await self.create_user(email=email, password_hash=None, display_name=display_name)
        self.users[user.id] = (user, None, google_sub)
        return user

    async def create_session(self, *, user_id, token_hash, expires_at) -> None:
        self.sessions[token_hash] = (user_id, expires_at)

    async def get_session_user(self, token_hash, *, now):
        entry = self.sessions.get(token_hash)
        if entry is None or entry[1] <= now:
            return None
        return await self.get_user(entry[0])

    async def delete_session(self, token_hash) -> None:
        self.sessions.pop(token_hash, None)


@pytest.fixture()
def stores():
    chat_store = FakeConversationHistoryStore()
    auth_store = FakeAuthStore()
    previous = getattr(app.state, "api_state", None)
    app.state.api_state = ApiState(
        agent_deps=TuviAgentDeps(book_root=""),
        conversation_history_store=chat_store,
        auth_store=auth_store,
    )
    try:
        yield chat_store, auth_store
    finally:
        if previous is not None:
            app.state.api_state = previous
        else:
            del app.state.api_state


@pytest.fixture()
async def client():
    transport = httpx.ASGITransport(app=app)
    async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as http:
        yield http


async def _register(client, email="me@example.com", password="correct horse"):
    return await client.post(
        "/api/v1/auth/register",
        json={"email": email, "password": password, "display_name": "Me"},
    )


async def test_me_is_empty_for_a_guest(client, stores) -> None:
    response = await client.get("/api/v1/auth/me")

    assert response.status_code == 200
    assert response.json() == {"user": None, "google_enabled": False}


async def test_register_starts_a_session(client, stores) -> None:
    response = await _register(client)
    me = await client.get("/api/v1/auth/me")

    assert response.status_code == 201
    assert response.json()["user"]["email"] == "me@example.com"
    assert SESSION_COOKIE in response.cookies
    assert "httponly" in response.headers["set-cookie"].lower()
    assert me.json()["user"]["email"] == "me@example.com"


async def test_register_rejects_a_taken_email_and_a_short_password(client, stores) -> None:
    await _register(client)

    taken = await _register(client, email="ME@example.com")
    short = await _register(client, email="other@example.com", password="short")
    bad_email = await _register(client, email="not-an-email")

    assert taken.status_code == 409
    assert short.status_code == 422
    assert bad_email.status_code == 422


async def test_login_checks_the_password(client, stores) -> None:
    await _register(client)
    client.cookies.clear()

    wrong = await client.post(
        "/api/v1/auth/login", json={"email": "me@example.com", "password": "wrong password"}
    )
    unknown = await client.post(
        "/api/v1/auth/login", json={"email": "nobody@example.com", "password": "whatever1"}
    )
    right = await client.post(
        "/api/v1/auth/login", json={"email": "Me@Example.com", "password": "correct horse"}
    )

    assert wrong.status_code == 401
    assert unknown.status_code == 401
    assert wrong.json() == unknown.json()
    assert right.status_code == 200
    assert (await client.get("/api/v1/auth/me")).json()["user"]["email"] == "me@example.com"


async def test_logout_ends_the_session(client, stores) -> None:
    _, auth_store = stores
    await _register(client)

    response = await client.post("/api/v1/auth/logout")

    assert response.status_code == 204
    assert auth_store.sessions == {}
    assert (await client.get("/api/v1/auth/me")).json()["user"] is None


async def test_expired_session_is_ignored(client, stores) -> None:
    _, auth_store = stores
    await _register(client)
    for token_hash, (user_id, _) in list(auth_store.sessions.items()):
        auth_store.sessions[token_hash] = (user_id, datetime.now(UTC) - timedelta(seconds=1))

    assert (await client.get("/api/v1/auth/me")).json()["user"] is None


async def test_guest_charts_move_to_the_account_on_register(client, stores) -> None:
    chat_store, _ = stores
    secret = get_settings().auth.secret_key
    client.cookies.set(OWNER_COOKIE, sign_owner_id("anon_guest", secret))
    chat_store.add_profile(owner_id="anon_guest", display_name="Made as guest")

    response = await _register(client)
    profiles = await client.get("/api/v1/chart-profiles")

    user_id = response.json()["user"]["id"]
    assert [p["display_name"] for p in profiles.json()["chart_profiles"]] == ["Made as guest"]
    assert all(p.owner_id == f"user_{user_id}" for p in chat_store.profiles.values())
    # The guest cookie is cleared once its data belongs to the account.
    assert f'{OWNER_COOKIE}=""' in response.headers.get_list("set-cookie")[-1]


async def test_logged_in_user_does_not_see_guest_data_of_others(client, stores) -> None:
    chat_store, _ = stores
    chat_store.add_profile(owner_id="anon_someone_else")
    await _register(client)

    profiles = await client.get("/api/v1/chart-profiles")

    assert profiles.json()["chart_profiles"] == []


def test_owner_cookie_signature() -> None:
    signed = sign_owner_id("anon_abc", "secret")

    assert unsign_owner_id(signed, "secret") == "anon_abc"
    assert unsign_owner_id(signed, "other-secret") is None
    assert unsign_owner_id("anon_abc.forged", "secret") is None
    # A client cannot claim a user's owner id through the guest cookie.
    assert unsign_owner_id(sign_owner_id("user_1", "secret"), "secret") is None


def test_secure_cookies_follow_the_public_url() -> None:
    assert AuthSettings(public_url="https://demo.example.com").cookie_secure is True
    assert AuthSettings(public_url="http://localhost:3000").cookie_secure is False


@pytest.fixture()
async def mongo_auth_store():
    import os

    from beanie import init_beanie
    from pymongo import AsyncMongoClient
    from pymongo.errors import ServerSelectionTimeoutError

    from tuvi_api.auth.storage import AUTH_DOCUMENT_MODELS, MongoAuthStore

    uri = os.environ.get("CONVERSATION_HISTORY_STORE__URI", "mongodb://localhost:27017")
    mongo = AsyncMongoClient(uri, serverSelectionTimeoutMS=300, tz_aware=True)
    try:
        await mongo.admin.command("ping")
    except ServerSelectionTimeoutError as exc:
        await mongo.close()
        pytest.skip(f"MongoDB is not available at {uri}: {exc}")
    database = mongo["tuvilm_auth_test"]
    for model in AUTH_DOCUMENT_MODELS:
        await database.drop_collection(model.Settings.name)
    await init_beanie(database=database, document_models=AUTH_DOCUMENT_MODELS)
    try:
        yield MongoAuthStore()
    finally:
        await mongo.drop_database("tuvilm_auth_test")
        await mongo.close()


async def test_mongo_auth_store_users_and_sessions(mongo_auth_store) -> None:
    store = mongo_auth_store
    user = await store.create_user(email=" Me@Example.com ", password_hash="h", display_name="Me")

    with pytest.raises(EmailTakenError):
        await store.create_user(email="me@example.com", password_hash="h2", display_name=None)
    assert (await store.get_password_hash("ME@example.com")) == (user, "h")
    assert await store.get_user("not-an-object-id") is None

    now = datetime.now(UTC)
    await store.create_session(user_id=user.id, token_hash="t1", expires_at=now + timedelta(days=1))
    await store.create_session(user_id=user.id, token_hash="t2", expires_at=now - timedelta(seconds=1))
    assert (await store.get_session_user("t1", now=now)) == user
    assert await store.get_session_user("t2", now=now) is None

    await store.delete_session("t1")
    assert await store.get_session_user("t1", now=now) is None


async def test_mongo_auth_store_links_google_to_an_existing_email(mongo_auth_store) -> None:
    store = mongo_auth_store
    user = await store.create_user(email="me@example.com", password_hash="h", display_name=None)

    linked = await store.upsert_google_user(
        google_sub="g-1", email="me@example.com", display_name="Me"
    )
    again = await store.upsert_google_user(
        google_sub="g-1", email="changed@example.com", display_name="Me"
    )
    other = await store.upsert_google_user(
        google_sub="g-2", email="new@example.com", display_name=None
    )

    assert linked.id == user.id and linked.display_name == "Me"
    assert again.id == user.id
    assert other.id != user.id
    assert (await store.get_password_hash("new@example.com")) is None
