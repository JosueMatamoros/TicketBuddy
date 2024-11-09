// src/components/PaymentMethods.jsx
import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useWebSocket } from '../contexts/WebSocketContext';
import Cards from 'react-credit-cards-2';
import 'react-credit-cards-2/dist/es/styles-compiled.css';

function PaymentMethods({ amount }) {
  const [cardDetails, setCardDetails] = useState({
    number: '',
    expiry: '',
    cvc: '',
    name: '',
    focus: '',
  });

  const { setPaymentStatus } = useWebSocket();
  const navigate = useNavigate();

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setCardDetails((prev) => ({ ...prev, [name]: value }));
  };

  const handleInputFocus = (e) => {
    setCardDetails((prev) => ({ ...prev, focus: e.target.name }));
  };

  const handlePayment = async () => {
    const { number, expiry, cvc, name } = cardDetails;

    if (!number || !expiry || !cvc || !name) {
      alert('Por favor, completa todos los campos del formulario.');
      return;
    }

    try {
      // Simular el resultado del pago (50% de probabilidad de éxito)
      const isSuccess = Math.random() < 0.5;

      if (isSuccess) {
        setPaymentStatus('success');
        alert('Pago procesado exitosamente.');
      } else {
        setPaymentStatus('failure');
        alert('El pago ha fallado.');
      }

      navigate('/'); // Regresar a la página principal
    } catch (error) {
      console.error('Error al procesar el pago:', error);
      alert('Error al procesar el pago.');
    }
  };

  return (
    <div className="flex flex-col items-center">
      <h2 className="text-2xl font-bold mb-4">Métodos de Pago</h2>
      <p className="mb-6">Monto a pagar: ${amount.toFixed(2)}</p>

      <div className="mb-6">
        <Cards
          number={cardDetails.number}
          expiry={cardDetails.expiry}
          cvc={cardDetails.cvc}
          name={cardDetails.name}
          focused={cardDetails.focus}
        />
      </div>

      <form className="w-64 space-y-4">
        <input
          type="tel"
          name="number"
          placeholder="Número de Tarjeta"
          className="p-2 border rounded w-full"
          value={cardDetails.number}
          onChange={handleInputChange}
          onFocus={handleInputFocus}
        />
        <input
          type="text"
          name="name"
          placeholder="Nombre del Titular"
          className="p-2 border rounded w-full"
          value={cardDetails.name}
          onChange={handleInputChange}
          onFocus={handleInputFocus}
        />
        <input
          type="text"
          name="expiry"
          placeholder="Fecha de Expiración (MM/AA)"
          className="p-2 border rounded w-full"
          value={cardDetails.expiry}
          onChange={handleInputChange}
          onFocus={handleInputFocus}
        />
        <input
          type="tel"
          name="cvc"
          placeholder="CVC"
          className="p-2 border rounded w-full"
          value={cardDetails.cvc}
          onChange={handleInputChange}
          onFocus={handleInputFocus}
        />
      </form>

      <button
        onClick={handlePayment}
        className="mt-6 bg-blue-600 text-white font-bold py-2 px-4 rounded hover:bg-blue-700"
      >
        Pagar
      </button>
    </div>
  );
}

export default PaymentMethods;
