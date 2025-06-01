const express = require('express');
const PumpController = require('../controllers/pumpController');

const router = express.Router();
const pumpController = new PumpController();

function setPumpRoutes(app) {
    router.post('/start', (req, res) => {
        pumpController.startPump(req.body)
            .then(result => res.status(200).json(result))
            .catch(error => res.status(500).json({ error: error.message }));
    });

    router.post('/stop', (req, res) => {
        pumpController.stopPump(req.body)
            .then(result => res.status(200).json(result))
            .catch(error => res.status(500).json({ error: error.message }));
    });

    app.use('/api/pump', router);
}

module.exports = setPumpRoutes;