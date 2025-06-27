"""
Dùng FastAPI để tạo một server JSON-RPC đơn giản.

"""

from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse

app = FastAPI()


@app.post("/rpc")
async def jsonrpc_endpoint(request: Request):
    data = await request.json()
    # Kiểm tra trường bắt buộc
    if data.get("jsonrpc") != "2.0":
        return JSONResponse(
            content={
                "jsonrpc": "2.0",
                "error": {"code": -32600, "message": "Invalid Request"},
                "id": data.get("id"),
            },
            status_code=400,
        )

    method = data.get("method")
    params = data.get("params")
    req_id = data.get("id")

    # Xử lý method
    if method == "echo":
        # Nhận params dạng object hoặc array
        if isinstance(params, dict):
            message = params.get("message")
        elif isinstance(params, list) and len(params) > 0:
            message = params[0]
        else:
            message = None

        return {"jsonrpc": "2.0", "result": message, "id": req_id}
    else:
        # Method không tồn tại
        return {
            "jsonrpc": "2.0",
            "error": {"code": -32601, "message": "Method not found"},
            "id": req_id,
        }


def main():
    import uvicorn

    # Chạy server với Uvicorn
    # uvicorn <tên_file_nguồn>:app --reload --port 8000
    uvicorn.run(app, host="localhost", port=8000)


# Chạy server:
if __name__ == "__main__":
    main()
