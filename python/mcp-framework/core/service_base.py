from abc import ABC, abstractmethod
from typing import Any, Optional, Union


class ServiceBase(ABC):
    """
    Base class cho tất cả services
    """

    @property
    @abstractmethod
    def name(self) -> str:
        """Tên của service"""
        pass

    @property
    @abstractmethod
    def description(self) -> str:
        """Mô tả service"""
        pass

    @property
    @abstractmethod
    def input_schema(self) -> dict:
        """JSON Schema cho input parameters"""
        pass

    @abstractmethod
    def execute(self, params: Optional[Union[dict, list]]) -> Any:
        """Thực thi service logic"""
        pass

    def get_tool_definition(self) -> dict:
        """Trả về tool definition cho MCP"""
        return {
            "name": self.name,
            "description": self.description,
            "inputSchema": self.input_schema,
        }
