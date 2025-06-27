#!/usr/bin/env python3
"""
MCP Services Framework - Main Entry Point
"""
import os
import logging
from config.settings import Settings
from app import create_app
import uvicorn


def configure_logging(level: str = "INFO"):
    """Cấu hình logging toàn hệ thống"""
    # Đảm bảo level luôn là chữ hoa
    level = level.upper() if isinstance(level, str) else level
    logging.basicConfig(
        level=level,
        format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
        handlers=[
            logging.StreamHandler(),
            # logging.FileHandler("mcp_framework.log")  # Bật nếu cần ghi log ra file
        ],
    )
    # Giảm mức log cho một số thư viện ồn ào
    logging.getLogger("httpx").setLevel(logging.WARNING)
    logging.getLogger("asyncio").setLevel(logging.WARNING)


def print_startup_banner(settings: Settings):
    """Hiển thị thông tin khởi động"""
    print("\n" + "=" * 60)
    print(f"🚀 Starting MCP Services Framework v{settings.VERSION}")
    print("=" * 60)
    print(f"🔧 Environment: {settings.ENV}")
    print(f"🌐 Host: {settings.HOST}:{settings.PORT}")
    print(f"🔒 Debug Mode: {'ON' if settings.DEBUG else 'OFF'}")
    print(f"📡 API Endpoint: POST /mcp")
    print(f"📄 Documentation: GET /docs")
    print("-" * 60)


def main():
    # Tải cấu hình từ biến môi trường
    settings = Settings()  # type: ignore

    # Cấu hình logging
    configure_logging(level=settings.LOG_LEVEL)
    logger = logging.getLogger("main")

    # Tạo ứng dụng FastAPI
    app = create_app(settings)

    # Hiển thị thông tin khởi động
    print_startup_banner(settings)

    # Khởi chạy server
    uvicorn.run(
        app,
        host=settings.HOST,
        port=settings.PORT,
        log_level=settings.LOG_LEVEL.lower(),
        reload=settings.RELOAD,
        server_header=False,  # Tăng cường bảo mật
        proxy_headers=settings.PROXY_HEADERS,
        timeout_keep_alive=settings.KEEP_ALIVE_TIMEOUT,
    )


if __name__ == "__main__":
    main()
