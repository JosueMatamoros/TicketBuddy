// pages/Home.jsx

import React from 'react';
import SeatReservationForm from '../components/SeatReservationForm';
import SeatMap from '../components/SeatMap';
import SeatSuggestionList from '../components/SeatSuggestions';
import { WebSocketProvider, useWebSocket } from '../contexts/WebSocketContext';

class HomeContent extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      seatCount: 1,
      suggestedSeats: [],
      selectedSuggestionIndex: null,
    };
    this.handleSeatRequest = this.handleSeatRequest.bind(this);
    this.setSeatCount = this.setSeatCount.bind(this);
    this.handleSuggestionSelect = this.handleSuggestionSelect.bind(this);
    this.handleAcceptSuggestion = this.handleAcceptSuggestion.bind(this);
    this.handleRejectSuggestion = this.handleRejectSuggestion.bind(this);
  }

  handleSeatRequest() {
    const { sendSeatRequest } = this.props;
    const { seatCount } = this.state;
    sendSeatRequest(seatCount);
  }

  setSeatCount(value) {
    this.setState({ seatCount: value });
  }

  // Método para manejar la selección de una sugerencia
  handleSuggestionSelect(index) {
    const { suggestions } = this.props;
    const suggestionString = suggestions[index];
    const seats = this.parseSuggestionString(suggestionString);
    this.setState({
      suggestedSeats: seats,
      selectedSuggestionIndex: index,
    });
  }

  // Método para analizar la cadena de sugerencia y extraer los detalles de los asientos
  parseSuggestionString(suggestionString) {
    const seatsPart = suggestionString.split(': ')[1];
    const seatStrings = seatsPart.split(', ');
    const seats = seatStrings
      .map((seatStr) => {
        seatStr = seatStr.replace(/"/g, '').replace(/'/g, '');
        const regex = /(.+?)-Fila(\d+)-Asiento(\d+)/;
        const match = seatStr.match(regex);
        if (match) {
          const [, section, row, number] = match;
          return {
            section,
            row: row.toString(),
            number: parseInt(number, 10),
          };
        } else {
          return null;
        }
      })
      .filter((seat) => seat !== null);
    return seats;
  }

  handleAcceptSuggestion() {
    const { sendChoice } = this.props;
    const { selectedSuggestionIndex } = this.state;
    if (selectedSuggestionIndex !== null) {
      sendChoice(selectedSuggestionIndex + 1); // Enviar el índice de la sugerencia seleccionada (basado en 1)
      // Opcionalmente, limpiar suggestedSeats después de aceptar
      // this.setState({ suggestedSeats: [], selectedSuggestionIndex: null });
    }
  }

  handleRejectSuggestion() {
    const { sendChoice } = this.props;
    sendChoice(0); // Enviar '0' para indicar rechazo de todas las sugerencias
    this.setState({ suggestedSeats: [], selectedSuggestionIndex: null });
  }

  render() {
    const { connected, suggestions, serverMessage, seatStates } = this.props;
    const { seatCount, suggestedSeats, selectedSuggestionIndex } = this.state;

    // Procesar seatStates para que coincidan con los tipos de datos esperados
    const processedSeatStates = seatStates.map((seat) => ({
      ...seat,
      row: seat.row.toString(),
      booked: seat.booked,
    }));

    return (
      <div className="flex flex-col items-center justify-center min-h-screen">
        <h1 className="text-4xl font-bold mb-6">Reserva de Asientos</h1>

        {processedSeatStates.length > 0 ? (
          <SeatMap seatStates={processedSeatStates} suggestedSeats={suggestedSeats} />
        ) : (
          <p>Cargando estado de los asientos...</p>
        )}

        <SeatReservationForm
          seatCount={seatCount}
          setSeatCount={this.setSeatCount}
          handleSeatRequest={this.handleSeatRequest}
          connected={connected}
        />

        {/* Renderizar SeatSuggestionList si hay sugerencias */}
        {suggestions.length > 0 && (
          <SeatSuggestionList
            suggestions={suggestions}
            onSelectSuggestion={this.handleSuggestionSelect}
            selectedSuggestionIndex={selectedSuggestionIndex}
            onAccept={this.handleAcceptSuggestion}
            onReject={this.handleRejectSuggestion}
          />
        )}

        {/* Mostrar serverMessage si existe */}
        {serverMessage && (
          <div className="mt-4">
            <h2 className="text-2xl font-bold mb-2">{serverMessage}</h2>
          </div>
        )}
      </div>
    );
  }
}

// Usando el contexto de WebSocket
const HomeContentWithWebSocket = () => {
  const {
    connected,
    suggestions,
    serverMessage,
    seatStates,
    sendSeatRequest,
    sendChoice,
  } = useWebSocket();

  return (
    <HomeContent
      connected={connected}
      suggestions={suggestions}
      serverMessage={serverMessage}
      seatStates={seatStates}
      sendSeatRequest={sendSeatRequest}
      sendChoice={sendChoice}
    />
  );
};

const Home = () => (
  <WebSocketProvider>
    <HomeContentWithWebSocket />
  </WebSocketProvider>
);

export default Home;
