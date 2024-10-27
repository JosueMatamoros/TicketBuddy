// src/contexts/WebSocketContext.js

import React, { createContext, useContext, useEffect, useState } from 'react';
import WebSocketInstance from '../services/WebSocketService';

const WebSocketContext = createContext();

export const WebSocketProvider = ({ children }) => {
  const [connected, setConnected] = useState(false);
  const [seatSuggestions, setSeatSuggestions] = useState([]);

  useEffect(() => {
    WebSocketInstance.connect();

    WebSocketInstance.addCallbacks((data) => {
      // Manejar las sugerencias de asientos recibidas del servidor
      setSeatSuggestions(data.split(', ')); // Suponiendo que el servidor envía una cadena separada por comas
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

  return (
    <WebSocketContext.Provider value={{ connected, seatSuggestions, sendSeatRequest }}>
      {children}
    </WebSocketContext.Provider>
  );
};

export const useWebSocket = () => useContext(WebSocketContext);
