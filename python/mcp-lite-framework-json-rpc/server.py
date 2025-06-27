from fastapi import FastAPI, HTTPException
from pydantic import BaseModel, Field
from typing import Any, Optional, Union, Dict, Callable
from abc import ABC, abstractmethod
from datetime import datetime
import json
import uvicorn
import inspect
import logging
import logging.config
import threading
import queue
import logging.handlers

app = FastAPI(title="MCP Services Framework", version="1.0.0")


# ========== ASYNC LOGGING CONFIG ==========
log_queue = queue.Queue(-1)


class QueueListenerThread(threading.Thread):
    def __init__(self, log_queue, handlers):
        super().__init__(daemon=True)
        self.listener = logging.handlers.QueueListener(log_queue, *handlers)

    def run(self):
        self.listener.start()


# ========== LOGGING ==========
def get_logging_config() -> Dict[str, Any]:
    """
    Trả về cấu hình logging với timestamp đầy đủ, hỗ trợ bất đồng bộ qua QueueHandler.
    Returns:
        Dict[str, Any]: Logging configuration dictionary
    """
    return {
        "version": 1,
        "disable_existing_loggers": False,
        "formatters": {
            "detailed": {
                "format": "%(asctime)s | %(name)-20s | %(levelname)-8s | %(message)s",
                "datefmt": "%Y-%m-%d %H:%M:%S",
            },
            "simple": {
                "format": "%(asctime)s | %(levelname)-8s | %(message)s",
                "datefmt": "%H:%M:%S",
            },
            "access": {
                "format": "%(asctime)s | ACCESS | %(message)s",
                "datefmt": "%Y-%m-%d %H:%M:%S",
            },
        },
        "handlers": {
            "console": {
                "class": "logging.StreamHandler",
                "formatter": "detailed",
                "stream": "ext://sys.stdout",
            },
            "access_console": {
                "class": "logging.StreamHandler",
                "formatter": "access",
                "stream": "ext://sys.stdout",
            },
            "queue": {
                "class": "logging.handlers.QueueHandler",
                "queue": log_queue,
            },
        },
        "loggers": {
            # Root logger
            "": {
                "handlers": ["queue"],
                "level": "INFO",
                "propagate": False,
            },
            # Uvicorn loggers
            "uvicorn": {
                "handlers": ["queue"],
                "level": "INFO",
                "propagate": False,
            },
            "uvicorn.error": {
                "handlers": ["queue"],
                "level": "INFO",
                "propagate": False,
            },
            "uvicorn.access": {
                "handlers": ["queue"],
                "level": "INFO",
                "propagate": False,
            },
            # PolyMind app loggers
            "backend": {
                "handlers": ["queue"],
                "level": "INFO",
                "propagate": False,
            },
        },
    }


# Cấu hình logging bất đồng bộ
logging.config.dictConfig(get_logging_config())

# Khởi động QueueListener cho bất đồng bộ
# Sử dụng handler với formatter 'detailed' để log ra console có timestamp
console_handler = logging.StreamHandler()
console_handler.setFormatter(logging.Formatter(
    fmt="%(asctime)s | %(name)-20s | %(levelname)-8s | %(message)s",
    datefmt="%Y-%m-%d %H:%M:%S"
))
queue_listener_thread = QueueListenerThread(log_queue, [console_handler])
queue_listener_thread.start()

logger = logging.getLogger("mcp.server")

# ========== JSON-RPC 2.0 ==========


# JSON-RPC 2.0 Models
class JsonRpcRequest(BaseModel):
    jsonrpc: str = Field(default="2.0", description="JSON-RPC version")
    method: str = Field(..., description="Method name")
    params: Optional[Union[dict, list]] = Field(None, description="Method parameters")
    id: Optional[Union[str, int]] = Field(None, description="Request ID")


class JsonRpcResponse(BaseModel):
    jsonrpc: str = Field(default="2.0", description="JSON-RPC version")
    result: Any = Field(..., description="Method result")
    id: Optional[Union[str, int]] = Field(None, description="Request ID")


class JsonRpcError(BaseModel):
    code: int = Field(..., description="Error code")
    message: str = Field(..., description="Error message")
    data: Optional[Any] = Field(None, description="Additional error data")


class JsonRpcErrorResponse(BaseModel):
    jsonrpc: str = Field(default="2.0", description="JSON-RPC version")
    error: JsonRpcError = Field(..., description="Error details")
    id: Optional[Union[str, int]] = Field(None, description="Request ID")


# Error codes theo JSON-RPC 2.0 specification
ERROR_CODES = {
    "PARSE_ERROR": -32700,
    "INVALID_REQUEST": -32600,
    "METHOD_NOT_FOUND": -32601,
    "INVALID_PARAMS": -32602,
    "INTERNAL_ERROR": -32603,
}


def create_error_response(
    error_code: str,
    message: str,
    request_id: Optional[Union[str, int]] = None,
    data: Any = None,
) -> JsonRpcErrorResponse:
    """Tạo response lỗi theo chuẩn JSON-RPC 2.0"""
    return JsonRpcErrorResponse(
        error=JsonRpcError(
            code=ERROR_CODES.get(error_code, -32603), message=message, data=data
        ),
        id=request_id,
    )


def create_success_response(
    result: Any, request_id: Optional[Union[str, int]] = None
) -> JsonRpcResponse:
    """Tạo response thành công theo chuẩn JSON-RPC 2.0"""
    return JsonRpcResponse(result=result, id=request_id)


# ========== FRAMEWORK CORE ==========


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


# Global registry instance
registry = ServiceRegistry()

# ========== BUILT-IN SERVICES ==========


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


# ========== METHOD HANDLERS ==========


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


# Khởi tạo services
register_services()

# ========== FASTAPI ENDPOINTS ==========


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
                "METHOD_NOT_FOUND", f"Method '{request.method}' not found", request.id
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
        host="localhost",
        port=8000,
        log_level="info",
        log_config=get_logging_config(),
        access_log=True,
        # reload=True,  # Bật reload nếu cần thiết
    )


if __name__ == "__main__":
    main()
