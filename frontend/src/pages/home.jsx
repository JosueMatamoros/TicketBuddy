// pages/Home.jsx

import React from 'react';
import SeatReservationForm from '../components/SeatReservationForm';
import SeatMap from '../components/SeatMap';
import { WebSocketProvider, useWebSocket } from '../contexts/WebSocketContext';

class HomeContent extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      seatCount: 1,
    };
    this.handleSeatRequest = this.handleSeatRequest.bind(this);
    this.setSeatCount = this.setSeatCount.bind(this);
  }

  handleSeatRequest() {
    const { sendSeatRequest } = this.props;
    const { seatCount } = this.state;
    sendSeatRequest(seatCount);
  }

  setSeatCount(value) {
    this.setState({ seatCount: value });
  }

  render() {
    const { connected, suggestions, serverMessage, seatStates, sendChoice } = this.props;
    const { seatCount } = this.state;

    // Process seatStates to match expected data types
    const processedSeatStates = seatStates.map((seat) => ({
      ...seat,
      row: seat.row.toString(), // Convert row to string if necessary
      booked: seat.booked, // Keep booked as char ('F', 'B', 'R')
    }));

    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 p-4">
        <h1 className="text-4xl font-bold mb-6">Reserva de Asientos</h1>

        {processedSeatStates.length > 0 ? (
          <SeatMap seatStates={processedSeatStates} />
        ) : (
          <p>Cargando estado de los asientos...</p>
        )}

        <SeatReservationForm
          seatCount={seatCount}
          setSeatCount={this.setSeatCount}
          handleSeatRequest={this.handleSeatRequest}
          connected={connected}
        />

        {/* Other components like SeatSuggestionList and serverMessage handling */}
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
