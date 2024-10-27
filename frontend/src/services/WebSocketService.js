// src/services/WebSocketService.js

class WebSocketService {
  static instance = null;
  callbacks = {};

  // Método para obtener la única instancia de WebSocketService
  static getInstance() {
    if (!WebSocketService.instance) {
      WebSocketService.instance = new WebSocketService();
    }
    return WebSocketService.instance;
  }

  constructor() {
    this.socketRef = null;
  }

  // Método para establecer la conexión WebSocket
  connect() {
    const path = 'ws://127.0.0.1:8080/';
    this.socketRef = new WebSocket(path);

    this.socketRef.onopen = () => {
      console.log('WebSocket connected');
    };

    this.socketRef.onmessage = (e) => {
      this.socketNewMessage(e.data);
    };

    this.socketRef.onerror = (e) => {
      console.error('WebSocket error:', e);
    };

    this.socketRef.onclose = () => {
      console.log('WebSocket closed');
      this.connect(); // Intentar reconectar automáticamente
    };
  }

  // Método para manejar nuevos mensajes recibidos
  socketNewMessage(data) {
    if (this.callbacks['message']) {
      this.callbacks['message'](data);
    }
  }

  // Método para añadir callbacks que manejarán ciertos tipos de mensajes
  addCallbacks(messageCallback) {
    this.callbacks['message'] = messageCallback;
  }

  // Método para enviar mensajes al servidor
  sendMessage(data) {
    try {
      this.socketRef.send(data);
    } catch (err) {
      console.error('WebSocket send error:', err.message);
    }
  }

  // Método para obtener el estado actual de la conexión WebSocket
  state() {
    return this.socketRef.readyState;
  }

  // Método para cerrar la conexión WebSocket
  disconnect() {
    this.socketRef.close();
  }
}

const WebSocketInstance = WebSocketService.getInstance();

export default WebSocketInstance;
