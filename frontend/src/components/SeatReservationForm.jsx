// src/components/SeatReservationForm.jsx

import React from 'react';
import { Button } from '@material-tailwind/react';

const SeatReservationForm = ({ seatCount, setSeatCount, handleSeatRequest, connected }) => {
  return (
    <div className="mb-4">
      <label htmlFor="seatCount" className="block text-lg mb-2">
        Número de asientos:
      </label>
      <input
        id="seatCount"
        type="number"
        min="1"
        max="10"
        value={seatCount}
        onChange={(e) => setSeatCount(e.target.value)}
        className="border border-gray-300 p-2 rounded-md w-32 mb-4"
      />
      <Button color="blue" onClick={handleSeatRequest} disabled={!connected}>
        Solicitar Sugerencias
      </Button>
    </div>
  );
};

export default SeatReservationForm;
