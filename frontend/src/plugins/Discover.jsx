import PaymentPlugin from './PaymentPlugin';

class Discover extends PaymentPlugin {
  constructor() {
    super('Discover', '../brands/discover.png'); // Asegúrate de que el logo esté en la ruta correcta
  }
}

export default Discover;