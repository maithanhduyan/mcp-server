from core.service_base import ServiceBase
from datetime import datetime
from typing import Any, Optional, Union


class PingService(ServiceBase):
    """Service để ping/pong"""

    @property
    def name(self) -> str:
        return "ping"

    @property
    def description(self) -> str:
        return "Simple ping service that returns pong"

    @property
    def input_schema(self) -> dict:
        return {"type": "object", "properties": {}}

    def execute(self, params: Optional[Union[dict, list]]) -> Any:
        return {
            "message": "pong",
            "timestamp": datetime.now().isoformat(),
            "service": self.name,
        }
