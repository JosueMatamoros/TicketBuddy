// src/App.jsx

import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Home from './pages/home';

const App = () => (
  <Router>
    <Routes>
      <Route path="/" element={<Home />} />
      {/* Puedes añadir más rutas aquí */}
    </Routes>
  </Router>
);

export default App;
