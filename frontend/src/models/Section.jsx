// models/Section.jsx

import React from 'react';
import PropTypes from 'prop-types';
import Row from './Row';

/**
 * Section Class Component
 *
 * Represents a section containing multiple rows.
 * Demonstrates composition and encapsulation.
 */
class Section extends React.Component {
  render() {
    const { name, rows } = this.props;
    return (
      <div className="m-1 p-2 border rounded">
        <div className="font-bold text-center text-xs mb-1">{name}</div>
        {rows.map((seats, index) => (
          <Row key={index} seats={seats} />
        ))}
      </div>
    );
  }
}

Section.propTypes = {
  name: PropTypes.string.isRequired,
  rows: PropTypes.arrayOf(PropTypes.array).isRequired,
};

export default Section;
