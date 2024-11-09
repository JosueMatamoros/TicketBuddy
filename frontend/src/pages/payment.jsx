// src/pages/Payment.jsx
import React from 'react';
import PaymentMethods from '../components/PaymentMethods';

const Payment = () => {
  const amount = 100.0; // Puedes ajustar esto según sea necesario

  return (
    <div className="min-h-screen bg-gray-100">
      <h1 className="text-center text-3xl font-bold py-4">Procesar Pago</h1>
      <PaymentMethods amount={amount} />
    </div>
  );
};

export default Payment;