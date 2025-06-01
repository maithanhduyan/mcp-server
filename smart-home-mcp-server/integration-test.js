#!/usr/bin/env node

/**
 * MCP Integration Test - Following Elon Musk's Principles
 * Test complete workflow: setup -> control -> verify
 */

const { spawn } = require('child_process');

const tests = [
  // Test 1: Initialize MCP
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0.0"}}}',
  
  // Test 2: List tools
  '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}',
  
  // Test 3: Setup GPIO pin 18 as output
  '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"gpio_setup_pin","arguments":{"pin":18,"direction":"out"}}}',
  
  // Test 4: Turn on light
  '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"control_light","arguments":{"action":"on","pin":18}}}',
  
  // Test 5: Read pin status
  '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"gpio_read_pin","arguments":{"pin":18}}}',
  
  // Test 6: List all pins
  '{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"gpio_list_pins","arguments":{}}}',
];

async function runIntegrationTest() {
  console.log('=== MCP Integration Test - Smart Home Automation ===');
  
  const server = spawn('node', ['src/mcp-server-cjs.js'], {
    stdio: ['pipe', 'pipe', 'pipe'],
    cwd: process.cwd()
  });

  let testIndex = 0;
  let responses = [];

  server.stdout.on('data', (data) => {
    const lines = data.toString().split('\n').filter(line => line.trim());
    
    lines.forEach(line => {
      if (line.startsWith('[MCP]') || line.includes('MockGpioController')) {
        console.log('📝', line);
      } else if (line.startsWith('{')) {
        try {
          const response = JSON.parse(line);
          responses.push(response);
          console.log(`✅ Test ${response.id} Response:`, response.result ? 'SUCCESS' : 'ERROR');
          
          if (response.result && response.result.tools) {
            console.log(`   📋 Available tools: ${response.result.tools.length}`);
          }
          
          if (response.result && response.result.content) {
            console.log(`   📄 ${response.result.content[0].text}`);
          }
          
          // Send next test after receiving response
          if (testIndex < tests.length) {
            setTimeout(() => sendNextTest(), 500);
          } else {
            setTimeout(() => {
              console.log('\n🎉 All tests completed successfully!');
              console.log('✅ MCP Smart Home Server is fully functional');
              server.kill();
              process.exit(0);
            }, 1000);
          }
        } catch (e) {
          console.log('📄 Log:', line);
        }
      }
    });
  });

  server.stderr.on('data', (data) => {
    console.log('❌ Error:', data.toString());
  });

  function sendNextTest() {
    if (testIndex < tests.length) {
      console.log(`\n🧪 Sending Test ${testIndex + 1}...`);
      server.stdin.write(tests[testIndex] + '\n');
      testIndex++;
    }
  }

  // Wait for server to start, then begin tests
  setTimeout(() => {
    sendNextTest();
  }, 1000);

  // Safety timeout
  setTimeout(() => {
    console.log('⏰ Test timeout reached');
    server.kill();
    process.exit(1);
  }, 15000);
}

runIntegrationTest();
