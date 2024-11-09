// src/contexts/WebSocketContext.js

import React, { createContext, useContext, useEffect, useState } from 'react';
import WebSocketInstance from '../services/WebSocketService';

const WebSocketContext = createContext();

export const WebSocketProvider = ({ children }) => {
  const [connected, setConnected] = useState(false);
  const [suggestions, setSuggestions] = useState([]);
  const [serverMessage, setServerMessage] = useState('');
  const [seatStates, setSeatStates] = useState([]);
  const [paymentStatus, setPaymentStatus] = useState(null);

  useEffect(() => {
    WebSocketInstance.connect();

    WebSocketInstance.addCallbacks((data) => {
      try {
        const jsonData = JSON.parse(data);
        if (Array.isArray(jsonData)) {
          if (jsonData.length > 0 && jsonData[0].hasOwnProperty('suggestion_number')) {
            // Es un array de sugerencias con precios
            setSuggestions(jsonData);
            setServerMessage('');
          } else if (jsonData.length > 0 && jsonData[0].hasOwnProperty('section')) {
            // Es el estado de los asientos
            setSeatStates(jsonData);
          }
        } else if (typeof jsonData === 'object' && jsonData !== null) {
          // Manejar otros objetos JSON si es necesario
        }
      } catch (e) {
        // Manejo de otros mensajes
        if (data === 'Sugerencia aceptada' || data === 'Sugerencias rechazadas') {
          setServerMessage(data);
          setSuggestions([]);
        } else if (data === 'Pago exitoso') {
          setServerMessage(data);
          setPaymentStatus('success');
        } else if (data === 'Pago fallido. Intente nuevamente.') {
          setServerMessage(data);
          setPaymentStatus('failure');
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

  const sendPaymentResult = (success, seats) => {
    const message = {
      type: 'payment_result',
      success,
      seats,
    };
    WebSocketInstance.sendMessage(JSON.stringify(message));
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
        paymentStatus,
        setPaymentStatus,
        sendPaymentResult,
      }}
    >
      {children}
    </WebSocketContext.Provider>
  );
};

export const useWebSocket = () => useContext(WebSocketContext);
