import argparse
from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse

import datetime

def get_tools():
    """List available tools."""
    return [
        {
            "name": "get_current_time",
            "description": "Get the current time.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "required": [],
            },
        }
    ]

def get_current_time():
    """Tool to fetch the current time."""
    now = datetime.datetime.now()
    time_str = now.strftime("%Y-%m-%d %H:%M:%S")
    return [{"type": "text", "text": time_str}]  # Đúng định dạng mcp.types.TextContent

def main():
    parser = argparse.ArgumentParser(description="Fetch the current time.")
    parser.add_argument("--port", type=int, default=8000, help="Port to run the server on")
    parser.add_argument("--host", default="localhost", help="Host to run the server on (default: localhost)")
    args = parser.parse_args()

    app = FastAPI()

    @app.get("/")
    def read_root():
        return {"hello": "world"}

    @app.get("/v1/list_tools")
    async def v1_list_tools():
        return get_tools()

    @app.post("/v1/call_tool")
    async def v1_call_tool(request: Request):
        body = await request.json()
        tool_name = body.get("name")
        if tool_name == "get_current_time":
            return get_current_time()
        return JSONResponse({"error": "Tool not found"}, status_code=404)

    import uvicorn
    uvicorn.run(app, host=args.host, port=args.port, log_level="info")

if __name__ == "__main__":
    main()
