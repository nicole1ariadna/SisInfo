const mongoose = require('mongoose');

const registrationSchema = mongoose.Schema(
  {
    name: { type: String, required: [true, 'Por favor añade un nombre'] },
    email: { type: String, required: [true, 'Por favor añade un email'] },
    event: { type: mongoose.Schema.Types.ObjectId, required: true, ref: 'Event' }
  },
  { timestamps: true }
);

module.exports = mongoose.model('Registration', registrationSchema);
