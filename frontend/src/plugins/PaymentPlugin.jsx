export default class PaymentPlugin {
    constructor(name, logo) {
      this.name = name;
      this.logo = logo;
    }
  
    processPayment(amount, cardDetails) {
      return new Promise((resolve) => {
        console.log(`Procesando pago de $${amount} con ${this.name}...`);
        setTimeout(() => {
          const isSuccess = Math.random() > 0.2; // Simula éxito o fracaso
          if (isSuccess) {
            resolve({ success: true, transactionId: `${this.name.toUpperCase()}${Date.now()}` });
          } else {
            resolve({ success: false });
          }
        }, 1000);
      });
    }
  }