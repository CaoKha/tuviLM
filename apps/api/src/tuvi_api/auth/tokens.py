"""Signed anonymous owner ids."""

from __future__ import annotations

import hashlib
import hmac
import secrets

ANONYMOUS_OWNER_PREFIX = "anon_"


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
