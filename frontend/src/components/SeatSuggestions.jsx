// components/SeatSuggestionList.jsx

import React from 'react';
import PropTypes from 'prop-types';
import { Button } from '@material-tailwind/react';

/**
 * SeatSuggestionList Class Component
 *
 * Muestra una lista de sugerencias de asientos y permite al usuario seleccionar una.
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
      <div className="mt-4 w-full max-w-2xl">
        <h2 className="text-2xl font-bold mb-2">Sugerencias de Asientos:</h2>
        <ul className="list-none pl-0">
          {suggestions.map((suggestion, index) => (
            <li
              key={index}
              className={`border p-2 mb-2 rounded cursor-pointer transition-colors duration-200 ${
                selectedSuggestionIndex === index
                  ? 'bg-blue-500 text-white'
                  : 'bg-white hover:bg-gray-100'
              }`}
              onClick={() => onSelectSuggestion(index)}
            >
              {suggestion}
            </li>
          ))}
        </ul>
        <div className="flex space-x-4 mt-4">
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
