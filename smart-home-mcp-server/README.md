# Smart Home MCP Server

This project implements a Model Context Protocol (MCP) server designed for smart home systems using a Raspberry Pi. The server allows for the control and monitoring of various home automation functionalities, including GPIO operations, temperature readings, relay control, pump management, and lighting adjustments.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Controllers](#controllers)
- [Routes](#routes)
- [Utilities](#utilities)
- [Environment Variables](#environment-variables)
- [Contributing](#contributing)
- [License](#license)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/smart-home-mcp-server.git
   cd smart-home-mcp-server
   ```

2. Install the dependencies:
   ```
   npm install
   ```

3. Set up your environment variables in the `.env` file.

## Usage

To start the MCP server, run the following command:
```
node src/server.js
```

The server will listen for requests related to smart home functionalities.

## Features

- Control GPIO pins for various devices.
- Read temperature data from sensors.
- Manage relay states for connected devices.
- Start and stop pumps based on commands.
- Control lighting, including brightness adjustments.

## Controllers

- **GpioController**: Handles GPIO pin operations.
- **TemperatureController**: Manages temperature sensor readings.
- **RelayController**: Controls the state of relays.
- **PumpController**: Manages pump operations.
- **LightingController**: Controls lighting features.

## Routes

- **GPIO Routes**: Define routes for GPIO operations.
- **Temperature Routes**: Define routes for temperature data retrieval.
- **Relay Routes**: Define routes for relay control.
- **Pump Routes**: Define routes for pump control.
- **Lighting Routes**: Define routes for lighting control.

## Utilities

- **gpioUtils**: Contains utility functions for GPIO operations.

## Environment Variables

The project uses a `.env` file to manage environment variables, including server configuration and GPIO pin mappings.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.