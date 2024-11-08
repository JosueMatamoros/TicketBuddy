import PaymentPlugin from './PaymentPlugin';

class Visa extends PaymentPlugin {
  constructor() {
    super('Visa', '../brands/visa.png'); // Asegúrate de que el logo esté en la ruta correcta
  }
}

export default Visa;