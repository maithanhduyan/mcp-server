class TemperatureController {
    constructor(sensor) {
        this.sensor = sensor; // Assume sensor is an object that interacts with the temperature sensor
    }

    async readTemperature() {
        try {
            const temperature = await this.sensor.getTemperature(); // Method to get temperature from the sensor
            return { success: true, temperature };
        } catch (error) {
            return { success: false, message: error.message };
        }
    }
}

module.exports = TemperatureController;