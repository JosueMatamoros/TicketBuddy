// components/SeatMap.jsx

import React from "react";
import PropTypes from "prop-types";
import Section from "../models/Section";

/**
 * SeatMap Class Component
 *
 * Representa todo el mapa de asientos que contiene múltiples secciones organizadas en grupos específicos.
 */
class SeatMap extends React.Component {
  render() {
    const { seatStates, suggestedSeats } = this.props;

    // Agrupar asientos por sección y fila
    const seatMap = {};

    seatStates.forEach((seat) => {
      const sectionKey = seat.section;
      const rowKey = seat.row.toString();

      if (!seatMap[sectionKey]) {
        seatMap[sectionKey] = {};
      }
      if (!seatMap[sectionKey][rowKey]) {
        seatMap[sectionKey][rowKey] = [];
      }
      seatMap[sectionKey][rowKey].push(seat);
    });

    // Definir los grupos de secciones
    const sectionGroups = [
      { name: "Group1", sections: ["D", "E", "F", "G", "H"] },
      { name: "Group2", sections: ["A3", "B3", "C3"] },
      { name: "Group3", sections: ["A2", "B2", "C2"] },
      { name: "Group4", sections: ["A1", "B1", "C1"] },
    ];

    return (
      <div className="mt-4">

        <div className="w-full max-w-4xl p-4 ">
          {sectionGroups.map((group, groupIndex) => (
            <div
              key={groupIndex}
              className={`flex justify-center mb-4 ${
                groupIndex === sectionGroups.length - 1 ? "mb-8" : ""
              }`}
            >
              {group.sections.map((sectionKey) => {
                const rows = seatMap[sectionKey];
                if (!rows) {
                  return null; // Si la sección no existe, omitir la renderización
                }
                // Ordenar filas y asientos dentro de las filas
                const sortedRows = Object.keys(rows)
                  .sort((a, b) => a - b)
                  .map((rowKey) =>
                    rows[rowKey].sort((a, b) => a.number - b.number)
                  );
                return (
                  <Section
                    key={sectionKey}
                    name={sectionKey}
                    rows={sortedRows}
                    suggestedSeats={suggestedSeats}
                  />
                );
              })}
            </div>
          ))}

          {/* Escenario */}
          <div className="w-full h-8 bg-blue-300 flex items-center justify-center text-white font-bold">
            Escenario
          </div>

          {/* Leyenda */}
          <div className="mt-4 flex justify-between">
            <div className="flex items-center">
              <div className="w-3 h-3 bg-red-500 mr-2"></div>
              <span className="text-sm">Ocupado</span>
            </div>
            <div className="flex items-center">
              <div className="w-3 h-3 bg-green-500 mr-2"></div>
              <span className="text-sm">Disponible</span>
            </div>
            <div className="flex items-center">
              <div className="w-3 h-3 bg-yellow-500 mr-2"></div>
              <span className="text-sm">Reservado Temporalmente</span>
            </div>
            <div className="flex items-center">
              <div className="w-3 h-3 bg-blue-500 mr-2"></div>
              <span className="text-sm">Seleccionados</span>
            </div>
          </div>
        </div>
      </div>
    );
  }
}

SeatMap.propTypes = {
  seatStates: PropTypes.arrayOf(
    PropTypes.shape({
      section: PropTypes.string.isRequired,
      row: PropTypes.oneOfType([PropTypes.string, PropTypes.number]).isRequired,
      number: PropTypes.number.isRequired,
      booked: PropTypes.string.isRequired, // 'F', 'B', 'R'
    })
  ).isRequired,
  suggestedSeats: PropTypes.arrayOf(
    PropTypes.shape({
      section: PropTypes.string.isRequired,
      row: PropTypes.string.isRequired,
      number: PropTypes.number.isRequired,
    })
  ),
};

export default SeatMap;
