// src/components/HomeContent.jsx

import React from 'react';
import SeatReservationForm from './SeatReservationForm';
import SeatMap from './SeatMap';
import SeatSuggestionList from './SeatSuggestions'; // Asegúrate de importar correctamente
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
      selectedSuggestion: null,
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
    const selectedSuggestion = suggestions[index];
    const seats = selectedSuggestion.seats.map((seat) => ({
      ...seat,
      section: seat.section.toString(),
      row: seat.row.toString(),
    }));
    this.setState({
      suggestedSeats: seats,
      selectedSuggestionIndex: index,
      selectedSuggestion: selectedSuggestion,
    });
  }

  handleAcceptSuggestion() {
    const { sendChoice } = this.props;
    const { selectedSuggestionIndex, selectedSuggestion } = this.state;
    if (selectedSuggestionIndex !== null) {
      sendChoice(selectedSuggestionIndex + 1);
      // Navegar a la página de pago pasando el monto y los asientos
      this.props.navigate('/payment', {
        state: {
          amount: selectedSuggestion.total_price,
          seats: selectedSuggestion.seats,
        },
      });
    } else {
      alert('Por favor, seleccione una sugerencia antes de aceptar.');
    }
  }

  handleRejectSuggestion() {
    const { sendChoice } = this.props;
    sendChoice(0);
    this.setState({
      suggestedSeats: [],
      selectedSuggestionIndex: null,
      selectedSuggestion: null,
      showForm: true,
    });
  }

  render() {
    const {
      connected,
      suggestions,
      serverMessage,
      seatStates,
      paymentStatus,
    } = this.props;
    const {
      seatCount,
      selectedCategory,
      suggestedSeats,
      selectedSuggestionIndex,
      showForm,
      selectedSuggestion,
    } = this.state;

    const processedSeatStates = seatStates.map((seat) => ({
      ...seat,
      section: seat.section.toString(),
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
          <SeatMap
            seatStates={processedSeatStates}
            suggestedSeats={
              paymentStatus === 'success' ? [] : suggestedSeats
            } // No resaltar asientos después del pago
          />
        ) : (
          <p>Cargando estado de los asientos...</p>
        )}

        {/* Mostrar solo si el pago no ha sido exitoso */}
        {paymentStatus !== 'success' && showForm && (
          <SeatReservationForm
            seatCount={seatCount}
            setSeatCount={this.setSeatCount}
            selectedCategory={selectedCategory}
            setSelectedCategory={this.setSelectedCategory}
            handleSeatRequest={this.handleSeatRequest}
            connected={connected}
          />
        )}

        {/* Mostrar solo si el pago no ha sido exitoso */}
        {paymentStatus !== 'success' && suggestions.length > 0 && (
          <SeatSuggestionList
            suggestions={suggestions}
            onSelectSuggestion={this.handleSuggestionSelect}
            selectedSuggestionIndex={selectedSuggestionIndex}
            onAccept={this.handleAcceptSuggestion}
            onReject={this.handleRejectSuggestion}
          />
        )}

        {/* Mostrar los asientos reservados después del pago exitoso */}
        {paymentStatus === 'success' && selectedSuggestion && (
          <div className="mt-4">
            <h2 className="text-2xl font-bold mb-2">Asientos Reservados:</h2>
            <ul>
              {selectedSuggestion.seats.map((seat, index) => (
                <li key={index}>
                  {seat.section}-Fila{seat.row}-Asiento{seat.number}
                </li>
              ))}
            </ul>
          </div>
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
      paymentStatus={paymentStatus}
      setPaymentStatus={setPaymentStatus}
    />
  );
};

export default HomeContentWithWebSocket;