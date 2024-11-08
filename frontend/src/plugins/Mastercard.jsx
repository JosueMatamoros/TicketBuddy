import PaymentPlugin from './PaymentPlugin';

class Mastercard extends PaymentPlugin {
  constructor() {
    super('Mastercard', '../brands/mastercard.png'); // Asegúrate de que el logo esté en la ruta correcta
  }
}

export default Mastercard;