// src/pages/Home.jsx

import React, { useState } from 'react';
import SeatReservationForm from '../components/SeatReservationForm';
import SeatSuggestionList from '../components/SeatSuggestions';
import { WebSocketProvider, useWebSocket } from '../contexts/WebSocketContext';

/**
 * Home es la vista principal donde los usuarios pueden solicitar reservas de asientos
 * y manejar las sugerencias proporcionadas por el servidor.
 */

const HomeContent = () => {
  const {
    connected,
    suggestions,
    serverMessage,
    sendSeatRequest,
    sendChoice,
  } = useWebSocket();
  const [seatCount, setSeatCount] = useState(1);

  // Maneja el envío de la solicitud de asientos
  const handleSeatRequest = () => {
    sendSeatRequest(seatCount);
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-4">
      <h1 className="text-4xl font-bold mb-6">Reserva de Asientos</h1>
      <SeatReservationForm
        seatCount={seatCount}
        setSeatCount={setSeatCount}
        handleSeatRequest={handleSeatRequest}
        connected={connected}
      />
      {suggestions.length > 0 && (
        <SeatSuggestionList suggestions={suggestions} sendChoice={sendChoice} />
      )}
      {serverMessage && (
        <div className="mt-4">
          <h2 className="text-2xl font-bold mb-2">{serverMessage}</h2>
        </div>
      )}
    </div>
  );
};

const Home = () => (
  <WebSocketProvider>
    <HomeContent />
  </WebSocketProvider>
);

export default Home;
