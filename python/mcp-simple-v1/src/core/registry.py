from typing import Dict, Callable, Optional
from core.service_base import ServiceBase
from core.logging import get_logger

logger = get_logger(__name__)


class ServiceRegistry:
    """
    Registry để quản lý các services
    """

    def __init__(self):
        self._services: Dict[str, ServiceBase] = {}
        self._method_handlers: Dict[str, Callable] = {}

    def register(self, service: ServiceBase):
        """Đăng ký một service mới"""
        self._services[service.name] = service
        logger.info(f"✅ Registered service: {service.name}")

    def get_service(self, name: str) -> Optional[ServiceBase]:
        """Lấy service theo tên"""
        return self._services.get(name)

    def get_all_services(self) -> Dict[str, ServiceBase]:
        """Lấy tất cả services"""
        return self._services.copy()

    def register_method_handler(self, method_name: str, handler: Callable):
        """Đăng ký method handler"""
        self._method_handlers[method_name] = handler
        logger.info(f"✅ Registered method handler: {method_name}")

    def get_method_handler(self, method_name: str) -> Optional[Callable]:
        """Lấy method handler"""
        return self._method_handlers.get(method_name)

    def get_all_methods(self) -> list:
        """Lấy tất cả method names"""
        return list(self._method_handlers.keys())


# Global registry instance dùng chung toàn dự án
registry = ServiceRegistry()
