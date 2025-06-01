# Use official Node.js runtime as base image
FROM node:18-alpine

# Set working directory in container
WORKDIR /app

# Copy package.json and package-lock.json (if available)
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy server source code
COPY server.js ./

# Make server.js executable
RUN chmod +x server.js

# Expose the port (if needed for HTTP endpoints in future)
# EXPOSE 3000

# Create non-root user for security
RUN addgroup -g 1001 -S nodejs && \
    adduser -S mcp -u 1001

# Switch to non-root user
USER mcp

# Set the entrypoint
ENTRYPOINT ["node", "server.js"]

# Add labels for better container management
LABEL \
  org.opencontainers.image.title="MCP Time Server" \
  org.opencontainers.image.description="Model Context Protocol server for time-related operations" \
  org.opencontainers.image.version="1.0.0" \
  org.opencontainers.image.source="https://github.com/your-username/mcp-time-server"