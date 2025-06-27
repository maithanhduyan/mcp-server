
```
mcp-framework/
├── core/                       # Core framework components
│   ├── __init__.py
│   ├── service_base.py         # ServiceBase abstract class
│   ├── registry.py             # ServiceRegistry implementation
│   ├── json_rpc.py             # JSON-RPC models and utilities
│   ├── security.py             # Security middleware and utilities
│   └── exceptions.py           # Custom exception classes
│
├── services/                   # Service implementations
│   ├── __init__.py             # Auto-registration mechanism
│   ├── builtin/                # Built-in services
│   │   ├── __init__.py
│   │   ├── echo.py             # EchoService
│   │   ├── time_service.py     # GetCurrentTimeService
│   │   └── ping.py             # PingService
│   │
│   └── custom/                 # User-defined services
│       ├── __init__.py
│       ├── plugin_loader.py    # Auto-discovery for custom services ⭐
│       └── ...                 # Custom service modules
│
├── handlers/                   # JSON-RPC method handlers
│   ├── __init__.py
│   ├── core_handlers.py        # initialize, tools/list, etc.
│   └── service_handlers.py     # Dynamic service handlers
│
├── api/                        # API endpoints
│   ├── __init__.py
│   ├── routes.py               # FastAPI route definitions
│   └── dependencies.py         # API dependencies
│
├── middlewares/                # Request/response middleware ⭐
│   ├── __init__.py
│   ├── auth.py                 # Authentication middleware
│   ├── logging.py              # Request logging
│   └── rate_limiter.py         # Rate limiting
│
├── config/                     # Configuration files
│   ├── __init__.py
│   ├── settings.py             # Main configuration (.env support)
│   └── security.py             # Security settings
│
├── utils/                      # Utility functions
│   ├── __init__.py
│   ├── logging.py              # Structured logging setup
│   ├── validation.py           # Data validation utils
│   └── response.py             # Standardized response formatting
│
├── docs/                       # API documentation ⭐
│   ├── openapi.json            # Auto-generated OpenAPI spec
│   └── examples/               # Example requests
│       ├── initialize.json
│       └── tools_call.json
│
├── tests/                      # Automated tests
│   ├── unit/
│   │   ├── test_services.py
│   │   └── test_handlers.py
│   └── integration/
│       ├── test_api_endpoints.py
│       └── test_security.py
│
├── scripts/                    # Management scripts ⭐
│   ├── generate_docs.py        # OpenAPI doc generator
│   └── service_scaffold.py     # Service template generator
│
├── app.py                      # FastAPI application factory
├── main.py                     # Application entry point
└── requirements.txt            # Dependencies

```