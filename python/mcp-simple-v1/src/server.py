from fastapi import FastAPI
from fastapi.responses import JSONResponse
import json
from pydantic import BaseModel, Field
from typing import Any, Optional, Union, Dict, Callable
from abc import ABC, abstractmethod
from datetime import datetime
import uvicorn

from core.json_rpc import (
    JsonRpcRequest,
    JsonRpcResponse,
    JsonRpcErrorResponse,
    create_error_response,
    create_success_response,
)

from core.service_base import ServiceBase
from core.registry import registry

from core.logging import setup_logging, get_logging_config, get_logger

app = FastAPI(title="MCP Services Framework", version="1.0.0")

# Cấu hình logging bất đồng bộ
setup_logging()
logger = get_logger("mcp.server")

# ========== METHOD HANDLERS ==========
from core.handlers import (
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
    from sevices.echo_service import EchoService
    from sevices.time_service import GetCurrentTimeService
    from sevices.ping_service import PingService

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


# Khởi tạo services
register_services()

# ========== FASTAPI ENDPOINTS ==========
from core.json_rpc import UnicodeJSONResponse


@app.post("/mcp")
async def json_rpc_endpoint(request: JsonRpcRequest):
    """JSON-RPC 2.0 endpoint chính"""
    try:
        # Kiểm tra JSON-RPC version
        if request.jsonrpc != "2.0":
            return UnicodeJSONResponse(
                content=create_error_response(
                    "INVALID_REQUEST",
                    "Invalid JSON-RPC version. Expected '2.0'",
                    request.id,
                ).model_dump(),
            )

        # Lấy method handler
        handler = registry.get_method_handler(request.method)
        if not handler:
            return UnicodeJSONResponse(
                content=create_error_response(
                    "METHOD_NOT_FOUND",
                    f"Method '{request.method}' not found",
                    request.id,
                ).model_dump(),
            )

        # Gọi method handler
        response = handler(request.params, request.id)
        # Trả về JSON giữ nguyên Unicode
        return UnicodeJSONResponse(content=response.model_dump())

    except Exception as e:
        return UnicodeJSONResponse(
            content=create_error_response(
                "INTERNAL_ERROR", f"Internal server error: {str(e)}", request.id
            ).model_dump(),
        )


@app.get("/")
async def root():
    """Root endpoint với thông tin framework"""
    services = registry.get_all_services()
    return {
        "name": "MCP Services Framework",
        "version": "1.0.0",
        "protocol": "JSON-RPC 2.0",
        "mcp_endpoint": "POST /mcp",
        "total_services": len(services),
        "available_services": list(services.keys()),
        "available_methods": registry.get_all_methods(),
        "framework_features": [
            "Easy service registration",
            "Automatic MCP tool generation",
            "Direct service calls",
            "Built-in error handling",
            "JSON Schema validation",
        ],
        "example_requests": [
            {
                "description": "Initialize MCP",
                "request": {
                    "jsonrpc": "2.0",
                    "method": "initialize",
                    "params": {"clientInfo": {"name": "vscode", "version": "1.0.0"}},
                    "id": 1,
                },
            },
            {
                "description": "Get current time",
                "request": {
                    "jsonrpc": "2.0",
                    "method": "get_current_time",
                    "params": {"timezone": "UTC", "format": "iso"},
                    "id": 2,
                },
            },
            {
                "description": "Call tool via MCP",
                "request": {
                    "jsonrpc": "2.0",
                    "method": "tools/call",
                    "params": {
                        "name": "get_current_time",
                        "arguments": {"format": "readable"},
                    },
                    "id": 3,
                },
            },
        ],
    }


@app.get("/services")
async def list_services():
    """Endpoint để list tất cả services"""
    services = registry.get_all_services()
    return {
        "total": len(services),
        "services": [
            {
                "name": service.name,
                "description": service.description,
                "input_schema": service.input_schema,
            }
            for service in services.values()
        ],
    }


@app.get("/health")
async def health_check():
    """Health check endpoint"""
    services = registry.get_all_services()
    return {
        "status": "healthy",
        "service": "MCP Services Framework",
        "services_count": len(services),
        "uptime": "running",
    }


def main():
    """Main entry point để chạy server"""
    logger.info("🚀 Starting MCP Services Framework...")
    logger.info("📡 Endpoint: http://localhost:8000/mcp")
    logger.info("🔧 Framework Features:")
    logger.info("   ✅ Easy service registration")
    logger.info("   ✅ Automatic MCP tool generation")
    logger.info("   ✅ Direct service calls")
    logger.info("   ✅ Built-in error handling")
    logger.info("   ✅ JSON Schema validation")
    logger.info("")

    services = registry.get_all_services()
    logger.info(f"📚 Available Services ({len(services)}):")
    for name, service in services.items():
        logger.info(f"   - {name}: {service.description}")

    logger.info("")
    methods = registry.get_all_methods()
    logger.info(f"🔗 Available Methods ({len(methods)}):")
    for method in methods:
        logger.info(f"   - {method}")

    logger.info("")
    logger.info("🔗 Endpoints:")
    logger.info("   - GET  / (framework info)")
    logger.info("   - GET  /services (service list)")
    logger.info("   - GET  /health (health check)")
    logger.info("   - POST /mcp (JSON-RPC endpoint)")
    logger.info("   - GET  /docs (API documentation)")

    uvicorn.run(
        app,
        host="0.0.0.0",  # Sửa để cho phép truy cập từ bên ngoài container
        port=8000,  # Đổi sang cổng 8001 nếu cần
        log_level="info",
        log_config=get_logging_config(),
        access_log=True,
        # reload=True,  # Bật reload nếu cần thiết
    )


if __name__ == "__main__":
    main()
