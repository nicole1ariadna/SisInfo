import axios from 'axios';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import RegistrationForm from '../components/RegistrationForm';

const EventDetailPage = () => {
  const [event, setEvent] = useState(null);
  const [loading, setLoading] = useState(true);
  const { id } = useParams();

  useEffect(() => {
    const fetchEvent = async () => {
      try {
        const response = await axios.get(`http://localhost:3000/api/events/${id}`);
        setEvent(response.data);
      } catch (error) {
        console.error('Error al cargar el evento:', error);
      } finally {
        setLoading(false);
      }
    };
    fetchEvent();
  }, [id]);

  if (loading) return <div>Cargando evento...</div>;
  if (!event) return <div>Evento no encontrado</div>;

  return (
    <div className="event-detail">
      <h2>{event.title}</h2>
      <p><strong>Fecha:</strong> {new Date(event.date).toLocaleString('es-ES')}</p>
      <p>{event.description}</p>
      <hr />
      <RegistrationForm eventId={event._id} />
    </div>
  );
};

export default EventDetailPage;
