// gpioUtils.js
const Gpio = require('onoff').Gpio;

const gpioUtils = {
    setupPin: function(pinNumber, direction) {
        const pin = new Gpio(pinNumber, direction);
        return pin;
    },

    readPin: function(pin) {
        return new Promise((resolve, reject) => {
            pin.read((err, value) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(value);
                }
            });
        });
    },

    writePin: function(pin, value) {
        return new Promise((resolve, reject) => {
            pin.write(value, (err) => {
                if (err) {
                    reject(err);
                } else {
                    resolve();
                }
            });
        });
    },

    cleanupPin: function(pin) {
        pin.unexport();
    }
};

module.exports = gpioUtils;