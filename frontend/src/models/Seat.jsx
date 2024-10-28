// models/Seat.jsx

import React from 'react';
import PropTypes from 'prop-types';
import { Button } from '@material-tailwind/react';

/**
 * Seat Class Component
 *
 * Representa un asiento individual.
 */
class Seat extends React.Component {
  render() {
    const { number, booked, section, row, suggestedSeats } = this.props;

    // Verificar si este asiento está en suggestedSeats
    const isSelected = suggestedSeats && suggestedSeats.some(
      (seat) =>
        seat.section === section &&
        seat.row === row.toString() &&
        seat.number === number
    );

    // Determinar el color basado en 'booked' y si está seleccionado
    let color = 'green'; // Disponible

    if (booked === 'B') {
      color = 'red'; // Ocupado
    } else if (booked === 'R') {
      color = 'yellow'; // Reservado temporalmente
    } else if (isSelected) {
      color = 'blue'; // Seleccionado en el front-end
    }

    return (
      <Button
        color={color}
        className="w-4 h-4 m-0.5 text-xs flex items-center justify-center"
        ripple={true}
      >
        {number}
      </Button>
    );
  }
}

Seat.propTypes = {
  number: PropTypes.number.isRequired,
  booked: PropTypes.string.isRequired, // 'F', 'B', 'R'
  section: PropTypes.string.isRequired,
  row: PropTypes.oneOfType([PropTypes.string, PropTypes.number]).isRequired,
  suggestedSeats: PropTypes.arrayOf(
    PropTypes.shape({
      section: PropTypes.string.isRequired,
      row: PropTypes.string.isRequired,
      number: PropTypes.number.isRequired,
    })
  ),
};

export default Seat;
