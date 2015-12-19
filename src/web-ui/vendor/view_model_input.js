var CONFIG = {
    webSocketServer: 'ws://localhost:1742'
};
function openWebsocket(url) {
    var connected = m.deferred(), socket = new WebSocket(url);
    socket.onopen = function () {
        connected.resolve(socket);
    };
    socket.onerror = connected.reject;
    return connected.promise;
}
function connectToServer() {
    return openWebsocket(CONFIG.webSocketServer);
}
function ViewModelInput(socket) {
    this.socket = socket;
    this.socket.onmessage = this._onMessage.bind(this);
    this.onMessageCallbacks = [];
    this.onErrorCallbacks = [];
}
ViewModelInput.connect = function () {
    return connectToServer().then(function (socket) {
        return new ViewModelInput(socket);
    });
};
ViewModelInput.prototype.onUpdate = function (callback) {
    this.onMessageCallbacks.push(callback);
};
ViewModelInput.prototype.onError = function (callback) {
    this.onErrorCallbacks.push(callback);
};
ViewModelInput.prototype._onMessage = function (message) {
    m.beginComputation();
    _.each(this.onErrorCalbacks, function (cb) { cb(message); });
    m.endComputation();
};
