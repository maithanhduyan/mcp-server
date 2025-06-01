class LightingController {
    constructor() {
        this.lights = {}; // Store light states
    }

    turnOn(lightId) {
        this.lights[lightId] = true;
        // Code to interact with GPIO to turn on the light
        console.log(`Light ${lightId} turned on.`);
    }

    turnOff(lightId) {
        this.lights[lightId] = false;
        // Code to interact with GPIO to turn off the light
        console.log(`Light ${lightId} turned off.`);
    }

    setBrightness(lightId, level) {
        if (level < 0 || level > 100) {
            throw new Error("Brightness level must be between 0 and 100.");
        }
        this.lights[lightId] = level;
        // Code to interact with GPIO to set brightness
        console.log(`Brightness of light ${lightId} set to ${level}.`);
    }

    getLightStatus(lightId) {
        return this.lights[lightId] || false; // Return false if lightId is not found
    }
}

module.exports = LightingController;