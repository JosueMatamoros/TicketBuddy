import React, { useEffect, useState } from 'react';

const Home = () => {
  const [ws, setWs] = useState(null);
  const [seatSuggestions, setSeatSuggestions] = useState([]);
  const [seatCount, setSeatCount] = useState(1);
  const [connected, setConnected] = useState(false);
  const [reservationStatus, setReservationStatus] = useState('');

  useEffect(() => {
    // Crear y abrir la conexión WebSocket
    const wsClient = new WebSocket('ws://127.0.0.1:8080');
    setWs(wsClient);

    wsClient.onopen = () => {
      console.log('Conectado al servidor WebSocket');
      setConnected(true);
    };

    wsClient.onmessage = (event) => {
      const data = event.data;
      console.log('Recibido del servidor:', data);

      if (data === 'Reserva confirmada' || data === 'Reserva rechazada') {
        setReservationStatus(data);
      } else {
        // Suponemos que las sugerencias están separadas por '|'
        const suggestions = data.split('|').map((suggestion) => suggestion.trim());
        setSeatSuggestions(suggestions);
      }
    };

    wsClient.onerror = (error) => {
      console.error('Error en WebSocket:', error);
    };

    wsClient.onclose = () => {
      console.log('Conexión WebSocket cerrada');
      setConnected(false);
    };

    // Cerrar la conexión cuando el componente se desmonte
    return () => wsClient.close();
  }, []);

  const handleSeatRequest = () => {
    if (ws && connected) {
      // Enviar el número de asientos al servidor
      ws.send(seatCount.toString());
      console.log('Número de asientos enviado:', seatCount);
      // Resetear estados
      setSeatSuggestions([]);
      setReservationStatus('');
    } else {
      console.error('WebSocket no está conectado');
    }
  };

  const handleSelection = (choice) => {
    if (ws && connected) {
      // Enviar la elección al servidor
      ws.send(choice.toString());
    } else {
      console.error('WebSocket no está conectado');
    }
  };

  return (
    <div>
      <h1>Reserva de Asientos</h1>
      <div>
        <label htmlFor="seatCount">Número de asientos:</label>
        <input
          id="seatCount"
          type="number"
          min="1"
          max="10"
          value={seatCount}
          onChange={(e) => setSeatCount(e.target.value)}
        />
        <button onClick={handleSeatRequest}>Solicitar Asientos</button>
      </div>
      <div>
        <h2>Asientos sugeridos:</h2>
        {seatSuggestions.length > 0 ? (
          <div>
            {seatSuggestions.map((suggestion, index) => (
              <div key={index}>
                <p>{suggestion}</p>
                <button onClick={() => handleSelection(index + 1)}>Aceptar Sugerencia {index + 1}</button>
              </div>
            ))}
            <button onClick={() => handleSelection(0)}>Rechazar todas las sugerencias</button>
          </div>
        ) : (
          <p>No hay sugerencias de asientos aún.</p>
        )}
      </div>
      {reservationStatus && <p>{reservationStatus}</p>}
    </div>
  );
};

export default Home;
