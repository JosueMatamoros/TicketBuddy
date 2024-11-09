// src/App.jsx

import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Home from './pages/home';
import Payment from './pages/payment';
import { WebSocketProvider } from './contexts/WebSocketContext';

const App = () => (
  <WebSocketProvider>
      <Router>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/payment" element={<Payment />} />
        </Routes>
      </Router>
    </WebSocketProvider>
);

export default App;
