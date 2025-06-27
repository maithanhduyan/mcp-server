"""
Handlers for JSON-RPC requests

"""

from typing import Optional, Union
import json
from core.json_rpc import (
    JsonRpcResponse,
    JsonRpcErrorResponse,
    create_success_response,
    create_error_response,
)
from core.registry import registry


def handle_initialize(
    params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
) -> JsonRpcResponse:
    """Initialize method - bắt buộc cho MCP integration"""
    client_info = {}
    if isinstance(params, dict):
        client_info = params.get("clientInfo", {})

    # Lấy tất cả services để tạo capabilities
    tools = {}
    for service_name, service in registry.get_all_services().items():
        tools[service_name] = service.get_tool_definition()

    return create_success_response(
        result={
            "protocolVersion": "2024-11-05",
            "capabilities": {"tools": tools, "resources": {}, "prompts": {}},
            "serverInfo": {"name": "mcp-services-framework", "version": "1.0.0"},
            "instructions": f"MCP Services Framework with {len(tools)} available services.",
        },
        request_id=request_id,
    )


def handle_tools_list(
    params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
) -> JsonRpcResponse:
    """List available tools"""
    tools = []
    for service in registry.get_all_services().values():
        tools.append(service.get_tool_definition())

    return create_success_response(result={"tools": tools}, request_id=request_id)


def handle_tools_call(
    params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
) -> Union[JsonRpcResponse, JsonRpcErrorResponse]:
    """Call a tool"""
    if not isinstance(params, dict):
        return create_error_response(
            "INVALID_PARAMS", "Invalid parameters for tools/call", request_id
        )

    tool_name = params.get("name")
    arguments = params.get("arguments", {})

    if not isinstance(tool_name, str):
        return create_error_response(
            "INVALID_PARAMS", "Tool name must be a string", request_id
        )

    service = registry.get_service(tool_name)
    if not service:
        return create_error_response(
            "METHOD_NOT_FOUND", f"Tool '{tool_name}' not found", request_id
        )

    try:
        result = service.execute(arguments)
        return create_success_response(
            result={
                "content": [{"type": "text", "text": json.dumps(result, indent=2)}],
                "isError": False,
            },
            request_id=request_id,
        )
    except Exception as e:
        return create_error_response(
            "INTERNAL_ERROR",
            f"Error executing tool '{tool_name}': {str(e)}",
            request_id,
        )


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
