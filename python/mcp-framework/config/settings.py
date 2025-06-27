from pydantic_settings import BaseSettings, SettingsConfigDict
from pydantic import Field, AnyHttpUrl


class Settings(BaseSettings):
    # Cấu hình cơ bản
    ENV: str = Field("production", description="Runtime environment")
    VERSION: str = Field("1.0.0", description="Application version")
    DEBUG: bool = Field(False, description="Debug mode")
    RELOAD: bool = Field(False, description="Auto-reload on code changes")

    # Cấu hình server
    HOST: str = Field("127.0.0.1", description="Server host")
    PORT: int = Field(8000, description="Server port")
    WORKERS: int = Field(1, description="Number of worker processes")
    LOG_LEVEL: str = Field("info", description="Logging level")
    KEEP_ALIVE_TIMEOUT: int = Field(5, description="Keep-alive timeout")
    PROXY_HEADERS: bool = Field(False, description="Trust proxy headers")

    # Cấu hình bảo mật
    API_KEY: str = Field(..., description="API key for authentication")
    CORS_ORIGINS: list[AnyHttpUrl] = Field([], description="Allowed CORS origins")
    RATE_LIMIT: str = Field("100/minute", description="Global rate limit")

    # Đường dẫn
    PLUGINS_DIR: str = Field("services/custom", description="Custom plugins directory")

    model_config = SettingsConfigDict(
        env_file=".env", env_file_encoding="utf-8", extra="ignore"
    )


# Tạo instance cấu hình
# settings = Settings()
