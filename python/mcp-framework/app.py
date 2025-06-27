from fastapi import FastAPI
from config.settings import Settings
from core import security
from api import routes
from utils import logging as logging_utils
from core.json_rpc import JsonRpcRequest, create_error_response
from core.registry import registry
from services.builtin import (
    register_services,
)


def create_app(settings: Settings) -> FastAPI:
    # Khởi tạo ứng dụng FastAPI
    app = FastAPI(
        title="MCP Services Framework",
        version=settings.VERSION,
        docs_url="/docs" if settings.DEBUG else None,
        redoc_url=None,
        openapi_url="/openapi.json" if settings.DEBUG else None,
        servers=(
            [{"url": "http://localhost:8000", "description": "Development Server"}]
            if settings.DEBUG
            else None
        ),
    )

    # Đăng ký middleware
    app = security.register_middleware(app, settings)

    # Khởi tạo services
    register_services()

    # Đăng ký router chính
    app.include_router(routes.api_router, prefix="/api", tags=["API"])

    @app.post("/mcp")
    async def json_rpc_endpoint(request: JsonRpcRequest):
        """JSON-RPC 2.0 endpoint chính"""
        try:
            # Kiểm tra JSON-RPC version
            if request.jsonrpc != "2.0":
                return create_error_response(
                    "INVALID_REQUEST",
                    "Invalid JSON-RPC version. Expected '2.0'",
                    request.id,
                )

            # Lấy method handler
            handler = registry.get_method_handler(request.method)
            if not handler:
                return create_error_response(
                    "METHOD_NOT_FOUND",
                    f"Method '{request.method}' not found",
                    request.id,
                )

            # Gọi method handler
            response = handler(request.params, request.id)
            return response

        except Exception as e:
            return create_error_response(
                "INTERNAL_ERROR", f"Internal server error: {str(e)}", request.id
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
                        "params": {
                            "clientInfo": {"name": "vscode", "version": "1.0.0"}
                        },
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

    # Đăng ký health check endpoint
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

    return app
