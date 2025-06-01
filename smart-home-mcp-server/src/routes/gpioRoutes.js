// This file defines the routes for GPIO operations, linking them to the GpioController methods.

const express = require('express');
const GpioController = require('../controllers/gpioController');

const router = express.Router();
const gpioController = new GpioController();

router.get('/status/:pin', gpioController.getStatus.bind(gpioController));
router.post('/set/:pin', gpioController.setPin.bind(gpioController));

module.exports = function setGpioRoutes(app) {
    app.use('/api/gpio', router);
};