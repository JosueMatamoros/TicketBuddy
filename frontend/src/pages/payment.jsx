import React from 'react';
import PaymentMethods from '../components/PaymentMethods';

const Payment = () => {
  // Puedes obtener el monto a pagar de los parámetros de la ruta o usar un valor fijo por ahora
  const amount = 100.0; // Monto fijo por ahora

  return (
    <div className="min-h-screen bg-gray-100">
      <h1 className="text-center text-3xl font-bold py-4">Procesar Pago</h1>
      <PaymentMethods amount={amount} />
    </div>
  );
};

export default Payment;