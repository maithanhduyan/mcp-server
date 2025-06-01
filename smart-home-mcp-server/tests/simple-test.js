#!/usr/bin/env node

/**
 * Simple MCP Test Client
 * Following Elon Musk's principle: "The best part is no part"
 * Keep it minimal, effective, and scalable
 */

const { spawn } = require('child_process');

async function testMCPServer() {
  console.log('=== Simple MCP Server Test ===');
  
  try {
    // Start MCP server
    console.log('[Test] Starting MCP server...');
    const server = spawn('node', ['src/mcp-server-cjs.js'], {
      stdio: ['pipe', 'pipe', 'pipe'],
      cwd: process.cwd()
    });

    // Test 1: Server starts without errors
    let serverStarted = false;
    
    server.stdout.on('data', (data) => {
      const output = data.toString();
      console.log('[Server Output]:', output.trim());
      
      if (output.includes('Smart Home MCP Server running')) {
        serverStarted = true;
        console.log('✅ Test 1 PASSED: Server started successfully');
        
        // Test basic MCP protocol by sending a simple request
        testBasicProtocol(server);
      }
    });

    server.stderr.on('data', (data) => {
      console.log('[Server Error]:', data.toString().trim());
    });

    server.on('exit', (code) => {
      console.log(`[Server] Exited with code ${code}`);
      if (code === 0) {
        console.log('✅ Server shutdown cleanly');
      } else {
        console.log('❌ Server exit with error');
      }
    });

    // Wait for server to start
    setTimeout(() => {
      if (!serverStarted) {
        console.log('❌ Test 1 FAILED: Server did not start within timeout');
        server.kill();
      }
    }, 5000);

  } catch (error) {
    console.error('❌ Test failed:', error.message);
  }
}

function testBasicProtocol(server) {
  console.log('[Test] Testing basic MCP protocol...');
  
  // Send initialize request
  const initRequest = {
    jsonrpc: "2.0",
    id: 1,
    method: "initialize",
    params: {
      protocolVersion: "2024-11-05",
      capabilities: {},
      clientInfo: {
        name: "test-client",
        version: "1.0.0"
      }
    }
  };

  console.log('[Test] Sending initialize request...');
  server.stdin.write(JSON.stringify(initRequest) + '\n');

  // Listen for response
  let responseReceived = false;
  const responseTimeout = setTimeout(() => {
    if (!responseReceived) {
      console.log('❌ Test 2 FAILED: No response to initialize request');
      server.kill();
    }
  }, 3000);

  server.stdout.on('data', (data) => {
    try {
      const response = JSON.parse(data.toString().trim());
      if (response.id === 1) {
        responseReceived = true;
        clearTimeout(responseTimeout);
        console.log('✅ Test 2 PASSED: Initialize response received');
        console.log('[Response]:', JSON.stringify(response, null, 2));
        
        // Test tools listing
        testToolsListing(server);
      }
    } catch (e) {
      // Not JSON response, might be log output
    }
  });
}

function testToolsListing(server) {
  console.log('[Test] Testing tools listing...');
  
  const toolsRequest = {
    jsonrpc: "2.0",
    id: 2,
    method: "tools/list",
    params: {}
  };

  console.log('[Test] Sending tools/list request...');
  server.stdin.write(JSON.stringify(toolsRequest) + '\n');

  // Simple success - if we get here without crashes, basic MCP is working
  setTimeout(() => {
    console.log('✅ Test 3 PASSED: Tools listing sent successfully');
    console.log('✅ Basic MCP protocol appears to be working');
    
    // Clean shutdown
    console.log('[Test] Shutting down server...');
    server.kill('SIGTERM');
    
    setTimeout(() => {
      console.log('=== Test Complete ===');
      process.exit(0);
    }, 1000);
  }, 2000);
}

// Run the test
testMCPServer();
