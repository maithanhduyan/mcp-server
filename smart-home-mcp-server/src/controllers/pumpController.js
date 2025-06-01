class PumpController {
    constructor() {
        this.isPumpOn = false; // Initial state of the pump
    }

    startPump() {
        if (!this.isPumpOn) {
            // Logic to start the pump
            this.isPumpOn = true;
            console.log("Pump started.");
            // Add GPIO control logic here
        } else {
            console.log("Pump is already running.");
        }
    }

    stopPump() {
        if (this.isPumpOn) {
            // Logic to stop the pump
            this.isPumpOn = false;
            console.log("Pump stopped.");
            // Add GPIO control logic here
        } else {
            console.log("Pump is already stopped.");
        }
    }

    getPumpStatus() {
        return this.isPumpOn ? "Pump is running." : "Pump is stopped.";
    }
}

module.exports = PumpController;