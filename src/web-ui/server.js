const CONFIG = {
  webSocketServer: 'ws://localhost:1742',
  commandFieldSeparator: '\u001f',
  connectTimeoutMs: 100
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

export default function connect() {
  return openWebsocket(CONFIG.webSocketServer)
    .then(socket => {
      let vmInput = new ViewModelInput(socket)
      let commandOutput = new CommandOutput(socket)
      return [vmInput, commandOutput]
    }, () => {
      let deferred = m.deferred()
      setTimeout(() => {
        connect().then(deferred.resolve)
      }, CONFIG.connectTimeoutMs)
      return deferred.promise
    })
}

class ViewModelInput {
  constructor(socket) {
    this.socket = socket
    this.socket.onmessage = this._onMessage.bind(this)
    this.socket.onclose = this._onClose.bind(this)

    this.onUpdateCallbacks = []
    this.onCloseCallbacks = []
    this.onErrorCallbacks = []
    this.lastVm = null;
  }

  onUpdate(callback) {
    this.onUpdateCallbacks.push(callback)

    // Sometimes we attach the initial update handler AFTER the first
    // VM has been sent, meaning the view wouldn't be rendered initially.
    if (this.lastVm) {
      callback(lastVm)
    }
  }

  onClose(callback) {
    this.onCloseCallbacks.push(callback)
  }

  onError(callback) {
    this.onErrorCallbacks.push(callback)
  }

  _onMessage(message) {
    m.startComputation();
    let vm = JSON.parse(message.data)
    this.lastVm = vm
    console.log('%c VM', 'color: green', vm)
    this.onUpdateCallbacks.forEach(x => x(vm))
    m.endComputation()
  }

  _onClose() {
    m.startComputation();
    this.onCloseCallbacks.forEach(x => x())
    m.endComputation();
  }
}

class CommandOutput {
  constructor(socket) {
    this.socket = socket;
  }

  createHistogram(name) {
    this._sendDoCommand('CreateHistogram', [name])
  }

  _sendDoCommand(commandName, args) {
    this.socket.send(this._encodeDoCommand(commandName, args))
    console.log('%c COMMAND', 'color: hotpink', commandName, args)
  }

  _encodeDoCommand(commandName, args) {
    return ['do', commandName].concat(args).join(CONFIG.commandFieldSeparator)
  }
}

