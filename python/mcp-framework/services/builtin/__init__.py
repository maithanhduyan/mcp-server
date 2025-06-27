from core.registry import registry
from .echo import EchoService
from .ping import PingService
from .time_service import GetCurrentTimeService
from handlers import (
    handle_initialize,
    handle_tools_list,
    handle_tools_call,
    handle_server_info,
    handle_service_direct_call,
)

# ========== SERVICE REGISTRATION ==========


def register_services():
    """Đăng ký tất cả services"""
    # Đăng ký built-in services
    registry.register(EchoService())
    registry.register(GetCurrentTimeService())
    registry.register(PingService())

    # Đăng ký method handlers
    registry.register_method_handler("initialize", handle_initialize)
    registry.register_method_handler("tools/list", handle_tools_list)
    registry.register_method_handler("tools/call", handle_tools_call)
    registry.register_method_handler("server/info", handle_server_info)

    # Đăng ký direct service call handlers
    for service_name in registry.get_all_services().keys():
        registry.register_method_handler(
            service_name, handle_service_direct_call(service_name)
        )
    print(
        f"✅ Registered {len(registry.get_all_services())} services and {len(registry.get_all_methods())} method handlers."
    )
