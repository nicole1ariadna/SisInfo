const Registration = require('../models/Registration');
const Event = require('../models/Event');
const qrCode = require('qrcode');

const createRegistration = async (req, res) => {
  const { name, email, eventId } = req.body;

  if (!name || !email || !eventId) {
    return res.status(400).json({ message: 'Por favor complete todos los campos' });
  }

  try {
    const event = await Event.findById(eventId);
    if (!event) return res.status(404).json({ message: 'Evento no encontrado' });

    const existingRegistration = await Registration.findOne({ email, event: eventId });
    if (existingRegistration) {
      return res.status(400).json({ message: 'Este email ya está registrado para este evento' });
    }

    const registration = new Registration({ name, email, event: eventId });
    const newRegistration = await registration.save();

    // Generar QR (Data URL) usando el id del registro
    const registrationIdString = newRegistration._id.toString();
    const qrCodeDataUrl = await qrCode.toDataURL(registrationIdString);

    res.status(201).json({
      message: 'Registro exitoso',
      registrationId: newRegistration._id,
      qrCodeDataUrl
    });
  } catch (error) {
    console.error(error);
    res.status(500).json({ message: 'Error del servidor' });
  }
};

module.exports = { createRegistration };
