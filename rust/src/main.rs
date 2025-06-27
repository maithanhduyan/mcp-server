use anyhow::{anyhow, Result};
use async_trait::async_trait;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

// ========== JSON-RPC 2.0 MODELS ==========

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcErrorResponse {
    pub jsonrpc: String,
    pub error: JsonRpcError,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
}

// JSON-RPC Error Codes
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

// ========== SERVICE FRAMEWORK ==========

#[derive(Debug, Clone, Serialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

#[async_trait]
pub trait Service: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;
    async fn execute(&self, params: Option<Value>) -> Result<Value>;

    fn get_tool_definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name().to_string(),
            description: self.description().to_string(),
            input_schema: self.input_schema(),
        }
    }
}

pub type ServiceBox = Box<dyn Service>;
pub type MethodHandler = Box<dyn Fn(Option<Value>, Option<Value>) -> Result<Value> + Send + Sync>;

#[derive(Default)]
pub struct ServiceRegistry {
    services: HashMap<String, ServiceBox>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register(&mut self, service: ServiceBox) {
        let name = service.name().to_string();
        info!("✅ Registered service: {}", name);
        self.services.insert(name, service);
    }

    pub fn get_service(&self, name: &str) -> Option<&ServiceBox> {
        self.services.get(name)
    }

    pub fn get_all_services(&self) -> &HashMap<String, ServiceBox> {
        &self.services
    }

    pub fn get_service_names(&self) -> Vec<String> {
        self.services.keys().cloned().collect()
    }
}

// ========== BUILT-IN SERVICES ==========

#[derive(Debug)]
pub struct EchoService;

#[async_trait]
impl Service for EchoService {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "Echo back the provided message"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "Message to echo back"
                }
            },
            "required": ["message"]
        })
    }

    async fn execute(&self, params: Option<Value>) -> Result<Value> {
        let message = params
            .as_ref()
            .and_then(|p| p.get("message"))
            .and_then(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        Ok(json!({
            "echoed_message": message,
            "timestamp": Utc::now().to_rfc3339(),
            "service": self.name()
        }))
    }
}

#[derive(Debug)]
pub struct GetCurrentTimeService;

#[async_trait]
impl Service for GetCurrentTimeService {
    fn name(&self) -> &str {
        "get_current_time"
    }

    fn description(&self) -> &str {
        "Get the current date and time"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "timezone": {
                    "type": "string",
                    "description": "Timezone (optional, defaults to UTC)",
                    "default": "UTC"
                },
                "format": {
                    "type": "string",
                    "description": "Time format (iso, timestamp, readable)",
                    "enum": ["iso", "timestamp", "readable"],
                    "default": "iso"
                }
            }
        })
    }

    async fn execute(&self, params: Option<Value>) -> Result<Value> {
        let now = Utc::now();
        let timezone = params
            .as_ref()
            .and_then(|p| p.get("timezone"))
            .and_then(|t| t.as_str())
            .unwrap_or("UTC");

        let format_type = params
            .as_ref()
            .and_then(|p| p.get("format"))
            .and_then(|f| f.as_str())
            .unwrap_or("iso");

        let current_time = match format_type {
            "iso" => json!(now.to_rfc3339()),
            "timestamp" => json!(now.timestamp()),
            "readable" => json!(now.format("%Y-%m-%d %H:%M:%S").to_string()),
            _ => json!(now.to_rfc3339()),
        };

        Ok(json!({
            "service": self.name(),
            "timezone": timezone,
            "format": format_type,
            "current_time": current_time
        }))
    }
}

#[derive(Debug)]
pub struct PingService;

#[async_trait]
impl Service for PingService {
    fn name(&self) -> &str {
        "ping"
    }

    fn description(&self) -> &str {
        "Simple ping service that returns pong"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {}
        })
    }

    async fn execute(&self, _params: Option<Value>) -> Result<Value> {
        Ok(json!({
            "message": "pong",
            "timestamp": Utc::now().to_rfc3339(),
            "service": self.name()
        }))
    }
}

// ========== APPLICATION STATE ==========

pub struct AppState {
    pub registry: RwLock<ServiceRegistry>,
}

impl AppState {
    pub fn new() -> Self {
        let mut registry = ServiceRegistry::new();

        // Register built-in services
        registry.register(Box::new(EchoService));
        registry.register(Box::new(GetCurrentTimeService));
        registry.register(Box::new(PingService));

        Self {
            registry: RwLock::new(registry),
        }
    }
}

// ========== HELPER FUNCTIONS ==========

fn create_success_response(result: Value, id: Option<Value>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result,
        id,
    }
}

fn create_error_response(code: i32, message: &str, id: Option<Value>) -> JsonRpcErrorResponse {
    JsonRpcErrorResponse {
        jsonrpc: "2.0".to_string(),
        error: JsonRpcError {
            code,
            message: message.to_string(),
            data: None,
        },
        id,
    }
}

// ========== METHOD HANDLERS ==========

async fn handle_initialize(
    params: Option<Value>,
    id: Option<Value>,
    state: &AppState,
) -> Result<Value> {
    let registry = state.registry.read().await;
    let services = registry.get_all_services();

    let mut tools = json!({});
    for (name, service) in services {
        tools[name] = json!(service.get_tool_definition());
    }

    Ok(json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": tools,
            "resources": {},
            "prompts": {}
        },
        "serverInfo": {
            "name": "mcp-rust-server",
            "version": "1.0.0"
        },
        "instructions": format!("MCP Rust Server with {} available services.", services.len())
    }))
}

async fn handle_tools_list(
    params: Option<Value>,
    id: Option<Value>,
    state: &AppState,
) -> Result<Value> {
    let registry = state.registry.read().await;
    let services = registry.get_all_services();

    let tools: Vec<ToolDefinition> = services
        .values()
        .map(|service| service.get_tool_definition())
        .collect();

    Ok(json!({
        "tools": tools
    }))
}

async fn handle_tools_call(
    params: Option<Value>,
    id: Option<Value>,
    state: &AppState,
) -> Result<Value> {
    let params = params.ok_or_else(|| anyhow!("Missing parameters for tools/call"))?;

    let tool_name = params
        .get("name")
        .and_then(|n| n.as_str())
        .ok_or_else(|| anyhow!("Missing tool name"))?;

    let arguments = params.get("arguments");

    let registry = state.registry.read().await;
    let service = registry
        .get_service(tool_name)
        .ok_or_else(|| anyhow!("Tool '{}' not found", tool_name))?;

    match service.execute(arguments.cloned()).await {
        Ok(result) => Ok(json!({
            "content": [
                {
                    "type": "text",
                    "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                }
            ],
            "isError": false
        })),
        Err(e) => Ok(json!({
            "content": [
                {
                    "type": "text",
                    "text": format!("Error: {}", e)
                }
            ],
            "isError": true
        })),
    }
}

async fn handle_service_direct_call(
    service_name: &str,
    params: Option<Value>,
    id: Option<Value>,
    state: &AppState,
) -> Result<Value> {
    let registry = state.registry.read().await;
    let service = registry
        .get_service(service_name)
        .ok_or_else(|| anyhow!("Service '{}' not found", service_name))?;

    service.execute(params).await
}

async fn handle_server_info(
    params: Option<Value>,
    id: Option<Value>,
    state: &AppState,
) -> Result<Value> {
    let registry = state.registry.read().await;
    let services = registry.get_all_services();

    let service_info: HashMap<String, String> = services
        .iter()
        .map(|(name, service)| (name.clone(), service.description().to_string()))
        .collect();

    let methods = vec![
        "initialize",
        "tools/list",
        "tools/call",
        "server/info",
        "echo",
        "get_current_time",
        "ping",
    ];

    Ok(json!({
        "name": "MCP Rust Services Framework",
        "version": "1.0.0",
        "protocol": "JSON-RPC 2.0",
        "total_services": services.len(),
        "services": service_info,
        "methods": methods
    }))
}

// ========== HTTP HANDLERS ==========

async fn json_rpc_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<JsonRpcRequest>,
) -> Result<Json<Value>, StatusCode> {
    info!("Received JSON-RPC request: method={}", request.method);

    // Validate JSON-RPC version
    if request.jsonrpc != "2.0" {
        let error_response = create_error_response(
            INVALID_REQUEST,
            "Invalid JSON-RPC version. Expected '2.0'",
            request.id,
        );
        return Ok(Json(serde_json::to_value(error_response).unwrap()));
    }

    // Handle methods
    let result = match request.method.as_str() {
        "initialize" => handle_initialize(request.params, request.id.clone(), &state).await,
        "tools/list" => handle_tools_list(request.params, request.id.clone(), &state).await,
        "tools/call" => handle_tools_call(request.params, request.id.clone(), &state).await,
        "server/info" => handle_server_info(request.params, request.id.clone(), &state).await,
        // Direct service calls
        method_name => {
            let registry = state.registry.read().await;
            if registry.get_service(method_name).is_some() {
                drop(registry); // Release the lock before async call
                handle_service_direct_call(method_name, request.params, request.id.clone(), &state)
                    .await
            } else {
                let error_response = create_error_response(
                    METHOD_NOT_FOUND,
                    &format!("Method '{}' not found", method_name),
                    request.id,
                );
                return Ok(Json(serde_json::to_value(error_response).unwrap()));
            }
        }
    };

    match result {
        Ok(result_value) => {
            let response = create_success_response(result_value, request.id);
            Ok(Json(serde_json::to_value(response).unwrap()))
        }
        Err(e) => {
            error!("Error handling request: {}", e);
            let error_response = create_error_response(
                INTERNAL_ERROR,
                &format!("Internal server error: {}", e),
                request.id,
            );
            Ok(Json(serde_json::to_value(error_response).unwrap()))
        }
    }
}

async fn root_handler(State(state): State<Arc<AppState>>) -> Json<Value> {
    let registry = state.registry.read().await;
    let services = registry.get_service_names();

    Json(json!({
        "name": "MCP Rust Services Framework",
        "version": "1.0.0",
        "protocol": "JSON-RPC 2.0",
        "mcp_endpoint": "POST /mcp",
        "total_services": services.len(),
        "available_services": services,
        "available_methods": [
            "initialize", "tools/list", "tools/call", "server/info",
            "echo", "get_current_time", "ping"
        ],
        "framework_features": [
            "Easy service registration",
            "Automatic MCP tool generation",
            "Direct service calls",
            "Built-in error handling",
            "JSON Schema validation",
            "Async/await support"
        ],
        "example_requests": [
            {
                "description": "Initialize MCP",
                "request": {
                    "jsonrpc": "2.0",
                    "method": "initialize",
                    "params": {
                        "clientInfo": {
                            "name": "vscode",
                            "version": "1.0.0"
                        }
                    },
                    "id": 1
                }
            },
            {
                "description": "Get current time",
                "request": {
                    "jsonrpc": "2.0",
                    "method": "get_current_time",
                    "params": {
                        "timezone": "UTC",
                        "format": "iso"
                    },
                    "id": 2
                }
            },
            {
                "description": "Call tool via MCP",
                "request": {
                    "jsonrpc": "2.0",
                    "method": "tools/call",
                    "params": {
                        "name": "get_current_time",
                        "arguments": {
                            "format": "readable"
                        }
                    },
                    "id": 3
                }
            }
        ]
    }))
}

async fn services_handler(State(state): State<Arc<AppState>>) -> Json<Value> {
    let registry = state.registry.read().await;
    let services = registry.get_all_services();

    let service_list: Vec<Value> = services
        .values()
        .map(|service| {
            json!({
                "name": service.name(),
                "description": service.description(),
                "input_schema": service.input_schema()
            })
        })
        .collect();

    Json(json!({
        "total": services.len(),
        "services": service_list
    }))
}

async fn health_handler() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "MCP Rust Services Framework",
        "uptime": "running"
    }))
}

// ========== MAIN APPLICATION ==========

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Create application state
    let state = Arc::new(AppState::new());

    // Create router
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/mcp", post(json_rpc_handler))
        .route("/services", get(services_handler))
        .route("/health", get(health_handler))
        .with_state(state.clone())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Print startup information
    println!("🚀 Starting MCP Rust Services Framework...");
    println!("📡 Endpoint: http://localhost:8000/mcp");
    println!("🔧 Framework Features:");
    println!("   ✅ Easy service registration");
    println!("   ✅ Automatic MCP tool generation");
    println!("   ✅ Direct service calls");
    println!("   ✅ Built-in error handling");
    println!("   ✅ JSON Schema validation");
    println!("   ✅ Async/await support");
    println!("");

    let registry = state.registry.read().await;
    let services = registry.get_all_services();
    println!("📚 Available Services ({}):", services.len());
    for (name, service) in services {
        println!("   - {}: {}", name, service.description());
    }

    println!("");
    println!("🔗 Available Methods:");
    let methods = vec![
        "initialize",
        "tools/list",
        "tools/call",
        "server/info",
        "echo",
        "get_current_time",
        "ping",
    ];
    for method in methods {
        println!("   - {}", method);
    }

    println!("");
    println!("🔗 Endpoints:");
    println!("   - GET  / (framework info)");
    println!("   - GET  /services (service list)");
    println!("   - GET  /health (health check)");
    println!("   - POST /mcp (JSON-RPC endpoint)");
    println!("");

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;
    info!("🎯 Server listening on http://127.0.0.1:8000");

    axum::serve(listener, app).await?;

    Ok(())
}
