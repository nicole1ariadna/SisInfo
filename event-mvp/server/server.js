require('dotenv').config();
const express = require('express');
const cors = require('cors');
const connectDB = require('./config/db');

const app = express();
connectDB();

app.use(cors());
app.use(express.json());

const PORT = process.env.PORT || 3000;

// Rutas
app.use('/api/events', require('./routes/eventRoutes'));
app.use('/api/register', require('./routes/registerRoutes'));

app.get('/', (req, res) => res.send('API de Gestión de Eventos está funcionando!'));

app.listen(PORT, () => {
  console.log(`Servidor corriendo en el puerto ${PORT}`);
});
