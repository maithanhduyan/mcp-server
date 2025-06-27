"""
MCP Echo Server using standard MCP Server SDK
Supports both stdio and HTTP transport
"""

import asyncio
import sys
import argparse
import logging
from typing import Any, Sequence

from mcp.server import Server
from mcp.server.stdio import stdio_server
from mcp.types import (
    CallToolRequest,
    CallToolResult,
    ListToolsRequest,
    ListToolsResult,
    Tool,
    TextContent,
)

# Set up logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Create the server instance
server = Server("echo-server")


@server.list_tools()
async def list_tools() -> list[Tool]:
    """List available tools"""
    return [
        Tool(
            name="echo",
            description="Echo the input text back",
            inputSchema={
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "Text to echo back"
                    }
                },
                "required": ["text"]
            }
        )
    ]


@server.call_tool()
async def call_tool(name: str, arguments: dict[str, Any]) -> list[TextContent]:
    """Handle tool calls"""
    
    if name == "echo":
        text = arguments.get("text", "")
        logger.info(f"Echo tool called with text: {text}")
        return [TextContent(type="text", text=text)]
    
    else:
        raise ValueError(f"Unknown tool: {name}")


async def run_stdio():
    """Run server in stdio mode"""
    logger.info("Starting MCP Echo Server in stdio mode...")
    
    async with stdio_server(server) as (read_stream, write_stream):
        await server.run(
            read_stream, 
            write_stream, 
            server.create_initialization_options()
        )


async def run_http(host: str = "localhost", port: int = 8000):
    """Run server in HTTP mode with proper MCP SSE transport"""
    try:
        from mcp.server.sse import SseServerTransport
        from starlette.applications import Starlette
        from starlette.routing import Route
        from starlette.responses import Response
        from starlette.middleware.cors import CORSMiddleware
        import uvicorn
        import socket
    except ImportError as e:
        logger.error(f"Missing dependencies for HTTP mode: {e}")
        logger.error("Please install: pip install starlette uvicorn")
        return

    # Find available port if the specified port is busy
    def find_free_port(start_port: int, max_attempts: int = 100) -> int:
        for p in range(start_port, start_port + max_attempts):
            try:
                with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                    s.bind((host, p))
                    return p
            except OSError:
                continue
        raise OSError(f"Could not find a free port starting from {start_port}")

    # Try to find available port
    try:
        actual_port = port
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind((host, actual_port))
    except OSError:
        logger.warning(f"Port {port} is already in use, finding alternative...")
        actual_port = find_free_port(port + 1)

    # Create SSE transport
    sse_transport = SseServerTransport("/sse")
    
    async def handle_sse(request):
        """Handle SSE connections"""
        return await sse_transport.handle_request(request)
    
    async def handle_messages(request):
        """Handle POST messages"""
        return await sse_transport.handle_post_message(request, server)
    
    async def handle_jsonrpc(request):
        """Handle JSON-RPC requests directly"""
        try:
            import json
            from starlette.responses import JSONResponse
            
            # Parse JSON-RPC request
            body = await request.body()
            rpc_request = json.loads(body.decode())
            
            method = rpc_request.get("method")
            params = rpc_request.get("params", {})
            request_id = rpc_request.get("id")
            
            if method == "tools/list":
                tools = await list_tools()
                tools_dict = [
                    {
                        "name": tool.name,
                        "description": tool.description,
                        "inputSchema": tool.inputSchema
                    }
                    for tool in tools
                ]
                
                return JSONResponse({
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "result": {"tools": tools_dict}
                })
            
            elif method == "tools/call":
                tool_name = params.get("name")
                arguments = params.get("arguments", {})
                
                if not tool_name:
                    return JSONResponse({
                        "jsonrpc": "2.0",
                        "id": request_id,
                        "error": {
                            "code": -32602,
                            "message": "Invalid params: tool name is required"
                        }
                    })
                
                try:
                    result = await call_tool(tool_name, arguments)
                    result_dict = [
                        {"type": content.type, "text": content.text}
                        for content in result
                    ]
                    
                    return JSONResponse({
                        "jsonrpc": "2.0",
                        "id": request_id,
                        "result": {"content": result_dict}
                    })
                    
                except ValueError as e:
                    return JSONResponse({
                        "jsonrpc": "2.0",
                        "id": request_id,
                        "error": {
                            "code": -32602,
                            "message": str(e)
                        }
                    })
            
            else:
                return JSONResponse({
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "error": {
                        "code": -32601,
                        "message": f"Method not found: {method}"
                    }
                })
                
        except json.JSONDecodeError:
            return JSONResponse({
                "jsonrpc": "2.0",
                "id": None,
                "error": {
                    "code": -32700,
                    "message": "Parse error"
                }
            })
        except Exception as e:
            return JSONResponse({
                "jsonrpc": "2.0",
                "id": request_id,
                "error": {
                    "code": -32603,
                    "message": f"Internal error: {str(e)}"
                }
            })
    
    async def handle_health(request):
        """Health check endpoint"""
        return Response(
            content='{"status": "healthy", "server": "echo-server"}',
            media_type="application/json"
        )
    
    # Create Starlette app with CORS
    app = Starlette(
        routes=[
            Route("/", handle_jsonrpc, methods=["POST"]),  # JSON-RPC endpoint
            Route("/health", handle_health, methods=["GET"]),
            Route("/sse", handle_sse, methods=["GET"]),
            Route("/sse", handle_messages, methods=["POST"]),  # MCP SSE endpoint
        ]
    )
    
    # Add CORS middleware
    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],
        allow_methods=["GET", "POST"],
        allow_headers=["*"],
    )
    
    logger.info(f"Starting MCP HTTP server on {host}:{actual_port}")
    logger.info(f"JSON-RPC endpoint: http://{host}:{actual_port}/")
    logger.info(f"SSE endpoint: http://{host}:{actual_port}/sse")
    logger.info(f"Health check: http://{host}:{actual_port}/health")
    
    print(f"\nServer running on http://{host}:{actual_port}")
    print("Available endpoints:")
    print(f"  POST http://{host}:{actual_port}/          - JSON-RPC API")
    print(f"  GET  http://{host}:{actual_port}/sse       - SSE connection")
    print(f"  POST http://{host}:{actual_port}/sse       - SSE messages")
    print(f"  GET  http://{host}:{actual_port}/health    - Health check")
    print("\nExample JSON-RPC usage:")
    print(f"curl -X POST http://{host}:{actual_port}/ \\")
    print('  -H "Content-Type: application/json" \\')
    print('  -d \'{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}\'')
    print(f"\ncurl -X POST http://{host}:{actual_port}/ \\")
    print('  -H "Content-Type: application/json" \\')
    print('  -d \'{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "echo", "arguments": {"text": "Hello World"}}}\'')
    print("\nPress Ctrl+C to stop...")
    
    # Configure and run uvicorn
    config = uvicorn.Config(
        app=app,
        host=host,
        port=actual_port,
        log_level="info",
        access_log=True
    )
    
    server_instance = uvicorn.Server(config)
    
    try:
        await server_instance.serve()
    except KeyboardInterrupt:
        logger.info("Server shutdown requested")
    except Exception as e:
        logger.error(f"Server error: {e}")
        raise


def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(description="MCP Echo Server")
    parser.add_argument(
        "transport",
        choices=["stdio", "http"],
        help="Transport type: stdio for MCP clients, http for web access"
    )
    parser.add_argument(
        "--host",
        default="localhost",
        help="Host for HTTP server (default: localhost)"
    )
    parser.add_argument(
        "--port",
        type=int,
        default=8000,
        help="Port for HTTP server (default: 8000)"
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Enable verbose logging"
    )
    
    args = parser.parse_args()
    
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    
    try:
        if args.transport == "stdio":
            asyncio.run(run_stdio())
        elif args.transport == "http":
            asyncio.run(run_http(args.host, args.port))
    except KeyboardInterrupt:
        logger.info("Server stopped by user")
    except Exception as e:
        logger.error(f"Server failed to start: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()