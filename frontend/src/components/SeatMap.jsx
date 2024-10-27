// src/components/SeatMap.jsx

import React from 'react';

const SeatMap = ({ seatStates }) => {
  // Agrupar los asientos por sección y fila
  const seatMap = {};

  seatStates.forEach((seat) => {
    const key = `${seat.section}-Fila${seat.row}`;
    if (!seatMap[key]) {
      seatMap[key] = [];
    }
    seatMap[key].push(seat);
  });

  return (
    <div className="mt-4">
      <h2 className="text-2xl font-bold mb-2">Estado de los Asientos:</h2>
      {Object.keys(seatMap).map((key) => {
        const seats = seatMap[key];
        return (
          <div key={key} className="mb-4">
            <h4 className="text-xl font-semibold mb-2">{key}</h4>
            <div className="flex flex-wrap">
              {seats
                .sort((a, b) => a.number - b.number)
                .map((seat) => (
                  <div
                    key={seat.number}
                    className={`w-10 h-10 m-1 text-center text-white leading-10 rounded ${
                      seat.booked === 'F' ? 'bg-green-500' : 'bg-red-500'
                    }`}
                  >
                    {seat.number}
                  </div>
                ))}
            </div>
          </div>
        );
      })}
    </div>
  );
};

export default SeatMap;
