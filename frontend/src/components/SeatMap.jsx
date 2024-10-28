// components/SeatMap.jsx

import React from "react";
import PropTypes from "prop-types";
import Section from "../models/Section";

/**
 * SeatMap Class Component
 *
 * Represents the entire seat map containing multiple sections arranged in specific groups.
 * Demonstrates the Composite design pattern.
 */
class SeatMap extends React.Component {
  render() {
    const { seatStates } = this.props;

    // Group seats by section and row
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

    // Define the section groups as per your example
    const sectionGroups = [
      { name: "Group1", sections: ["D", "E", "F", "G", "H"] },
      { name: "Group2", sections: ["A3", "B3", "C3"] },
      { name: "Group3", sections: ["A2", "B2", "C2"] },
      { name: "Group4", sections: ["A1", "B1", "C1"] },
    ];

    return (
      <div className="mt-4">
        <h2 className="text-2xl font-bold mb-2">Estado de los Asientos:</h2>

        <div className="w-full max-w-4xl p-4 rounded-lg shadow-lg">
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
                  return null; // If the section doesn't exist, skip rendering
                }
                // Sort rows and seats within rows
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
                  />
                );
              })}
            </div>
          ))}

          {/* Escenario (Stage) */}
          <div className="w-full h-8 bg-blue-300 flex items-center justify-center text-white font-bold">
            Escenario
          </div>

          {/* Legend */}
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
};

export default SeatMap;
