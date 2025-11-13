import { Link, useLocation } from 'react-router-dom';

const SuccessPage = () => {
  const location = useLocation();
  const qrCode = location.state?.qrCodeDataUrl;

  return (
    <div className="success-page">
      <h2>¡Registro Exitoso!</h2>
      <p>Gracias por registrarte. Muestra este código QR en la entrada del evento.</p>
      {qrCode ? (
        <div className="qr-code-container">
          <img src={qrCode} alt="Tu código QR de registro" />
          <p><a href={qrCode} download="registro_qr.png">Descargar mi QR</a></p>
        </div>
      ) : (
        <p>No se pudo generar el código QR. Si recargaste la página, vuelve al evento y regístrate o guarda el QR al momento.</p>
      )}
      <Link to="/" className="btn">Volver al inicio</Link>
    </div>
  );
};

export default SuccessPage;
