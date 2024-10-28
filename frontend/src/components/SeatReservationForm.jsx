// components/SeatReservationForm.jsx

import React from "react";
import { Button, Select, Option } from "@material-tailwind/react";

const SeatReservationForm = ({
  seatCount,
  setSeatCount,
  selectedCategory,
  setSelectedCategory,
  handleSeatRequest,
  connected,
}) => {
  return (
    <div className="w-1/2 flex flex-row justify-between">
      <div>
        <label htmlFor="seatCount" className="block text-lg mb-2">
          Cantidad:
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
      </div>
      <div>
        <label htmlFor="category" className="block text-lg mb-2">
          Categoría:
        </label>
        <div className="w-72">
          <Select
            label="Seleccione una categoría"
            animate={{
              mount: { y: 0 },
              unmount: { y: 25 },
            }}
            id="category"
            value={selectedCategory}
            onChange={(value) => setSelectedCategory(value)} // Maneja el valor seleccionado directamente
          >
            <Option value="VIP">VIP</Option>
            <Option value="Business">Business</Option>
            <Option value="Economy">Economy</Option>
          </Select>
        </div>
      </div>

      <div className="flex items-center">
        <Button
          color="blue"
          onClick={handleSeatRequest}
          disabled={!connected}
          className="w-auto h-auto p-4"
        >
          Solicitar Sugerencias
        </Button>
      </div>
    </div>
  );
};

export default SeatReservationForm;
