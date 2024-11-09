// src/pages/Payment.jsx

import React from 'react';
import { useLocation } from 'react-router-dom';
import PaymentMethods from '../components/PaymentMethods';

const Payment = () => {
  const location = useLocation();
  const amount = location.state?.amount || 0.0;
  const seats = location.state?.seats || [];

  return (
    <div className="min-h-screen bg-gray-100">
      <h1 className="text-center text-3xl font-bold py-4">Procesar Pago</h1>
      <PaymentMethods amount={amount} seats={seats} />
    </div>
  );
};

export default Payment;
