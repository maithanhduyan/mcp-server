class GpioController {
    constructor() {
        const Gpio = require('onoff').Gpio; // Import the onoff library for GPIO control
        this.pins = {}; // Object to hold pin instances
    }

    // Method to initialize a GPIO pin
    initPin(pinNumber, direction) {
        if (!this.pins[pinNumber]) {
            this.pins[pinNumber] = new Gpio(pinNumber, direction);
        }
    }

    // Method to read the state of a GPIO pin
    readPin(pinNumber) {
        if (this.pins[pinNumber]) {
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
}

module.exports = GpioController;