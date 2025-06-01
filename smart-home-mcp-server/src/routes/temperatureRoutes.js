// This file defines the routes for temperature data retrieval, linking them to the TemperatureController methods.

const express = require('express');
const TemperatureController = require('../controllers/temperatureController');

const setTemperatureRoutes = (app) => {
    const router = express.Router();
    const temperatureController = new TemperatureController();

    router.get('/temperature', (req, res) => {
        temperatureController.getTemperature(req, res);
    });

    app.use('/api', router);
};

module.exports = setTemperatureRoutes;