#!/usr/bin/env node

/**
 * MCP Client Test Script
 * Test the Smart Home MCP Server functionality
 */

const { Client } = require('@modelcontextprotocol/sdk/client/index.js');
const { StdioClientTransport } = require('@modelcontextprotocol/sdk/client/stdio.js');
const { spawn } = require('child_process');

class MCPTestClient {
  constructor() {
    this.client = new Client(
      {
        name: 'test-client',
        version: '1.0.0',
      },
      {
        capabilities: {},
      }
    );
  }

  async connect() {
    console.log('[Test] Starting MCP server process...');
    
    // Start the MCP server as a child process
    const serverProcess = spawn('node', ['src/mcp-server-cjs.js'], {
      stdio: ['pipe', 'pipe', 'pipe'],
      cwd: process.cwd()
    });

    // Handle server errors
    serverProcess.stderr.on('data', (data) => {
      console.log('[Server Error]:', data.toString());
    });

    serverProcess.on('exit', (code) => {
      console.log(`[Server] Process exited with code ${code}`);
    });

    // Create transport using server's stdin/stdout
    const transport = new StdioClientTransport({
      stdin: serverProcess.stdin,
      stdout: serverProcess.stdout
    });

    console.log('[Test] Connecting to MCP server...');
    await this.client.connect(transport);
    
    return serverProcess;
  }

  async testListTools() {
    console.log('\n[Test] Testing list tools...');
    try {
      const response = await this.client.request(
        { method: 'tools/list' },
        { method: 'tools/list' }
      );
      console.log('[Test] Available tools:');
      response.tools.forEach(tool => {
        console.log(`  - ${tool.name}: ${tool.description}`);
      });
      return response.tools;
    } catch (error) {
      console.error('[Test] Error listing tools:', error);
      return [];
    }
  }

  async testGpioOperations() {
    console.log('\n[Test] Testing GPIO operations...');
    
    try {
      // Test GPIO setup
      console.log('[Test] Setting up GPIO pin 18 as output...');
      const setupResponse = await this.client.request(
        { method: 'tools/call' },
        {
          method: 'tools/call',
          params: {
            name: 'gpio_setup_pin',
            arguments: {
              pin: 18,
              direction: 'out'
            }
          }
        }
      );
      console.log('[Test] Setup result:', setupResponse.content[0].text);

      // Test GPIO write
      console.log('[Test] Writing HIGH to GPIO pin 18...');
      const writeResponse = await this.client.request(
        { method: 'tools/call' },
        {
          method: 'tools/call',
          params: {
            name: 'gpio_write_pin',
            arguments: {
              pin: 18,
              value: 1
            }
          }
        }
      );
      console.log('[Test] Write result:', writeResponse.content[0].text);

      // Test GPIO read
      console.log('[Test] Reading GPIO pin 18...');
      const readResponse = await this.client.request(
        { method: 'tools/call' },
        {
          method: 'tools/call',
          params: {
            name: 'gpio_read_pin',
            arguments: {
              pin: 18
            }
          }
        }
      );
      console.log('[Test] Read result:', readResponse.content[0].text);

    } catch (error) {
      console.error('[Test] Error in GPIO operations:', error);
    }
  }

  async testLightControl() {
    console.log('\n[Test] Testing light control...');
    
    try {
      // Test light on
      console.log('[Test] Turning on light on pin 18...');
      const onResponse = await this.client.request(
        { method: 'tools/call' },
        {
          method: 'tools/call',
          params: {
            name: 'control_light',
            arguments: {
              action: 'on',
              pin: 18
            }
          }
        }
      );
      console.log('[Test] Light on result:', onResponse.content[0].text);

      // Wait a moment
      await new Promise(resolve => setTimeout(resolve, 1000));

      // Test light off
      console.log('[Test] Turning off light on pin 18...');
      const offResponse = await this.client.request(
        { method: 'tools/call' },
        {
          method: 'tools/call',
          params: {
            name: 'control_light',
            arguments: {
              action: 'off',
              pin: 18
            }
          }
        }
      );
      console.log('[Test] Light off result:', offResponse.content[0].text);

    } catch (error) {
      console.error('[Test] Error in light control:', error);
    }
  }

  async testPumpControl() {
    console.log('\n[Test] Testing pump control...');
    
    try {
      // Test pump start with duration
      console.log('[Test] Starting pump on pin 19 for 3 seconds...');
      const startResponse = await this.client.request(
        { method: 'tools/call' },
        {
          method: 'tools/call',
          params: {
            name: 'control_pump',
            arguments: {
              action: 'start',
              pin: 19,
              duration: 3
            }
          }
        }
      );
      console.log('[Test] Pump start result:', startResponse.content[0].text);

      // Check pump status
      console.log('[Test] Checking pump status...');
      const statusResponse = await this.client.request(
        { method: 'tools/call' },
        {
          method: 'tools/call',
          params: {
            name: 'control_pump',
            arguments: {
              action: 'status',
              pin: 19
            }
          }
        }
      );
      console.log('[Test] Pump status result:', statusResponse.content[0].text);

      // Wait for auto-stop
      console.log('[Test] Waiting for pump to auto-stop...');
      await new Promise(resolve => setTimeout(resolve, 4000));

      // Check status again
      const finalStatusResponse = await this.client.request(
        { method: 'tools/call' },
        {
          method: 'tools/call',
          params: {
            name: 'control_pump',
            arguments: {
              action: 'status',
              pin: 19
            }
          }
        }
      );
      console.log('[Test] Final pump status:', finalStatusResponse.content[0].text);

    } catch (error) {
      console.error('[Test] Error in pump control:', error);
    }
  }

  async testListPins() {
    console.log('\n[Test] Testing list all pins...');
    
    try {
      const listResponse = await this.client.request(
        { method: 'tools/call' },
        {
          method: 'tools/call',
          params: {
            name: 'gpio_list_pins',
            arguments: {}
          }
        }
      );
      console.log('[Test] All pins status:', listResponse.content[0].text);

    } catch (error) {
      console.error('[Test] Error listing pins:', error);
    }
  }

  async runAllTests() {
    let serverProcess;
    
    try {
      // Connect to server
      serverProcess = await this.connect();
      
      // Wait a moment for server to initialize
      await new Promise(resolve => setTimeout(resolve, 2000));

      // Run tests
      await this.testListTools();
      await this.testGpioOperations();
      await this.testLightControl();
      await this.testPumpControl();
      await this.testListPins();

      console.log('\n[Test] All tests completed successfully!');

    } catch (error) {
      console.error('[Test] Test suite failed:', error);
    } finally {
      // Clean up
      if (serverProcess) {
        console.log('\n[Test] Shutting down server...');
        serverProcess.kill('SIGTERM');
      }
      
      await this.client.close();
      console.log('[Test] Test client closed.');
    }
  }
}

// Run tests
async function main() {
  console.log('=== Smart Home MCP Server Test Suite ===\n');
  
  const testClient = new MCPTestClient();
  await testClient.runAllTests();
  
  console.log('\n=== Test Suite Complete ===');
  process.exit(0);
}

if (require.main === module) {
  main().catch(console.error);
}

module.exports = { MCPTestClient };
