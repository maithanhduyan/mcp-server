// This file defines the routes for relay control, linking them to the RelayController methods.

const express = require('express');
const RelayController = require('../controllers/relayController');

const router = express.Router();
const relayController = new RelayController();

router.post('/relay/on', (req, res) => {
    relayController.turnOn(req.body.relayId)
        .then(() => res.status(200).send({ message: 'Relay turned on' }))
        .catch(err => res.status(500).send({ error: err.message }));
});

router.post('/relay/off', (req, res) => {
    relayController.turnOff(req.body.relayId)
        .then(() => res.status(200).send({ message: 'Relay turned off' }))
        .catch(err => res.status(500).send({ error: err.message }));
});

router.get('/relay/status/:relayId', (req, res) => {
    relayController.getStatus(req.params.relayId)
        .then(status => res.status(200).send({ status }))
        .catch(err => res.status(500).send({ error: err.message }));
});

module.exports = router;