// lightingRoutes.js
const express = require('express');
const LightingController = require('../controllers/lightingController');

const router = express.Router();
const lightingController = new LightingController();

function setLightingRoutes(app) {
    router.post('/lights/on', (req, res) => {
        lightingController.turnOn(req, res);
    });

    router.post('/lights/off', (req, res) => {
        lightingController.turnOff(req, res);
    });

    router.post('/lights/brightness', (req, res) => {
        lightingController.setBrightness(req, res);
    });

    app.use('/api/lighting', router);
}

module.exports = setLightingRoutes;