// src/components/SeatSuggestions.jsx

import React from 'react';

const SeatSuggestions = ({ seatSuggestions }) => {
  if (seatSuggestions.length === 0) {
    return null; // No mostrar nada si no hay sugerencias
  }

  return (
    <div className="mt-4">
      <h2 className="text-2xl font-bold mb-2">Sugerencias de Asientos:</h2>
      <ul className="list-disc pl-5">
        {seatSuggestions.map((suggestion, index) => (
          <li key={index}>{suggestion}</li>
        ))}
      </ul>
    </div>
  );
};

export default SeatSuggestions;
