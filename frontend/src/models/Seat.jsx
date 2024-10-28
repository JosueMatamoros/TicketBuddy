// models/Seat.jsx

import React from 'react';
import PropTypes from 'prop-types';
import { Button } from '@material-tailwind/react';

/**
 * Seat Class Component
 *
 * Represents an individual seat.
 * Demonstrates encapsulation by containing seat properties and rendering logic.
 */
class Seat extends React.Component {
  render() {
    const { number, booked } = this.props;

    // Determine color based on 'booked' status
    let color = 'green'; // Free
    if (booked === 'B') {
      color = 'red'; // Booked
    } else if (booked === 'R') {
      color = 'yellow'; // Temporarily reserved
    }

    return (
      <Button
        color={color}
        className="w-4 h-4 m-0.5 text-lg flex items-center justify-center"
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
};

export default Seat;
