import { Route, Routes } from 'react-router-dom';
import EventDetailPage from './pages/EventDetailPage';
import HomePage from './pages/HomePage';
import SuccessPage from './pages/SucessPage';

function App() {
  return (
    <div className="container">
      <header><h1>Gestor de Eventos MVP</h1></header>
      <main>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/event/:id" element={<EventDetailPage />} />
          <Route path="/success" element={<SuccessPage />} />
        </Routes>
      </main>
    </div>
  );
}

export default App;
