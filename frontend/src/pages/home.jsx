// pages/Home.jsx

import React from 'react';
import SeatReservationForm from '../components/SeatReservationForm';
import SeatMap from '../components/SeatMap';
import SeatSuggestionList from '../components/SeatSuggestionList';
import { WebSocketProvider, useWebSocket } from '../contexts/WebSocketContext';

class HomeContent extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      seatCount: 1,
      suggestedSeats: [], // State to hold suggested seats
      selectedSuggestionIndex: null, // To track the selected suggestion
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

  // Method to handle suggestion selection
  handleSuggestionSelect(index) {
    const { suggestions } = this.props;
    // Parse the suggestion string to extract seat details
    const suggestionString = suggestions[index];
    const seats = this.parseSuggestionString(suggestionString);
    this.setState({
      suggestedSeats: seats,
      selectedSuggestionIndex: index,
    });
  }

  // Method to parse the suggestion string
  parseSuggestionString(suggestionString) {
    // Extract the part after ': '
    const seatsPart = suggestionString.split(': ')[1];
    const seatStrings = seatsPart.split(', ');
    const seats = seatStrings
      .map((seatStr) => {
        // Remove quotes
        seatStr = seatStr.replace(/"/g, '').replace(/'/g, '');
        // Extract section, row, and number
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
      sendChoice(selectedSuggestionIndex + 1); // Send the selected suggestion index (1-based)
      // Optionally clear suggestedSeats after accepting
      // this.setState({ suggestedSeats: [], selectedSuggestionIndex: null });
    }
  }

  handleRejectSuggestion() {
    const { sendChoice } = this.props;
    sendChoice(0); // Send '0' to indicate rejection of all suggestions
    this.setState({ suggestedSeats: [], selectedSuggestionIndex: null });
  }

  render() {
    const { connected, suggestions, serverMessage, seatStates } = this.props;
    const { seatCount, suggestedSeats, selectedSuggestionIndex } = this.state;

    // Process seatStates to match expected data types
    const processedSeatStates = seatStates.map((seat) => ({
      ...seat,
      row: seat.row.toString(),
      booked: seat.booked,
    }));

    // Update seatStates to mark suggested seats as 'R' (yellow)
    const updatedSeatStates = processedSeatStates.map((seat) => {
      // Check if seat is in suggestedSeats
      const isSuggested = suggestedSeats.some(
        (s) =>
          s.section === seat.section &&
          s.row === seat.row.toString() &&
          s.number === seat.number
      );
      if (isSuggested) {
        return {
          ...seat,
          booked: 'R', // Mark as 'R' (yellow)
        };
      } else {
        return seat;
      }
    });

    return (
      <div className="flex flex-col items-center justify-center min-h-screen">
        <h1 className="text-4xl font-bold mb-6">Reserva de Asientos</h1>

        {updatedSeatStates.length > 0 ? (
          <SeatMap seatStates={updatedSeatStates} />
        ) : (
          <p>Cargando estado de los asientos...</p>
        )}

        <SeatReservationForm
          seatCount={seatCount}
          setSeatCount={this.setSeatCount}
          handleSeatRequest={this.handleSeatRequest}
          connected={connected}
        />

        {/* Render SeatSuggestionList if there are suggestions */}
        {suggestions.length > 0 && (
          <SeatSuggestionList
            suggestions={suggestions}
            onSelectSuggestion={this.handleSuggestionSelect}
            selectedSuggestionIndex={selectedSuggestionIndex}
            onAccept={this.handleAcceptSuggestion}
            onReject={this.handleRejectSuggestion}
          />
        )}

        {/* Display serverMessage if it exists */}
        {serverMessage && (
          <div className="mt-4">
            <h2 className="text-2xl font-bold mb-2">{serverMessage}</h2>
          </div>
        )}
      </div>
    );
  }
}

// Using the WebSocket context
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
