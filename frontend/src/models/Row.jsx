// models/Row.jsx

import React from 'react';
import PropTypes from 'prop-types';
import Seat from './Seat';

/**
 * Row Class Component
 *
 * Represents a row of seats.
 * Demonstrates composition by containing multiple Seat instances.
 */
class Row extends React.Component {
  render() {
    const { seats } = this.props;
    return (
      <div className="flex justify-center">
        {seats.map((seat) => (
          <Seat
            key={seat.number}
            number={seat.number}
            booked={seat.booked}
          />
        ))}
      </div>
    );
  }
}

Row.propTypes = {
  seats: PropTypes.arrayOf(
    PropTypes.shape({
      number: PropTypes.number.isRequired,
      booked: PropTypes.string.isRequired, // 'F', 'B', 'R'
    })
  ).isRequired,
};

export default Row;
