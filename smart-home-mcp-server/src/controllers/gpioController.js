class GpioController {
    constructor() {
        const Gpio = require('onoff').Gpio; // Import the onoff library for GPIO control
        this.pins = {}; // Object to hold pin instances
        console.log('GpioController initialized'); // Debugging log
    }

    // Method to initialize a GPIO pin
    initPin(pinNumber, direction) {
        if (!this.pins[pinNumber]) {
            console.log(`Initializing pin ${pinNumber} with direction ${direction}`); // Debugging log
            this.pins[pinNumber] = new Gpio(pinNumber, direction);
        }
    }

    // Method to read the state of a GPIO pin
    readPin(pinNumber) {
        if (this.pins[pinNumber]) {
            console.log(`Reading state of pin ${pinNumber}`); // Debugging log
            return this.pins[pinNumber].readSync(); // Read the pin state synchronously
        }
        throw new Error(`Pin ${pinNumber} is not initialized.`);
    }

    // Method to write a value to a GPIO pin
    writePin(pinNumber, value) {
        if (this.pins[pinNumber]) {
            this.pins[pinNumber].writeSync(value); // Write the value to the pin
        } else {
            throw new Error(`Pin ${pinNumber} is not initialized.`);
        }
    }

    // Method to cleanup GPIO pins
    cleanup() {
        for (const pin in this.pins) {
            this.pins[pin].unexport(); // Unexport the pin
        }
    }

    // Method to get the status of a GPIO pin (for HTTP routes)
    getStatus(req, res) {
        try {
            const pinNumber = parseInt(req.params.pin);
            console.log(`Getting status for pin ${pinNumber}`); // Debugging log
            
            if (!this.pins[pinNumber]) {
                // Initialize pin as input if not already initialized
                this.initPin(pinNumber, 'in');
            }
            
            const status = this.readPin(pinNumber);
            res.json({ pin: pinNumber, status: status });
        } catch (error) {
            console.error('Error getting pin status:', error.message); // Debugging log
            res.status(500).json({ error: error.message });
        }
    }

    // Method to set a GPIO pin value (for HTTP routes)
    setPin(req, res) {
        try {
            const pinNumber = parseInt(req.params.pin);
            const value = parseInt(req.body.value);
            console.log(`Setting pin ${pinNumber} to value ${value}`); // Debugging log
            
            if (!this.pins[pinNumber]) {
                // Initialize pin as output if not already initialized
                this.initPin(pinNumber, 'out');
            }
            
            this.writePin(pinNumber, value);
            res.json({ pin: pinNumber, value: value, message: 'Pin set successfully' });
        } catch (error) {
            console.error('Error setting pin value:', error.message); // Debugging log
            res.status(500).json({ error: error.message });
        }
    }
}

module.exports = GpioController;