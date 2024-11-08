import React, { useState, useEffect } from 'react';
import Cards from 'react-credit-cards-2';
import 'react-credit-cards-2/dist/es/styles-compiled.css';

function PaymentMethods({ amount }) {
  const [paymentMethods, setPaymentMethods] = useState([]);
  const [selectedMethod, setSelectedMethod] = useState(null);
  const [cardDetails, setCardDetails] = useState({
    number: '',
    expiry: '',
    cvc: '',
    name: '',
    focus: '',
  });

  useEffect(() => {
    const plugins = [];
    const modules = import.meta.glob('../plugins/*.jsx', { eager: true });

    for (const path in modules) {
      const PluginClass = modules[path].default;
      const pluginInstance = new PluginClass();
      plugins.push(pluginInstance);
    }

    setPaymentMethods(plugins);
  }, []);

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setCardDetails((prev) => ({ ...prev, [name]: value }));
  };

  const handleInputFocus = (e) => {
    setCardDetails((prev) => ({ ...prev, focus: e.target.name }));
  };

  const handlePayment = async () => {
    if (!selectedMethod) {
      alert('Por favor, selecciona un método de pago.');
      return;
    }

    const { number, expiry, cvc, name } = cardDetails;

    if (!number || !expiry || !cvc || !name) {
      alert('Por favor, completa todos los campos del formulario.');
      return;
    }

    try {
      const result = await selectedMethod.processPayment(amount, cardDetails);
      if (result.success) {
        alert(`Pago exitoso. ID de transacción: ${result.transactionId}`);
      } else {
        alert('Error al procesar el pago.');
      }
    } catch (error) {
      console.error('Error al procesar el pago:', error);
    }
  };

  return (
    <div className="flex flex-col items-center">
      <h2 className="text-2xl font-bold mb-4">Métodos de Pago Disponibles</h2>
      <p className="mb-6">Monto a pagar: ${amount.toFixed(2)}</p>

      <select
        className="p-2 border rounded mb-4 w-64"
        onChange={(e) =>
          setSelectedMethod(
            paymentMethods.find((method) => method.name === e.target.value)
          )
        }
      >
        <option value="">Selecciona un método de pago</option>
        {paymentMethods.map((method) => (
          <option key={method.name} value={method.name}>
            {method.name}
          </option>
        ))}
      </select>

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
          placeholder="Card Number"
          className="p-2 border rounded w-full"
          value={cardDetails.number}
          onChange={handleInputChange}
          onFocus={handleInputFocus}
        />
        <input
          type="text"
          name="name"
          placeholder="Cardholder Name"
          className="p-2 border rounded w-full"
          value={cardDetails.name}
          onChange={handleInputChange}
          onFocus={handleInputFocus}
        />
        <input
          type="text"
          name="expiry"
          placeholder="MM/YY Expiry"
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
