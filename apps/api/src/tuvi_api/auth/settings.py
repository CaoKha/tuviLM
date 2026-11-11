from __future__ import annotations

from pydantic import BaseModel

DEV_SECRET_KEY = "dev-only-secret-change-me"


class AuthSettings(BaseModel):
    # Signs the anonymous owner cookie. Set AUTH__SECRET_KEY in every deployed
    # environment; the default is only for local development.
    secret_key: str = DEV_SECRET_KEY
    # Public URL of the web app, as the browser sees it. The API is served under
    # the same origin through the web app's /api rewrite.
    public_url: str = "http://localhost:3000"

    @property
    def cookie_secure(self) -> bool:
        return self.public_url.startswith("https://")
