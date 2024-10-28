// src/components/SeatSuggestionList.jsx

import React from 'react';
import PropTypes from 'prop-types';
import { Button } from '@material-tailwind/react';

/**
 * SeatSuggestionList Class Component
 *
 * Displays a list of seat suggestions and allows the user to select one.
 * Demonstrates encapsulation by managing its own rendering and event handling.
 */
class SeatSuggestionList extends React.Component {
  render() {
    const {
      suggestions,
      onSelectSuggestion,
      selectedSuggestionIndex,
      onAccept,
      onReject,
    } = this.props;

    return (
      <div className="mt-4">
        <h2 className="text-2xl font-bold mb-2">Sugerencias de Asientos:</h2>
        <ul className="list-none pl-0">
          {suggestions.map((suggestion, index) => (
            <li
              key={index}
              className={`border p-2 mb-2 rounded cursor-pointer ${
                selectedSuggestionIndex === index ? 'bg-blue-200' : 'bg-white'
              }`}
              onClick={() => onSelectSuggestion(index)}
            >
              {suggestion}
            </li>
          ))}
        </ul>
        <div className="flex space-x-4">
          <Button
            color="green"
            onClick={onAccept}
            disabled={selectedSuggestionIndex === null}
          >
            Aceptar Sugerencia
          </Button>
          <Button color="red" onClick={onReject}>
            Rechazar Todas
          </Button>
        </div>
      </div>
    );
  }
}

SeatSuggestionList.propTypes = {
  suggestions: PropTypes.arrayOf(PropTypes.string).isRequired,
  onSelectSuggestion: PropTypes.func.isRequired,
  selectedSuggestionIndex: PropTypes.number,
  onAccept: PropTypes.func.isRequired,
  onReject: PropTypes.func.isRequired,
};

export default SeatSuggestionList;
