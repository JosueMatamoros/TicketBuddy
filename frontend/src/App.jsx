// App.jsx o donde configures tus rutas
import React from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Home from './pages/home';

const App = () => (
  <Router>
    <Routes>
      <Route path="/" element={<Home />} />
      {/* Otras rutas */}
    </Routes>
  </Router>
);

export default App;
