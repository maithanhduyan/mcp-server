#!/usr/bin/env node

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ErrorCode,
  ListToolsRequestSchema,
  McpError,
} from "@modelcontextprotocol/sdk/types.js";

class TimeServer {
  constructor() {
    this.server = new Server(
      {
        name: "time-server",
        version: "1.0.0",
      },
      {
        capabilities: {
          tools: {},
        },
      }
    );

    this.setupToolHandlers();
    
    // Error handling
    this.server.onerror = (error) => console.error("[MCP Error]", error);
    process.on("SIGINT", async () => {
      await this.server.close();
      process.exit(0);
    });
  }

  setupToolHandlers() {
    // List available tools
    this.server.setRequestHandler(ListToolsRequestSchema, async () => {
      return {
        tools: [
          {
            name: "get_current_time",
            description: "Get current time in a specific timezone",
            inputSchema: {
              type: "object",
              properties: {
                timezone: {
                  type: "string",
                  description: "IANA timezone name (e.g., 'America/New_York', 'Europe/London'). Use 'UTC' as local timezone if no timezone provided by the user.",
                },
              },
              required: ["timezone"],
            },
          },
          {
            name: "convert_time",
            description: "Convert time between timezones",
            inputSchema: {
              type: "object",
              properties: {
                time: {
                  type: "string",
                  description: "Time to convert in 24-hour format (HH:MM)",
                },
                source_timezone: {
                  type: "string",
                  description: "Source IANA timezone name (e.g., 'America/New_York', 'Europe/London'). Use 'UTC' as local timezone if no source timezone provided by the user.",
                },
                target_timezone: {
                  type: "string",
                  description: "Target IANA timezone name (e.g., 'Asia/Tokyo', 'America/San_Francisco'). Use 'UTC' as local timezone if no target timezone provided by the user.",
                },
              },
              required: ["time", "source_timezone", "target_timezone"],
            },
          },
        ],
      };
    });

    // Handle tool calls
    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      try {
        switch (request.params.name) {
          case "get_current_time":
            return await this.getCurrentTime(request.params.arguments);
          case "convert_time":
            return await this.convertTime(request.params.arguments);
          default:
            throw new McpError(
              ErrorCode.MethodNotFound,
              `Unknown tool: ${request.params.name}`
            );
        }
      } catch (error) {
        throw new McpError(
          ErrorCode.InternalError,
          `Tool execution failed: ${error.message}`
        );
      }
    });
  }

  async getCurrentTime(args) {
    const { timezone = "UTC" } = args;

    try {
      const now = new Date();
      
      // Validate timezone
      if (!this.isValidTimezone(timezone)) {
        throw new Error(`Invalid timezone: ${timezone}`);
      }

      // Format current time in the specified timezone
      const formatter = new Intl.DateTimeFormat("en-US", {
        timeZone: timezone,
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
        hour12: false,
      });

      const parts = formatter.formatToParts(now);
      const formattedParts = {};
      parts.forEach(part => {
        formattedParts[part.type] = part.value;
      });

      const dateTime = `${formattedParts.year}-${formattedParts.month}-${formattedParts.day} ${formattedParts.hour}:${formattedParts.minute}:${formattedParts.second}`;
      
      // Get timezone offset
      const tempDate = new Date();
      const utcTime = tempDate.getTime() + (tempDate.getTimezoneOffset() * 60000);
      const targetTime = new Date(utcTime + this.getTimezoneOffset(timezone, tempDate));
      const offset = this.formatTimezoneOffset(timezone, tempDate);

      return {
        content: [
          {
            type: "text",
            text: `Current time in ${timezone}: ${dateTime} (${offset})`,
          },
        ],
      };
    } catch (error) {
      return {
        content: [
          {
            type: "text",
            text: `Error getting current time: ${error.message}`,
          },
        ],
      };
    }
  }

  async convertTime(args) {
    const { time, source_timezone, target_timezone } = args;

    try {
      // Validate input
      if (!this.isValidTimeFormat(time)) {
        throw new Error("Time must be in HH:MM format (24-hour)");
      }

      if (!this.isValidTimezone(source_timezone)) {
        throw new Error(`Invalid source timezone: ${source_timezone}`);
      }

      if (!this.isValidTimezone(target_timezone)) {
        throw new Error(`Invalid target timezone: ${target_timezone}`);
      }

      // Parse time
      const [hours, minutes] = time.split(":").map(Number);
      
      // Create a date object for today in the source timezone
      const today = new Date();
      const year = today.getFullYear();
      const month = today.getMonth();
      const day = today.getDate();
      
      // Create date in source timezone
      const sourceDate = new Date();
      sourceDate.setFullYear(year, month, day);
      sourceDate.setHours(hours, minutes, 0, 0);
      
      // Convert to target timezone
      const sourceFormatter = new Intl.DateTimeFormat("en-CA", {
        timeZone: source_timezone,
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
      });

      const targetFormatter = new Intl.DateTimeFormat("en-CA", {
        timeZone: target_timezone,
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
      });

      // Calculate the time difference
      const utcTime = this.getUTCTime(year, month, day, hours, minutes, source_timezone);
      const targetTime = this.formatTimeInTimezone(utcTime, target_timezone);
      
      const sourceOffset = this.formatTimezoneOffset(source_timezone, sourceDate);
      const targetOffset = this.formatTimezoneOffset(target_timezone, sourceDate);

      return {
        content: [
          {
            type: "text",
            text: `Time conversion:
${time} in ${source_timezone} (${sourceOffset})
→ ${targetTime} in ${target_timezone} (${targetOffset})`,
          },
        ],
      };
    } catch (error) {
      return {
        content: [
          {
            type: "text",
            text: `Error converting time: ${error.message}`,
          },
        ],
      };
    }
  }

  isValidTimeFormat(time) {
    const timeRegex = /^([0-1]?[0-9]|2[0-3]):[0-5][0-9]$/;
    return timeRegex.test(time);
  }

  isValidTimezone(timezone) {
    try {
      Intl.DateTimeFormat(undefined, { timeZone: timezone });
      return true;
    } catch (error) {
      return false;
    }
  }

  getTimezoneOffset(timezone, date) {
    const utc1 = new Date(date.getTime() + (date.getTimezoneOffset() * 60000));
    const utc2 = new Date(utc1.toLocaleString("en-US", { timeZone: timezone }));
    return utc2.getTime() - utc1.getTime();
  }

  formatTimezoneOffset(timezone, date) {
    const formatter = new Intl.DateTimeFormat("en", {
      timeZone: timezone,
      timeZoneName: "longOffset",
    });
    
    const parts = formatter.formatToParts(date);
    const offset = parts.find(part => part.type === "timeZoneName")?.value || "";
    return offset;
  }

  getUTCTime(year, month, day, hours, minutes, timezone) {
    // Create a date string that represents the time in the source timezone
    const dateStr = `${year}-${String(month + 1).padStart(2, '0')}-${String(day).padStart(2, '0')}T${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:00`;
    
    // Parse this as if it's in the source timezone
    const tempDate = new Date(dateStr);
    const utcDate = new Date(tempDate.getTime() - this.getTimezoneOffset(timezone, tempDate));
    
    return utcDate;
  }

  formatTimeInTimezone(utcDate, timezone) {
    const formatter = new Intl.DateTimeFormat("en-CA", {
      timeZone: timezone,
      hour: "2-digit",
      minute: "2-digit",
      hour12: false,
    });
    
    return formatter.format(utcDate);
  }

  async run() {
    const transport = new StdioServerTransport();
    await this.server.connect(transport);
    console.error("MCP Time Server running on stdio");
  }
}

const server = new TimeServer();
server.run().catch(console.error);