class RelayController {
    constructor() {
        this.relayStates = {};
    }

    // Method to turn on a relay
    turnOn(relayId) {
        this.relayStates[relayId] = true;
        // Code to interact with GPIO to turn on the relay
        console.log(`Relay ${relayId} is turned ON`);
    }

    // Method to turn off a relay
    turnOff(relayId) {
        this.relayStates[relayId] = false;
        // Code to interact with GPIO to turn off the relay
        console.log(`Relay ${relayId} is turned OFF`);
    }

    // Method to toggle a relay state
    toggle(relayId) {
        if (this.relayStates[relayId]) {
            this.turnOff(relayId);
        } else {
            this.turnOn(relayId);
        }
    }

    // Method to get the current state of a relay
    getState(relayId) {
        return this.relayStates[relayId] || false;
    }
}

module.exports = RelayController;