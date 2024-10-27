// src/contexts/WebSocketContext.js

import React, { createContext, useContext, useEffect, useState } from 'react';
import WebSocketInstance from '../services/WebSocketService';

const WebSocketContext = createContext();

export const WebSocketProvider = ({ children }) => {
  const [connected, setConnected] = useState(false);
  const [suggestions, setSuggestions] = useState([]); // Almacena las sugerencias de asientos
  const [serverMessage, setServerMessage] = useState(''); // Almacena mensajes de confirmación o rechazo

  useEffect(() => {
    WebSocketInstance.connect();

    WebSocketInstance.addCallbacks((data) => {
      // Manejar los diferentes tipos de mensajes recibidos del servidor
      if (data.startsWith('Sugerencia')) {
        // Recibimos las sugerencias
        const suggestionsArray = data.split('|');
        setSuggestions(suggestionsArray);
        setServerMessage('');
      } else if (data === 'Reserva confirmada' || data === 'Reserva rechazada') {
        // Recibimos la confirmación o rechazo
        setServerMessage(data);
        setSuggestions([]); // Limpiamos las sugerencias
      } else {
        // Otros mensajes
        setServerMessage(data);
      }
    });

    WebSocketInstance.socketRef.onopen = () => {
      setConnected(true);
    };

    WebSocketInstance.socketRef.onclose = () => {
      setConnected(false);
    };

    return () => WebSocketInstance.disconnect();
  }, []);

  const sendSeatRequest = (seatCount) => {
    WebSocketInstance.sendMessage(seatCount.toString());
  };

  const sendChoice = (choice) => {
    WebSocketInstance.sendMessage(choice.toString());
  };

  return (
    <WebSocketContext.Provider
      value={{
        connected,
        suggestions,
        serverMessage,
        sendSeatRequest,
        sendChoice,
      }}
    >
      {children}
    </WebSocketContext.Provider>
  );
};

export const useWebSocket = () => useContext(WebSocketContext);
