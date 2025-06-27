"""
Main entry point for the MCP Simple v1 JSON-RPC server.
This server implements a basic JSON-RPC 2.0 interface for MCP services.
"""

from fastapi import FastAPI, Request
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI(title="MCP Simple v1 JSON-RPC Server")

# Allow CORS for all origins (for development)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.post("/mcp")
async def json_rpc(request: Request):
    return 0


@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {
        "status": "healthy",
        "service": "MCP Services Framework",
        "uptime": "running",
    }


def main():
    """Main entry point to run the server"""
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8001, log_level="info")


if __name__ == "__main__":
    main()
