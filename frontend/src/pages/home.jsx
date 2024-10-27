// src/pages/Home.jsx

import React, { useState } from 'react';
import SeatReservationForm from '../components/SeatReservationForm';
import SeatSuggestions from '../components/SeatSuggestions';
import { WebSocketProvider, useWebSocket } from '../contexts/WebSocketContext';

/**
 * Home es la vista principal donde los usuarios pueden solicitar reservas de asientos
 * y ver las sugerencias proporcionadas por el servidor.
 * 
 * - **Patrón Presentational-Container**: 
 *   Este componente `Home` actúa como el **componente contenedor** que maneja la lógica de negocio 
 *   (conexión WebSocket y envío de solicitudes de asientos). Los componentes `SeatReservationForm` y 
 *   `SeatSuggestions` son **componentes presentacionales** que se encargan de mostrar la UI.
 */

const HomeContent = () => {
  const { connected, seatSuggestions, sendSeatRequest } = useWebSocket();
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
      <SeatSuggestions seatSuggestions={seatSuggestions} />
    </div>
  );
};

const Home = () => (
  <WebSocketProvider>
    <HomeContent />
  </WebSocketProvider>
);

export default Home;
