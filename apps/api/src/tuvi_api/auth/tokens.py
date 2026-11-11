"""Session tokens, signed anonymous owner ids and password hashing."""

from __future__ import annotations

import hashlib
import hmac
import secrets

from pwdlib import PasswordHash

from tuvi_api.auth.models import ANONYMOUS_OWNER_PREFIX

_password_hash = PasswordHash.recommended()


def hash_password(password: str) -> str:
    return _password_hash.hash(password)


def verify_password(password: str, password_hash: str) -> bool:
    return _password_hash.verify(password, password_hash)


def new_session_token() -> str:
    return secrets.token_urlsafe(32)


def hash_session_token(token: str) -> str:
    """Only the hash is stored, so a database leak does not expose live sessions."""
    return hashlib.sha256(token.encode()).hexdigest()


def new_anonymous_owner_id() -> str:
    return f"{ANONYMOUS_OWNER_PREFIX}{secrets.token_urlsafe(24)}"


def sign_owner_id(owner_id: str, secret_key: str) -> str:
    return f"{owner_id}.{_signature(owner_id, secret_key)}"


def unsign_owner_id(value: str, secret_key: str) -> str | None:
    """Return the anonymous owner id if the signature is valid, else None."""
    owner_id, _, signature = value.rpartition(".")
    if not owner_id.startswith(ANONYMOUS_OWNER_PREFIX):
        return None
    if not hmac.compare_digest(signature, _signature(owner_id, secret_key)):
        return None
    return owner_id


def _signature(owner_id: str, secret_key: str) -> str:
    return hmac.new(secret_key.encode(), owner_id.encode(), hashlib.sha256).hexdigest()
