from core.service_base import ServiceBase
from datetime import datetime
from typing import Any, Optional, Union


class EchoService(ServiceBase):
    """Service để echo message"""

    @property
    def name(self) -> str:
        return "echo"

    @property
    def description(self) -> str:
        return "Echo back the provided message"

    @property
    def input_schema(self) -> dict:
        return {
            "type": "object",
            "properties": {
                "message": {"type": "string", "description": "Message to echo back"}
            },
            "required": ["message"],
        }

    def execute(self, params: Optional[Union[dict, list]]) -> Any:
        if not isinstance(params, dict):
            raise ValueError("Echo service requires dict parameters")

        message = params.get("message", "")
        return {
            "echoed_message": message,
            "timestamp": datetime.now().isoformat(),
            "service": self.name,
        }
