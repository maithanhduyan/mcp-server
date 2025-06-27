import logging
from typing import Dict, Any


def get_logger(name: str = "root"):
    """Trả về logger theo tên, mặc định là root logger."""
    return logging.getLogger(name)


def get_logging_config() -> Dict[str, Any]:
    """
    Trả về cấu hình logging với timestamp đầy đủ.

    Returns:
        Dict[str, Any]: Logging configuration dictionary
    """
    return {
        "version": 1,
        "disable_existing_loggers": False,
        "formatters": {
            "detailed": {
                "format": "%(asctime)s | %(name)-20s | %(levelname)-8s | %(message)s",
                "datefmt": "%Y-%m-%d %H:%M:%S",
            },
            "simple": {
                "format": "%(asctime)s | %(levelname)-8s | %(message)s",
                "datefmt": "%H:%M:%S",
            },
            "access": {
                "format": "%(asctime)s | ACCESS | %(message)s",
                "datefmt": "%Y-%m-%d %H:%M:%S",
            },
        },
        "handlers": {
            "console": {
                "class": "logging.StreamHandler",
                "formatter": "detailed",
                "stream": "ext://sys.stdout",
            },
            "access_console": {
                "class": "logging.StreamHandler",
                "formatter": "access",
                "stream": "ext://sys.stdout",
            },
        },
        "loggers": {
            # Root logger
            "": {
                "handlers": ["console"],
                "level": "INFO",
                "propagate": False,
            },
            # Uvicorn loggers
            "uvicorn": {
                "handlers": ["console"],
                "level": "INFO",
                "propagate": False,
            },
            "uvicorn.error": {
                "handlers": ["console"],
                "level": "INFO",
                "propagate": False,
            },
            "uvicorn.access": {
                "handlers": ["access_console"],
                "level": "INFO",
                "propagate": False,
            },
            # PolyMind app loggers
            "backend": {
                "handlers": ["console"],
                "level": "INFO",
                "propagate": False,
            },
        },
    }
