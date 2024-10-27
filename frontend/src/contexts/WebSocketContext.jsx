// src/contexts/WebSocketContext.js

import React, { createContext, useContext, useEffect, useState } from 'react';
import WebSocketInstance from '../services/WebSocketService';

const WebSocketContext = createContext();

export const WebSocketProvider = ({ children }) => {
  const [connected, setConnected] = useState(false);
  const [suggestions, setSuggestions] = useState([]); // Almacena las sugerencias de asientos
  const [serverMessage, setServerMessage] = useState(''); // Almacena mensajes de confirmación o rechazo
  const [seatStates, setSeatStates] = useState([]); // Almacena el estado de los asientos

  useEffect(() => {
    WebSocketInstance.connect();

    WebSocketInstance.addCallbacks((data) => {
      try {
        const jsonData = JSON.parse(data);
        if (Array.isArray(jsonData)) {
          // Asumimos que es el estado de los asientos
          setSeatStates(jsonData);
        }
      } catch (e) {
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
        seatStates,
        sendSeatRequest,
        sendChoice,
      }}
    >
      {children}
    </WebSocketContext.Provider>
  );
};

export const useWebSocket = () => useContext(WebSocketContext);
