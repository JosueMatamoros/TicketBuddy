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
  constructor(props) {
    super(props);
    // Encapsulates seat state
    this.state = {
      booked: props.booked, // 'F', 'B', 'R'
    };
    this.handleClick = this.handleClick.bind(this);
  }

  // Method to handle seat click events
  handleClick() {
    // Logic to handle seat booking can be implemented here
    // For example, toggling between 'F' and 'B'
    this.setState((prevState) => ({
      booked: prevState.booked === 'F' ? 'B' : 'F',
    }));
  }

  render() {
    const { number } = this.props;
    const { booked } = this.state;

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
        onClick={this.handleClick}
        className="w-6 h-6 m-0.5 text-xs flex items-center justify-center"
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
