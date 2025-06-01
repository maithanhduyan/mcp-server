# Use Raspberry Pi OS base image
FROM arm32v7/node:14

# Set working directory
WORKDIR /usr/src/app

# Copy package.json and install dependencies
COPY package.json .
RUN npm install

# Copy the rest of the application
COPY . .

# Create a startup script that can handle both MCP and HTTP modes
RUN echo '#!/bin/bash\nif [ "$MCP_MODE" = "true" ]; then\n  echo "Starting MCP Server..."\n  exec node src/mcp-server-cjs.js\nelse\n  echo "Starting HTTP Server..."\n  exec node src/server.js\nfi' > /usr/src/app/start.sh && chmod +x /usr/src/app/start.sh

# Expose the port for HTTP mode (MCP uses stdin/stdout)
EXPOSE 3000

# Start in MCP mode by default
ENV MCP_MODE=true
CMD ["/usr/src/app/start.sh"]
