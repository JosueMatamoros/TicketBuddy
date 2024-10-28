// models/Section.jsx

import React from 'react';
import PropTypes from 'prop-types';
import Seat from './Seat';

/**
 * Section Class Component
 *
 * Representa una sección que contiene filas de asientos.
 */
class Section extends React.Component {
  render() {
    const { name, rows, suggestedSeats } = this.props;

    return (
      <div className="flex flex-col items-center mx-2">
        <h3 className="text-lg font-bold mb-2">{name}</h3>
        {rows.map((row, rowIndex) => (
          <div key={rowIndex} className="flex">
            {row.map((seat) => (
              <Seat
                key={`${seat.section}-${seat.row}-${seat.number}`}
                number={seat.number}
                booked={seat.booked}
                section={seat.section}
                row={seat.row}
                suggestedSeats={suggestedSeats}
              />
            ))}
          </div>
        ))}
      </div>
    );
  }
}

Section.propTypes = {
  name: PropTypes.string.isRequired,
  rows: PropTypes.arrayOf(PropTypes.array).isRequired,
  suggestedSeats: PropTypes.arrayOf(
    PropTypes.shape({
      section: PropTypes.string.isRequired,
      row: PropTypes.string.isRequired,
      number: PropTypes.number.isRequired,
    })
  ),
};

export default Section;
