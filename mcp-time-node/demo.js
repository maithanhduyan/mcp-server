#!/usr/bin/env node

// Simple demo script to test MCP Time Server functionality
console.log('🕐 MCP Time Server Demo\n');

// Test 1: List tools
console.log('1. Testing tools list...');
console.log('Run: echo \'{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}\' | node server.js\n');

// Test 2: Get current time
console.log('2. Testing get current time...');
console.log('Run: echo \'{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_current_time","arguments":{"timezone":"UTC"}}}\' | node server.js\n');

// Test 3: Convert time
console.log('3. Testing time conversion...');
console.log('Run: echo \'{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"convert_time","arguments":{"time":"14:30","source_timezone":"Asia/Ho_Chi_Minh","target_timezone":"America/New_York"}}}\' | node server.js\n');

console.log('🎉 Demo commands shown above. You can run them manually to test the server!');
console.log('\nUsage Options:');
console.log('- Node.js: node server.js');
console.log('- Docker: docker run -i --rm mcp-time-server');
console.log('- VS Code MCP: Use .vscode/mcp.json configuration');
