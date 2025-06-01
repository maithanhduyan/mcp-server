# MCP Server for Windows Security Monitoring

## 🎯 Mục tiêu
MCP Server bảo vệ máy tính Windows khỏi các cuộc tấn công từ mạng internet với các tính năng nâng cao:

### 🔐 Bảo vệ cơ bản
- 🛡️ Bảo vệ máy tính khỏi bị tấn công từ mạng internet
- 📊 Đọc thông số của Windows để có hướng tiếp cận nâng cao bảo mật
- 📈 Giám sát những chương trình dùng CPU, RAM quá mức cần thiết (> 70%)
- 🔍 Theo dõi trạng thái Windows Firewall

### 🚀 Tính năng nâng cao mới
- ⚡ **Smart Firewall Management**: Quản lý quy tắc firewall thông minh với tự động hết hạn
- 🤖 **Automated Response System**: Hệ thống phản ứng tự động với các mối đe dọa
- 🔍 **Enhanced Threat Detection**: Phát hiện mối đe dọa nâng cao với pattern matching
- ⏰ **Auto-close Firewall Rules**: Tự động đóng quy tắc firewall sau timeout
- 📡 **Background Monitoring**: Giám sát liên tục 24/7 trong background
- 🌐 **Network Connection Scanning**: Quét kết nối mạng đáng ngờ
- 📊 **Security Alerts Dashboard**: Bảng điều khiển cảnh báo bảo mật
- 🎯 **Real-time Threat Response**: Phản ứng tức thời với mối đe dọa

## 🚀 Cài đặt và Sử dụng

### 📦 Portable Installation
**Server này có thể hoạt động trên bất kỳ máy tính Windows nào!**
- ✅ Không cần đường dẫn cố định
- ✅ Tự động detect thư mục làm việc
- ✅ Sử dụng `${workspaceFolder}` trong VS Code
- ✅ Scripts tự động tìm đường dẫn tương đối

👉 **Xem hướng dẫn chi tiết**: [PORTABLE_INSTALLATION.md](PORTABLE_INSTALLATION.md)

### 1. Yêu cầu hệ thống
- Windows 10/11
- Python 3.10+ 
- uv (Python package manager)

### 2. Cài đặt môi trường (Bất kỳ máy nào)

```powershell
# Copy toàn bộ thư mục mcp-server đến vị trí mong muốn
# Ví dụ: D:\Projects\mcp-server hoặc C:\Tools\mcp-server

# Di chuyển đến thư mục dự án
cd "path\to\your\mcp-server\mcp-windows-python"
```

### 3. Chạy server

#### Cách 1: Sử dụng PowerShell script
```powershell
.\start_server.ps1
```

#### Cách 2: Sử dụng batch file
```cmd
start_server.bat
```

#### Cách 3: Chạy trực tiếp
```powershell
.\.venv\Scripts\Activate.ps1
python server.py
```

## Cấu hình

Chỉnh sửa file `.env` để thay đổi cấu hình:

```env
# Security Monitoring Settings
CPU_THRESHOLD=70
MEMORY_THRESHOLD=70
MONITOR_INTERVAL=60
FIREWALL_AUTO_CLOSE_TIMEOUT=300

# Security Features
ENABLE_FIREWALL_MONITORING=true
ENABLE_PROCESS_MONITORING=true
ENABLE_NETWORK_MONITORING=true
```

## Tính năng

### 📊 Resources (Tài nguyên)
- **System Information** - Thông tin hệ thống Windows
- **Running Processes** - Danh sách tiến trình đang chạy
- **Firewall Status** - Trạng thái tường lửa Windows

### 🛠️ Tools (Công cụ)
- **monitor_system_resources** - Giám sát CPU/RAM với ngưỡng cảnh báo
- **get_windows_security_status** - Trạng thái bảo mật tổng thể
- **check_firewall_rules** - Kiểm tra quy tắc tường lửa

## 📁 Cấu trúc Project (v2.0 - Optimized Structure)

**Following Elon Musk's principles: Simplified, Integrated, Efficient**

```
mcp-windows-python/
├── src/                    # 📦 Core Source Code (Single point of truth)
│   ├── __init__.py        # Package initialization
│   └── server.py          # Main MCP server application
├── tests/                  # 🧪 Complete Test Suite  
│   ├── __init__.py        # Test package init
│   ├── test_dependencies.py      # Dependency validation
│   ├── test_functionality.py     # Core functionality tests
│   ├── test_advanced_features.py # Advanced security tests
│   ├── test_portable.py          # Portability tests
│   ├── validate_server.py        # Server validation
│   └── demo_showcase.py          # Live demonstration
├── docs/                   # 📚 All Documentation
│   ├── README.md          # Main documentation
│   ├── SETUP_SUMMARY.md   # Quick setup guide
│   ├── STATUS.md          # Current status
│   ├── CHANGE_LOG.md      # Version history
│   ├── PORTABLE_*.md      # Portable deployment guides
│   └── *.md               # Other documentation
├── .env                    # Environment configuration
├── config.json            # Server settings
├── requirements.txt       # Dependencies (simplified)
├── pyproject.toml         # Project config (updated for src/)
├── start_server.ps1       # PowerShell launcher (updated)
├── start_server.bat       # Batch launcher (updated)
└── run_tests.ps1          # Test runner (new)
```

### 🎯 Benefits of New Structure:
- **Loại bỏ không cần thiết**: No scattered files
- **Đơn giản hóa triệt để**: Clear separation (src/tests/docs)
- **Tích hợp & giảm điểm hỏng**: Minimal components, maximum cohesion
- **Tốc độ là chìa khóa**: Faster navigation and development

## Phát triển

### Cài đặt development dependencies
```powershell
uv sync --dev
```

### Chạy tests
```powershell
python test_dependencies.py
```

## Tính năng sắp tới

- 🔥 Smart Firewall Management - Quản lý tường lửa thông minh
- 🚨 Enhanced Security Monitoring - Giám sát bảo mật nâng cao  
- 🤖 Automated Response System - Hệ thống phản ứng tự động
- 🔍 Advanced Process Analysis - Phân tích tiến trình nâng cao