import anyio
import click
import datetime
from mcp.types import (
    Tool,
    TextContent,
    CallToolRequest,
)
from mcp.server.lowlevel import Server

async def fetch_time() -> TextContent:
    """Fetch the current time from the system."""
    now = datetime.datetime.now()
    time_str = now.strftime("%Y-%m-%d %H:%M:%S")
    return TextContent(type="text", text=time_str)

@click.command()
@click.option("--port", default=8000, help="Port to listen on for SSE")
@click.option(
    "--transport",
    type=click.Choice(["stdio", "sse"]),
    default="stdio",
    help="Transport type",
)
def main(port: int, transport: str) -> int:
    app = Server("mcp-time")

    @app.call_tool()
    async def get_current_time(self, request: CallToolRequest) -> list[TextContent]:
        """Tool to fetch the current time."""
        time_content = await fetch_time()  # Gọi hàm fetch_time để lấy thời gian
        return [time_content]  # Trả về thời gian thực tế

    @app.list_tools()
    async def list_tools() -> list[Tool]:
        return [
            Tool(
                name="get_current_time",
                description="Get the current time.",
                inputSchema={
                    "type": "object",
                    "properties": {},
                    "required": [],
                },
            )
        ]

    if transport == "sse":
        from mcp.server.sse import SseServerTransport
        from starlette.applications import Starlette
        from starlette.responses import Response
        from starlette.routing import Mount, Route

        sse = SseServerTransport("/messages/")

        async def handle_sse(request):
            async with sse.connect_sse(
                request.scope, request.receive, request._send
            ) as streams:
                await app.run(
                    streams[0], streams[1], app.create_initialization_options()
                )
            return Response()

        starlette_app = Starlette(
            debug=True,
            routes=[
                Route("/sse", endpoint=handle_sse, methods=["GET"]),
                Mount("/messages/", app=sse.handle_post_message),
            ],
        )

        import uvicorn

        uvicorn.run(starlette_app, host="127.0.0.1", port=port)
    else:
        from mcp.server.stdio import stdio_server

        async def arun():
            async with stdio_server() as streams:
                await app.run(
                    streams[0], streams[1], app.create_initialization_options()
                )

        anyio.run(arun)

    return 0

if __name__ == "__main__":
    main()