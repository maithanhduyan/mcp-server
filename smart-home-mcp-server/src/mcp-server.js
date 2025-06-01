#!/usr/bin/env node

/**
 * Smart Home MCP Server
 * Model Context Protocol server for smart home automation
 */

import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import {
  CallToolRequestSchema,
  ErrorCode,
  ListToolsRequestSchema,
  McpError,
} from '@modelcontextprotocol/sdk/types.js';

// Import controllers based on environment
let GpioController;
try {
  // Try to import real GPIO controller for Raspberry Pi
  GpioController = require('./controllers/gpioController.js');
} catch (error) {
  // Fall back to mock controller for development/Windows
  console.log('Using mock GPIO controller (development mode)');
  GpioController = require('./controllers/mockGpioController.js');
}

class SmartHomeMCPServer {
  constructor() {
    this.server = new Server(
      {
        name: 'smart-home-mcp-server',
        version: '1.0.0',
      },
      {
        capabilities: {
          tools: {},
        },
      }
    );

    this.gpioController = new GpioController();
    this.setupToolHandlers();
    this.setupErrorHandling();
  }

  setupToolHandlers() {
    // List available tools
    this.server.setRequestHandler(ListToolsRequestSchema, async () => {
      return {
        tools: [
          {
            name: 'gpio_read_pin',
            description: 'Read the current state of a GPIO pin',
            inputSchema: {
              type: 'object',
              properties: {
                pin: {
                  type: 'number',
                  description: 'GPIO pin number to read',
                  minimum: 0,
                  maximum: 40
                }
              },
              required: ['pin']
            }
          },
          {
            name: 'gpio_write_pin',
            description: 'Set the state of a GPIO pin (HIGH/LOW)',
            inputSchema: {
              type: 'object',
              properties: {
                pin: {
                  type: 'number',
                  description: 'GPIO pin number to control',
                  minimum: 0,
                  maximum: 40
                },
                value: {
                  type: 'number',
                  description: 'Pin value: 0 for LOW, 1 for HIGH',
                  enum: [0, 1]
                }
              },
              required: ['pin', 'value']
            }
          },
          {
            name: 'gpio_setup_pin',
            description: 'Setup a GPIO pin as input or output',
            inputSchema: {
              type: 'object',
              properties: {
                pin: {
                  type: 'number',
                  description: 'GPIO pin number to setup',
                  minimum: 0,
                  maximum: 40
                },
                direction: {
                  type: 'string',
                  description: 'Pin direction: in for input, out for output',
                  enum: ['in', 'out']
                }
              },
              required: ['pin', 'direction']
            }
          },
          {
            name: 'gpio_list_pins',
            description: 'List all configured GPIO pins and their current states',
            inputSchema: {
              type: 'object',
              properties: {}
            }
          },
          {
            name: 'control_light',
            description: 'Control smart home lighting',
            inputSchema: {
              type: 'object',
              properties: {
                action: {
                  type: 'string',
                  description: 'Light control action',
                  enum: ['on', 'off', 'toggle', 'dim']
                },
                pin: {
                  type: 'number',
                  description: 'GPIO pin connected to the light relay',
                  minimum: 0,
                  maximum: 40
                },
                brightness: {
                  type: 'number',
                  description: 'Brightness level (0-100) for dimming',
                  minimum: 0,
                  maximum: 100
                }
              },
              required: ['action', 'pin']
            }
          },
          {
            name: 'control_pump',
            description: 'Control water pump or irrigation system',
            inputSchema: {
              type: 'object',
              properties: {
                action: {
                  type: 'string',
                  description: 'Pump control action',
                  enum: ['start', 'stop', 'status']
                },
                pin: {
                  type: 'number',
                  description: 'GPIO pin connected to the pump relay',
                  minimum: 0,
                  maximum: 40
                },
                duration: {
                  type: 'number',
                  description: 'Duration in seconds for timed operation',
                  minimum: 1
                }
              },
              required: ['action', 'pin']
            }
          }
        ]
      };
    });

    // Handle tool calls
    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params;

      try {
        switch (name) {
          case 'gpio_read_pin':
            return await this.handleGpioRead(args);
          
          case 'gpio_write_pin':
            return await this.handleGpioWrite(args);
          
          case 'gpio_setup_pin':
            return await this.handleGpioSetup(args);
          
          case 'gpio_list_pins':
            return await this.handleGpioList(args);
          
          case 'control_light':
            return await this.handleLightControl(args);
          
          case 'control_pump':
            return await this.handlePumpControl(args);
          
          default:
            throw new McpError(
              ErrorCode.MethodNotFound,
              `Unknown tool: ${name}`
            );
        }
      } catch (error) {
        console.error(`Error handling tool ${name}:`, error);
        throw new McpError(
          ErrorCode.InternalError,
          `Tool execution failed: ${error.message}`
        );
      }
    });
  }

  async handleGpioRead(args) {
    const { pin } = args;
    console.log(`[MCP] Reading GPIO pin ${pin}`);
    
    const value = await this.gpioController.readPin(pin);
    
    return {
      content: [
        {
          type: 'text',
          text: `GPIO pin ${pin} current state: ${value === 1 ? 'HIGH' : 'LOW'} (${value})`
        }
      ]
    };
  }

  async handleGpioWrite(args) {
    const { pin, value } = args;
    console.log(`[MCP] Writing GPIO pin ${pin} to ${value}`);
    
    await this.gpioController.writePin(pin, value);
    
    return {
      content: [
        {
          type: 'text',
          text: `GPIO pin ${pin} set to ${value === 1 ? 'HIGH' : 'LOW'} (${value})`
        }
      ]
    };
  }

  async handleGpioSetup(args) {
    const { pin, direction } = args;
    console.log(`[MCP] Setting up GPIO pin ${pin} as ${direction}`);
    
    await this.gpioController.setupPin(pin, direction);
    
    return {
      content: [
        {
          type: 'text',
          text: `GPIO pin ${pin} configured as ${direction.toUpperCase()}`
        }
      ]
    };
  }

  async handleGpioList(args) {
    console.log('[MCP] Listing all GPIO pins');
    
    const status = await this.gpioController.getAllPinStatus();
    
    return {
      content: [
        {
          type: 'text',
          text: `GPIO Pin Status:\n${JSON.stringify(status, null, 2)}`
        }
      ]
    };
  }

  async handleLightControl(args) {
    const { action, pin, brightness } = args;
    console.log(`[MCP] Light control: ${action} on pin ${pin}`);
    
    let result;
    switch (action) {
      case 'on':
        await this.gpioController.writePin(pin, 1);
        result = `Light on pin ${pin} turned ON`;
        break;
      case 'off':
        await this.gpioController.writePin(pin, 0);
        result = `Light on pin ${pin} turned OFF`;
        break;
      case 'toggle':
        const currentState = await this.gpioController.readPin(pin);
        const newState = currentState === 1 ? 0 : 1;
        await this.gpioController.writePin(pin, newState);
        result = `Light on pin ${pin} toggled to ${newState === 1 ? 'ON' : 'OFF'}`;
        break;
      case 'dim':
        if (brightness !== undefined) {
          // For simplicity, treat dimming as on/off based on brightness
          const dimValue = brightness > 50 ? 1 : 0;
          await this.gpioController.writePin(pin, dimValue);
          result = `Light on pin ${pin} dimmed to ${brightness}% (${dimValue === 1 ? 'ON' : 'OFF'})`;
        } else {
          throw new Error('Brightness value required for dimming');
        }
        break;
    }
    
    return {
      content: [
        {
          type: 'text',
          text: result
        }
      ]
    };
  }

  async handlePumpControl(args) {
    const { action, pin, duration } = args;
    console.log(`[MCP] Pump control: ${action} on pin ${pin}`);
    
    let result;
    switch (action) {
      case 'start':
        await this.gpioController.writePin(pin, 1);
        result = `Pump on pin ${pin} started`;
        
        if (duration) {
          // Set a timer to stop the pump after duration
          setTimeout(async () => {
            await this.gpioController.writePin(pin, 0);
            console.log(`[MCP] Pump on pin ${pin} stopped after ${duration} seconds`);
          }, duration * 1000);
          result += ` for ${duration} seconds`;
        }
        break;
      case 'stop':
        await this.gpioController.writePin(pin, 0);
        result = `Pump on pin ${pin} stopped`;
        break;
      case 'status':
        const currentState = await this.gpioController.readPin(pin);
        result = `Pump on pin ${pin} is ${currentState === 1 ? 'RUNNING' : 'STOPPED'}`;
        break;
    }
    
    return {
      content: [
        {
          type: 'text',
          text: result
        }
      ]
    };
  }

  setupErrorHandling() {
    this.server.onerror = (error) => {
      console.error('[MCP Server Error]:', error);
    };

    process.on('SIGINT', async () => {
      console.log('\n[MCP] Shutting down server...');
      await this.server.close();
      process.exit(0);
    });
  }

  async run() {
    console.log('[MCP] Starting Smart Home MCP Server...');
    console.log('[MCP] Server capabilities:', this.server.getCapabilities());
    
    const transport = new StdioServerTransport();
    await this.server.connect(transport);
    
    console.log('[MCP] Smart Home MCP Server running and ready for connections');
  }
}

// Start the server
async function main() {
  try {
    const server = new SmartHomeMCPServer();
    await server.run();
  } catch (error) {
    console.error('[MCP] Failed to start server:', error);
    process.exit(1);
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch(console.error);
}

export { SmartHomeMCPServer };
