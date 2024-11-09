// src/contexts/WebSocketContext.js
import React, { createContext, useContext, useEffect, useState } from 'react';
import WebSocketInstance from '../services/WebSocketService';

const WebSocketContext = createContext();

export const WebSocketProvider = ({ children }) => {
  const [connected, setConnected] = useState(false);
  const [suggestions, setSuggestions] = useState([]);
  const [serverMessage, setServerMessage] = useState('');
  const [seatStates, setSeatStates] = useState([]);
  const [paymentStatus, setPaymentStatus] = useState(null); // Añadido

  useEffect(() => {
    WebSocketInstance.connect();

    WebSocketInstance.addCallbacks((data) => {
      try {
        const jsonData = JSON.parse(data);
        if (Array.isArray(jsonData)) {
          setSeatStates(jsonData);
        }
      } catch (e) {
        if (data.startsWith('Sugerencia')) {
          const suggestionsArray = data.split('|');
          setSuggestions(suggestionsArray);
          setServerMessage('');
        } else if (
          data === 'Sugerencia aceptada' ||
          data === 'Sugerencias rechazadas'
        ) {
          setServerMessage(data);
          setSuggestions([]);
        } else if (data === 'No hay suficientes asientos disponibles en la categoría solicitada') {
          alert('No hay suficientes asientos disponibles en la categoría solicitada.');
          setSuggestions([]);
          setServerMessage('');
        } else {
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

  const sendSeatRequest = (seatCount, selectedCategory) => {
    const seatRequest = {
      category: selectedCategory,
      seat_count: parseInt(seatCount),
    };
    WebSocketInstance.sendMessage(JSON.stringify(seatRequest));
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
        paymentStatus,       // Añadido
        setPaymentStatus,    // Añadido
      }}
    >
      {children}
    </WebSocketContext.Provider>
  );
};

export const useWebSocket = () => useContext(WebSocketContext);
