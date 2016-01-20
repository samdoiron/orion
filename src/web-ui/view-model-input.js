let CONFIG = {
  webSocketServer: 'ws://localhost:1742'
}

function openWebsocket(url) {
  let connected = m.deferred(),
    socket = new WebSocket(url)

  socket.onopen = function () {
    connected.resolve(socket)
  }

  socket.onerror = function (err) {
    connected.reject('lol')
  }

  return connected.promise
}

function connectToServer() {
  return openWebsocket(CONFIG.webSocketServer)
}

export default class ViewModelInput {
  constructor(socket) {
    this.socket = socket
    this.socket.onmessage = this._onMessage.bind(this)

    this.onMessageCallbacks = []
    this.onErrorCallbacks = []
  }

  static connect() {
    return connectToServer().then(socket => {
      window.socket = socket
      return new ViewModelInput(socket)
    })
  }

  onUpdate(callback) {
    this.onMessageCallbacks.push(callback)
  }

  onError(callback) {
    this.onErrorCallbacks.push(callback)
  }

  _onMessage(message) {
    m.startComputation();
    let vm = JSON.parse(message.data)
    console.log('%c VM', 'color: hotpink', vm)
    this.onMessageCallbacks.forEach(x => x(vm))
    m.endComputation();
  }
}
