"""
Built-in
Time Service
Service để lấy thời gian hiện tại
"""

from datetime import datetime
from typing import Any, Optional, Union
from core.service_base import ServiceBase


class GetCurrentTimeService(ServiceBase):
    """Service để lấy thời gian hiện tại"""

    @property
    def name(self) -> str:
        return "get_current_time"

    @property
    def description(self) -> str:
        return "Get the current date and time"

    @property
    def input_schema(self) -> dict:
        return {
            "type": "object",
            "properties": {
                "timezone": {
                    "type": "string",
                    "description": "Timezone (optional, defaults to UTC)",
                    "default": "UTC",
                },
                "format": {
                    "type": "string",
                    "description": "Time format (iso, timestamp, readable)",
                    "enum": ["iso", "timestamp", "readable"],
                    "default": "iso",
                },
            },
        }

    def execute(self, params: Optional[Union[dict, list]]) -> Any:
        now = datetime.now()

        # Default values
        timezone = "UTC"
        format_type = "iso"

        if isinstance(params, dict):
            timezone = params.get("timezone", "UTC")
            format_type = params.get("format", "iso")

        result = {"service": self.name, "timezone": timezone, "format": format_type}

        if format_type == "iso":
            result["current_time"] = now.isoformat()
        elif format_type == "timestamp":
            result["current_time"] = int(now.timestamp())
        elif format_type == "readable":
            result["current_time"] = now.strftime("%Y-%m-%d %H:%M:%S")

        return result
