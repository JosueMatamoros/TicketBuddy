// src/components/HomeContent.jsx
import React from 'react';
import SeatReservationForm from './SeatReservationForm';
import SeatMap from './SeatMap';
import SeatSuggestionList from './SeatSuggestions';
import { useWebSocket } from '../contexts/WebSocketContext';
import { useNavigate } from 'react-router-dom';

class HomeContent extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      seatCount: 1,
      selectedCategory: '',
      suggestedSeats: [],
      selectedSuggestionIndex: null,
      showForm: true,
    };
    this.handleSeatRequest = this.handleSeatRequest.bind(this);
    this.setSeatCount = this.setSeatCount.bind(this);
    this.setSelectedCategory = this.setSelectedCategory.bind(this);
    this.handleSuggestionSelect = this.handleSuggestionSelect.bind(this);
    this.handleAcceptSuggestion = this.handleAcceptSuggestion.bind(this);
    this.handleRejectSuggestion = this.handleRejectSuggestion.bind(this);
  }

  setSeatCount(value) {
    this.setState({ seatCount: value });
  }

  setSelectedCategory(value) {
    this.setState({ selectedCategory: value });
  }

  handleSeatRequest() {
    const { sendSeatRequest } = this.props;
    const { seatCount, selectedCategory } = this.state;

    if (!selectedCategory) {
      alert('Por favor, seleccione una categoría.');
      return;
    }

    sendSeatRequest(seatCount, selectedCategory);
    this.setState({ showForm: false });
  }

  handleSuggestionSelect(index) {
    const { suggestions } = this.props;
    const suggestionString = suggestions[index];
    const seats = this.parseSuggestionString(suggestionString);
    this.setState({
      suggestedSeats: seats,
      selectedSuggestionIndex: index,
    });
  }

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
      sendChoice(selectedSuggestionIndex + 1);
      this.props.navigate('/payment');
    } else {
      alert('Por favor, seleccione una sugerencia antes de aceptar.');
    }
  }

  handleRejectSuggestion() {
    const { sendChoice } = this.props;
    sendChoice(0);
    this.setState({ suggestedSeats: [], selectedSuggestionIndex: null, showForm: true });
  }

  render() {
    const { connected, suggestions, serverMessage, seatStates, paymentStatus } = this.props;
    const { seatCount, selectedCategory, suggestedSeats, selectedSuggestionIndex, showForm } = this.state;

    const processedSeatStates = seatStates.map((seat) => ({
      ...seat,
      row: seat.row.toString(),
      booked: seat.booked,
    }));

    return (
      <div className="flex flex-col items-center justify-center min-h-screen">
        <h1 className="text-4xl font-bold m-2">Ticket Buddy</h1>

        {/* Mostrar mensaje de pago si existe */}
        {paymentStatus === 'success' && (
          <div className="bg-green-200 text-green-800 p-4 rounded mb-4">
            Pago realizado con éxito.
          </div>
        )}
        {paymentStatus === 'failure' && (
          <div className="bg-red-200 text-red-800 p-4 rounded mb-4">
            El pago ha fallado. Por favor, inténtalo de nuevo.
          </div>
        )}

        {processedSeatStates.length > 0 ? (
          <SeatMap seatStates={processedSeatStates} suggestedSeats={suggestedSeats} />
        ) : (
          <p>Cargando estado de los asientos...</p>
        )}

        {showForm && (
          <SeatReservationForm
            seatCount={seatCount}
            setSeatCount={this.setSeatCount}
            selectedCategory={selectedCategory}
            setSelectedCategory={this.setSelectedCategory}
            handleSeatRequest={this.handleSeatRequest}
            connected={connected}
          />
        )}

        {suggestions.length > 0 && (
          <SeatSuggestionList
            suggestions={suggestions}
            onSelectSuggestion={this.handleSuggestionSelect}
            selectedSuggestionIndex={selectedSuggestionIndex}
            onAccept={this.handleAcceptSuggestion}
            onReject={this.handleRejectSuggestion}
          />
        )}

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
  const navigate = useNavigate();
  const {
    connected,
    suggestions,
    serverMessage,
    seatStates,
    sendSeatRequest,
    sendChoice,
    paymentStatus,       
    setPaymentStatus,    
  } = useWebSocket();

  return (
    <HomeContent
      connected={connected}
      suggestions={suggestions}
      serverMessage={serverMessage}
      seatStates={seatStates}
      sendSeatRequest={sendSeatRequest}
      sendChoice={sendChoice}
      navigate={navigate}
      paymentStatus={paymentStatus}       // Pasamos el estado del pago
      setPaymentStatus={setPaymentStatus} // Pasamos el setter
    />
  );
};

export default HomeContentWithWebSocket;
