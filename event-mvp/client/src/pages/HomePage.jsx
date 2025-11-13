import axios from 'axios';
import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';

const HomePage = () => {
  const [events, setEvents] = useState([]); // CORREGIDO: inicializar con array
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchEvents = async () => {
      try {
        const response = await axios.get('http://localhost:3000/api/events');
        setEvents(response.data);
      } catch (error) {
        console.error('Error al cargar eventos:', error);
      } finally {
        setLoading(false);
      }
    };
    fetchEvents();
  }, []); // CORREGIDO: array de dependencias vacío

  if (loading) return <div>Cargando eventos...</div>;

  return (
    <div className="event-list">
      <h2>Próximos Eventos</h2>
      {events.length === 0 ? (
        <p>No hay eventos por el momento.</p>
      ) : (
        events.map((event) => (
          <div key={event._id} className="event-card">
            <h3>{event.title}</h3>
            <p>{new Date(event.date).toLocaleDateString('es-ES')}</p>
            <p>{event.description.substring(0, 100)}...</p>
            <Link to={`/event/${event._id}`} className="btn">Ver Detalles y Registrarse</Link>
          </div>
        ))
      )}
    </div>
  );
};

export default HomePage;
