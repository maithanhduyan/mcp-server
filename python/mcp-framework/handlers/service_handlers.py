# Các handler động cho service sẽ được định nghĩa ở đây
from typing import Optional, Union
from core.json_rpc import (
    JsonRpcResponse,
    create_success_response,
    create_error_response,
    JsonRpcErrorResponse,
)
from core.registry import registry
from core.json_rpc import ERROR_CODES


def handle_service_direct_call(service_name: str):
    """Tạo handler cho direct service call"""

    def handler(
        params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
    ) -> Union[JsonRpcResponse, JsonRpcErrorResponse]:
        service = registry.get_service(service_name)
        if not service:
            return create_error_response(
                "METHOD_NOT_FOUND", f"Service '{service_name}' not found", request_id
            )

        try:
            result = service.execute(params)
            return create_success_response(result=result, request_id=request_id)
        except Exception as e:
            return create_error_response(
                "INTERNAL_ERROR",
                f"Error executing service '{service_name}': {str(e)}",
                request_id,
            )

    return handler


def handle_server_info(
    params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
) -> JsonRpcResponse:
    """Thông tin về server"""
    services = registry.get_all_services()
    return create_success_response(
        result={
            "name": "MCP Services Framework",
            "version": "1.0.0",
            "protocol": "JSON-RPC 2.0",
            "total_services": len(services),
            "services": {
                name: service.description for name, service in services.items()
            },
            "methods": registry.get_all_methods(),
        },
        request_id=request_id,
    )
