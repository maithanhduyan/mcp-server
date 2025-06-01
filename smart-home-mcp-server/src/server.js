const express = require('express');
const bodyParser = require('body-parser');
const gpioRoutes = require('./routes/gpioRoutes');
const temperatureRoutes = require('./routes/temperatureRoutes');
const relayRoutes = require('./routes/relayRoutes');
const pumpRoutes = require('./routes/pumpRoutes');
const lightingRoutes = require('./routes/lightingRoutes');

const app = express();
const PORT = process.env.PORT || 3000;

app.use(bodyParser.json());

app.use('/gpio', gpioRoutes);
app.use('/temperature', temperatureRoutes);
app.use('/relay', relayRoutes);
app.use('/pump', pumpRoutes);
app.use('/lighting', lightingRoutes);

app.listen(PORT, () => {
    console.log(`Smart Home MCP Server is running on port ${PORT}`);
});