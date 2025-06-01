// Mock GPIO Controller for Windows testing
class MockGpioController {
    constructor() {
        this.pins = {}; // Object to hold pin states
        console.log('MockGpioController initialized (Windows-compatible)'); // Debugging log
    }

    // Method to initialize a GPIO pin (mocked)
    initPin(pinNumber, direction) {
        if (!this.pins[pinNumber]) {
            console.log(`Mock: Initializing pin ${pinNumber} with direction ${direction}`);
            this.pins[pinNumber] = { direction: direction, value: 0 };
        }
    }    // Method to read the state of a GPIO pin (mocked)
    readPin(pinNumber) {
        if (this.pins[pinNumber]) {
            console.log(`Mock: Reading state of pin ${pinNumber}`);
            return Promise.resolve(this.pins[pinNumber].value);
        }
        return Promise.reject(new Error(`Pin ${pinNumber} is not initialized.`));
    }

    // Method to write a value to a GPIO pin (mocked)
    writePin(pinNumber, value) {
        if (this.pins[pinNumber]) {
            console.log(`Mock: Writing value ${value} to pin ${pinNumber}`);
            this.pins[pinNumber].value = value;
            return Promise.resolve();
        } else {
            return Promise.reject(new Error(`Pin ${pinNumber} is not initialized.`));
        }
    }

    // Method to cleanup GPIO pins (mocked)
    cleanup() {
        console.log('Mock: Cleaning up GPIO pins');
        this.pins = {};
    }

    // Method to get the status of a GPIO pin (for HTTP routes)
    getStatus(req, res) {
        try {
            const pinNumber = parseInt(req.params.pin);
            console.log(`Mock: Getting status for pin ${pinNumber}`);
            
            if (!this.pins[pinNumber]) {
                // Initialize pin as input if not already initialized
                this.initPin(pinNumber, 'in');
            }
            
            const status = this.readPin(pinNumber);
            res.json({ pin: pinNumber, status: status, mock: true });
        } catch (error) {
            console.error('Mock: Error getting pin status:', error.message);
            res.status(500).json({ error: error.message, mock: true });
        }
    }

    // Method to set a GPIO pin value (for HTTP routes)
    setPin(req, res) {
        try {
            const pinNumber = parseInt(req.params.pin);
            const value = parseInt(req.body.value);
            console.log(`Mock: Setting pin ${pinNumber} to value ${value}`);
            
            if (!this.pins[pinNumber]) {
                // Initialize pin as output if not already initialized
                this.initPin(pinNumber, 'out');
            }
            
            this.writePin(pinNumber, value);
            res.json({ pin: pinNumber, value: value, message: 'Pin set successfully (mock)', mock: true });
        } catch (error) {
            console.error('Mock: Error setting pin value:', error.message);
            res.status(500).json({ error: error.message, mock: true });
        }
    }

    // Method to setup a GPIO pin (for MCP)
    setupPin(pinNumber, direction) {
        console.log(`Mock: Setting up pin ${pinNumber} as ${direction}`);
        this.initPin(pinNumber, direction);
        return Promise.resolve(); // Make it async compatible
    }

    // Method to get all pin status (for MCP)
    getAllPinStatus() {
        console.log('Mock: Getting all pin status');
        const status = {};
        Object.keys(this.pins).forEach(pin => {
            status[pin] = {
                direction: this.pins[pin].direction,
                value: this.pins[pin].value
            };
        });
        return Promise.resolve(status); // Make it async compatible
    }
}

module.exports = MockGpioController;
