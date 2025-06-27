# src/core/logging.py

import logging
import logging.config
import threading
import queue
import logging.handlers
from typing import Any, Dict

log_queue = queue.Queue(-1)


class QueueListenerThread(threading.Thread):
    def __init__(self, log_queue, handlers):
        super().__init__(daemon=True)
        self.listener = logging.handlers.QueueListener(log_queue, *handlers)

    def run(self):
        self.listener.start()


def get_logging_config() -> Dict[str, Any]:
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
            "file": {
                "class": "logging.handlers.RotatingFileHandler",
                "formatter": "detailed",
                "filename": "logs/server.log",
                "maxBytes": 5_000_000,
                "backupCount": 5,
                "encoding": "utf-8",
            },
            "queue": {
                "class": "logging.handlers.QueueHandler",
                "queue": log_queue,
            },
        },
        "loggers": {
            "": {
                "handlers": ["queue"],
                "level": "INFO",
                "propagate": False,
            },
            "uvicorn": {
                "handlers": ["queue"],
                "level": "INFO",
                "propagate": False,
            },
            "uvicorn.error": {
                "handlers": ["queue"],
                "level": "INFO",
                "propagate": False,
            },
            "uvicorn.access": {
                "handlers": ["queue"],
                "level": "INFO",
                "propagate": False,
            },
            "backend": {
                "handlers": ["queue"],
                "level": "INFO",
                "propagate": False,
            },
        },
    }


def setup_logging():
    """Khởi tạo config và QueueListener cho toàn hệ thống"""
    logging.config.dictConfig(get_logging_config())
    # Handlers cho console & file
    console_handler = logging.StreamHandler()
    console_handler.setFormatter(
        logging.Formatter(
            fmt="%(asctime)s | %(name)-20s | %(levelname)-8s | %(message)s",
            datefmt="%Y-%m-%d %H:%M:%S",
        )
    )
    file_handler = logging.handlers.RotatingFileHandler(
        filename="logs/server.log", maxBytes=5_000_000, backupCount=5, encoding="utf-8"
    )
    file_handler.setFormatter(
        logging.Formatter(
            fmt="%(asctime)s | %(name)-20s | %(levelname)-8s | %(message)s",
            datefmt="%Y-%m-%d %H:%M:%S",
        )
    )
    # Start QueueListener thread
    queue_listener_thread = QueueListenerThread(
        log_queue, [console_handler, file_handler]
    )
    queue_listener_thread.start()


def get_logger(name: str):
    return logging.getLogger(name)
