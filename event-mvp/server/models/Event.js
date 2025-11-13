const mongoose = require('mongoose');

const eventSchema = mongoose.Schema(
  {
    title: { type: String, required: [true, 'Por favor añada un título'] },
    description: { type: String, required: [true, 'Por favor añada una descripción'] },
    date: { type: Date, required: [true, 'Por favor añada una fecha'] }
  },
  { timestamps: true }
);

module.exports = mongoose.model('Event', eventSchema);
