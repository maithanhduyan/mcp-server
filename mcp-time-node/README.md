# MCP Time Server 

A Model Context Protocol (MCP) server that provides time-related functionality using Node.js.

## Features

1. **Get Current Time** - Lấy thời gian hiện tại theo múi giờ được chỉ định
2. **Convert Time** - Chuyển đổi thời gian giữa các múi giờ khác nhau

## Installation

```bash
npm install
```

## Usage

### Running the Server
```bash
npm start
```

### Available Tools

#### get_current_time
Lấy thời gian hiện tại trong múi giờ được chỉ định.

**Parameters:**
- `timezone` (string): IANA timezone name (e.g., 'America/New_York', 'Europe/London', 'Asia/Ho_Chi_Minh')

**Example:**
```json
{
  "name": "get_current_time",
  "arguments": {
    "timezone": "Asia/Ho_Chi_Minh"
  }
}
```

#### convert_time
Chuyển đổi thời gian từ múi giờ này sang múi giờ khác.

**Parameters:**
- `time` (string): Thời gian cần chuyển đổi theo định dạng 24 giờ (HH:MM)
- `source_timezone` (string): Múi giờ nguồn
- `target_timezone` (string): Múi giờ đích

**Example:**
```json
{
  "name": "convert_time",
  "arguments": {
    "time": "14:30",
    "source_timezone": "Asia/Ho_Chi_Minh",
    "target_timezone": "America/New_York"
  }
}
```

## MCP Configuration

Server đã được cấu hình trong `.vscode/mcp.json`:

```json
{
  "servers": {
    "mcp-time-node": {
      "command": "node",
      "args": [
        "mcp-time-node/server.js"
      ],
      "env": {}
    }
  }
}
```

## Docker Support

Có thể sử dụng Dockerfile để đóng gói ứng dụng thành container (xem file Dockerfile).