const express = require('express');
const bodyParser = require('body-parser');
const setGpioRoutes = require('./routes/gpioRoutes');

const app = express();
const PORT = process.env.PORT || 3000;

app.use(bodyParser.json());

// Set up routes
setGpioRoutes(app);

// Add a simple health check endpoint
app.get('/health', (req, res) => {
    res.json({ status: 'OK', message: 'Smart Home MCP Server is running' });
});

app.listen(PORT, () => {
    console.log(`Smart Home MCP Server is running on port ${PORT}`);
});