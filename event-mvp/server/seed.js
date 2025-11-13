const mongoose = require('mongoose');
const dotenv = require('dotenv');
const connectDB = require('./config/db');
const Event = require('./models/Event');

dotenv.config();
connectDB();

const events = [
  { title: 'Fiesta Universitaria', description: 'Tardeo para estudiantes', date: new Date('2025-10-31T18:00:00Z') },
  { title: 'Concierto Indie', description: 'Bandas locales', date: new Date('2025-11-20T20:00:00Z') }
];

const importData = async () => {
  try {
    await Event.deleteMany();
    await Event.insertMany(events);
    console.log('✅ Eventos importados exitosamente!');
    process.exit();
  } catch (error) {
    console.error(`❌ Error: ${error.message}`);
    process.exit(1);
  }
};

importData();
