// pages/Home.jsx
import React, { useEffect, useState } from 'react';

const Home = () => {
  const [ws, setWs] = useState(null);
  const [seatSuggestions, setSeatSuggestions] = useState([]);
  const [seatCount, setSeatCount] = useState(1);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    // Crear y abrir la conexión WebSocket
    const wsClient = new WebSocket('ws://127.0.0.1:8080');
    setWs(wsClient);

    wsClient.onopen = () => {
      console.log('Connected to WebSocket server');
      setConnected(true);
    };

    wsClient.onmessage = (event) => {
      const data = event.data;
      console.log('Received from server:', data);

      // Suponemos que el servidor envía una lista de asientos en formato de cadena
      const seats = data.split(', ');
      setSeatSuggestions(seats);
    };

    wsClient.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    wsClient.onclose = () => {
      console.log('WebSocket connection closed');
      setConnected(false);
    };

    // Cerrar la conexión cuando el componente se desmonte
    return () => wsClient.close();
  }, []);

  const handleSeatRequest = () => {
    if (ws && connected) {
      // Enviar el número de asientos al servidor
      ws.send(seatCount.toString());
      console.log('Sent seat count:', seatCount);
    } else {
      console.error('WebSocket is not connected');
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
          <ul>
            {seatSuggestions.map((seat, index) => (
              <li key={index}>{seat}</li>
            ))}
          </ul>
        ) : (
          <p>No hay sugerencias de asientos aún.</p>
        )}
      </div>
    </div>
  );
};

export default Home;
