// src/pages/Home.jsx

import React, { useState } from 'react';
import SeatReservationForm from '../components/SeatReservationForm';
import SeatSuggestionList from '../components/SeatSuggestions';
import SeatMap from '../components/SeatMap';
import { WebSocketProvider, useWebSocket } from '../contexts/WebSocketContext';

const HomeContent = () => {
  const {
    connected,
    suggestions,
    serverMessage,
    seatStates,
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

      {seatStates.length > 0 ? (
        <SeatMap seatStates={seatStates} />
      ) : (
        <p>Cargando estado de los asientos...</p>
      )}

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
