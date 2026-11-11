from __future__ import annotations

from functools import lru_cache
from pathlib import Path

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict

from tuvi_api.auth.settings import AuthSettings
from tuvi_api.chat.storage.settings import MongoConversationHistorySettings


class ApiSettings(BaseSettings):
    model_config = SettingsConfigDict(
        env_nested_delimiter="__",
        env_file=".env",
        extra="ignore",
    )

    conversation_history_store: MongoConversationHistorySettings = Field(
        default_factory=MongoConversationHistorySettings
    )
    auth: AuthSettings = Field(default_factory=AuthSettings)
    # Browser origins allowed to call the API directly. Not needed when the web
    # app proxies /api to this server (the default setup).
    cors_origins: list[str] = Field(default_factory=lambda: ["http://localhost:3000"])
    agent_model: str = "openrouter:qwen/qwen3.7-max"
    # The book files are not in git. Relative paths resolve from the working directory.
    book_root: Path = Path("data/tuvitanbien_chunking_compact/part_2")


@lru_cache
def get_settings() -> ApiSettings:
    return ApiSettings()
