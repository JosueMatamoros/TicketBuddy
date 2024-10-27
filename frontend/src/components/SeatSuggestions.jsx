// src/components/SeatSuggestionList.jsx

import React, { useState } from 'react';
import { Button } from '@material-tailwind/react';

const SeatSuggestionList = ({ suggestions, sendChoice }) => {
  const [selectedSuggestion, setSelectedSuggestion] = useState(null);

  const handleAccept = () => {
    if (selectedSuggestion !== null) {
      sendChoice(selectedSuggestion + 1); // Enviamos el índice de la sugerencia seleccionada (1-based)
    }
  };

  const handleReject = () => {
    sendChoice(0); // Enviamos '0' para indicar que rechazamos todas las sugerencias
  };

  return (
    <div className="mt-4">
      <h2 className="text-2xl font-bold mb-2">Sugerencias de Asientos:</h2>
      <ul className="list-none pl-0">
        {suggestions.map((suggestion, index) => (
          <li
            key={index}
            className={`border p-2 mb-2 rounded cursor-pointer ${
              selectedSuggestion === index ? 'bg-blue-200' : 'bg-white'
            }`}
            onClick={() => setSelectedSuggestion(index)}
          >
            {suggestion}
          </li>
        ))}
      </ul>
      <div className="flex space-x-4">
        <Button
          color="green"
          onClick={handleAccept}
          disabled={selectedSuggestion === null}
        >
          Aceptar Sugerencia
        </Button>
        <Button color="red" onClick={handleReject}>
          Rechazar Todas
        </Button>
      </div>
    </div>
  );
};

export default SeatSuggestionList;
