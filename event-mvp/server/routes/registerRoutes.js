const express = require('express');
const router = express.Router();
const { createRegistration } = require('../controllers/registerController');

router.route('/').post(createRegistration);

module.exports = router;