from __future__ import annotations

from pydantic import BaseModel, Field

DEV_SECRET_KEY = "dev-only-secret-change-me"


class AuthSettings(BaseModel):
    # Signs the anonymous owner cookie. Set AUTH__SECRET_KEY in every deployed
    # environment; the default is only for local development.
    secret_key: str = DEV_SECRET_KEY
    # Public URL of the web app, as the browser sees it. The API is served under
    # the same origin through the web app's /api rewrite.
    public_url: str = "http://localhost:3000"
    session_days: int = Field(default=30, gt=0)
    google_client_id: str | None = None
    google_client_secret: str | None = None

    @property
    def cookie_secure(self) -> bool:
        return self.public_url.startswith("https://")

    @property
    def google_enabled(self) -> bool:
        return bool(self.google_client_id and self.google_client_secret)

    @property
    def google_redirect_uri(self) -> str:
        return f"{self.public_url.rstrip('/')}/api/v1/auth/google/callback"
