from fastapi import FastAPI, HTTPException
from pydantic import BaseModel, Field
from typing import Any, Optional, Union
import json
import uvicorn

app = FastAPI(title="JSON-RPC 2.0 Echo Server", version="1.0.0")


# JSON-RPC 2.0 Request Model
class JsonRpcRequest(BaseModel):
    jsonrpc: str = Field(default="2.0", description="JSON-RPC version")
    method: str = Field(..., description="Method name")
    params: Optional[Union[dict, list]] = Field(None, description="Method parameters")
    id: Optional[Union[str, int]] = Field(None, description="Request ID")


# JSON-RPC 2.0 Response Models
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


# Các method handlers
def handle_echo(
    params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
) -> JsonRpcResponse:
    """
    Hàm echo đơn giản - trả về chính xác những gì được gửi trong params
    """
    if params is None:
        return create_success_response(
            result={"message": "Hello from MCP Echo Server!", "echoed": None},
            request_id=request_id,
        )

    return create_success_response(
        result={"message": "Echo successful", "echoed": params}, request_id=request_id
    )


def handle_ping(
    params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
) -> JsonRpcResponse:
    """
    Hàm ping đơn giản để kiểm tra kết nối
    """
    return create_success_response(
        result={"message": "pong", "timestamp": "2025-06-27T10:00:00Z"},
        request_id=request_id,
    )


def handle_initialize(
    params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
) -> JsonRpcResponse:
    """
    Initialize method - bắt buộc cho MCP integration với VSCode
    """
    # Lấy thông tin client từ params nếu có
    client_info = {}
    if isinstance(params, dict):
        client_info = params.get("clientInfo", {})

    return create_success_response(
        result={
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {
                    "echo": {
                        "description": "Echo back the provided message",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "message": {
                                    "type": "string",
                                    "description": "Message to echo back",
                                }
                            },
                        },
                    }
                },
                "resources": {},
                "prompts": {},
            },
            "serverInfo": {"name": "mcp-echo-server", "version": "1.0.0"},
            "instructions": "This is an MCP Echo Server that can echo back messages.",
        },
        request_id=request_id,
    )


def handle_tools_list(
    params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
) -> JsonRpcResponse:
    """
    List available tools - method cần thiết cho MCP
    """
    return create_success_response(
        result={
            "tools": [
                {
                    "name": "echo",
                    "description": "Echo back the provided message",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "message": {
                                "type": "string",
                                "description": "Message to echo back",
                            }
                        },
                        "required": ["message"],
                    },
                }
            ]
        },
        request_id=request_id,
    )


def handle_tools_call(
    params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
) -> JsonRpcResponse:
    """
    Call a tool - xử lý tool calls từ MCP client
    """
    if not isinstance(params, dict):
        return create_error_response(
            "INVALID_PARAMS", "Invalid parameters for tools/call", request_id
        )

    tool_name = params.get("name")
    arguments = params.get("arguments", {})

    if tool_name == "echo":
        message = arguments.get("message", "")
        return create_success_response(
            result={
                "content": [{"type": "text", "text": f"Echo: {message}"}],
                "isError": False,
            },
            request_id=request_id,
        )
    else:
        return create_error_response(
            "METHOD_NOT_FOUND", f"Tool '{tool_name}' not found", request_id
        )


def handle_info(
    params: Optional[Union[dict, list]], request_id: Optional[Union[str, int]]
) -> JsonRpcResponse:
    """
    Thông tin về server
    """
    return create_success_response(
        result={
            "name": "MCP Echo Server",
            "version": "1.0.0",
            "protocol": "JSON-RPC 2.0",
            "methods": [
                "initialize",
                "tools/list",
                "tools/call",
                "echo",
                "ping",
                "info",
            ],
        },
        request_id=request_id,
    )


# Method registry
METHOD_HANDLERS = {
    "initialize": handle_initialize,
    "tools/list": handle_tools_list,
    "tools/call": handle_tools_call,
    "echo": handle_echo,
    "ping": handle_ping,
    "info": handle_info,
}


@app.post("/mcp")
async def json_rpc_endpoint(request: JsonRpcRequest):
    """
    JSON-RPC 2.0 endpoint chính
    """
    try:
        # Kiểm tra JSON-RPC version
        if request.jsonrpc != "2.0":
            return create_error_response(
                "INVALID_REQUEST",
                "Invalid JSON-RPC version. Expected '2.0'",
                request.id,
            )

        # Kiểm tra method có tồn tại không
        if request.method not in METHOD_HANDLERS:
            return create_error_response(
                "METHOD_NOT_FOUND", f"Method '{request.method}' not found", request.id
            )

        # Gọi method handler
        handler = METHOD_HANDLERS[request.method]
        response = handler(request.params, request.id)

        return response

    except Exception as e:
        return create_error_response(
            "INTERNAL_ERROR", f"Internal server error: {str(e)}", request.id
        )


@app.get("/")
async def root():
    """
    Root endpoint với thông tin cơ bản
    """
    return {
        "name": "JSON-RPC 2.0 Echo Server",
        "version": "1.0.0",
        "protocol": "JSON-RPC 2.0",
        "endpoint": "/mcp",
        "methods": list(METHOD_HANDLERS.keys()),
        "example_requests": [
            {
                "description": "Initialize MCP connection",
                "request": {
                    "jsonrpc": "2.0",
                    "method": "initialize",
                    "params": {"clientInfo": {"name": "vscode", "version": "1.0.0"}},
                    "id": 1,
                },
            },
            {
                "description": "Echo message",
                "request": {
                    "jsonrpc": "2.0",
                    "method": "echo",
                    "params": {"message": "Hello World!"},
                    "id": 2,
                },
            },
            {
                "description": "Call echo tool",
                "request": {
                    "jsonrpc": "2.0",
                    "method": "tools/call",
                    "params": {
                        "name": "echo",
                        "arguments": {"message": "Hello from MCP!"},
                    },
                    "id": 3,
                },
            },
        ],
    }


@app.get("/health")
async def health_check():
    """
    Health check endpoint
    """
    return {"status": "healthy", "service": "MCP Echo Server"}


if __name__ == "__main__":
    print("🚀 Starting MCP JSON-RPC 2.0 Echo Server...")
    print("📡 Endpoint: http://localhost:8000/mcp")
    print("📚 Available methods:")
    print("   - initialize (required for MCP)")
    print("   - tools/list (list available tools)")
    print("   - tools/call (call tools)")
    print("   - echo, ping, info")
    print("🔗 API docs: http://localhost:8000/docs")
    print("📝 VSCode MCP config example:")
    print("   {")
    print('     "mcpServers": {')
    print('       "echo-server": {')
    print('         "command": "python",')
    print('         "args": ["path/to/this/script.py"],')
    print('         "env": {}')
    print("       }")
    print("     }")
    print("   }")

    uvicorn.run(app, host="localhost", port=8000, log_level="info")
