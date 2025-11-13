import axios from 'axios';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

const RegistrationForm = ({ eventId }) => {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const navigate = useNavigate();

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError('');
    try {
      const response = await axios.post('http://localhost:3000/api/register', {
        name,
        email,
        eventId,
      });
      setLoading(false);
      navigate('/success', { state: { qrCodeDataUrl: response.data.qrCodeDataUrl } });
    } catch (err) {
      setLoading(false);
      setError(err.response?.data?.message || 'Error al registrarse');
    }
  };

  return (
    <form onSubmit={handleSubmit} className="registration-form">
      <h3>Regístrate en la Lista Free</h3>
      {error && <div className="error-message">{error}</div>}
      <div className="form-group">
        <label htmlFor="name">Nombre:</label>
        <input id="name" value={name} onChange={(e) => setName(e.target.value)} required />
      </div>
      <div className="form-group">
        <label htmlFor="email">Email:</label>
        <input id="email" type="email" value={email} onChange={(e) => setEmail(e.target.value)} required />
      </div>
      <button type="submit" className="btn" disabled={loading}>
        {loading ? 'Registrando...' : 'Registrarme'}
      </button>
    </form>
  );
};

export default RegistrationForm;
