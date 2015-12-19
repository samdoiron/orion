'use strict';

var _createClass = (function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; })();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

var CONFIG = {
  webSocketServer: 'ws://localhost:1742'
};

function openWebsocket(url) {
  var connected = m.deferred(),
      socket = new WebSocket(url);

  socket.onopen = function () {
    connected.resolve(socket);
  };

  socket.onerror = function (err) {
    connected.reject('lol');
  };

  return connected.promise;
}

function connectToServer() {
  return openWebsocket(CONFIG.webSocketServer);
}

var ViewModelInput = (function () {
  function ViewModelInput(socket) {
    _classCallCheck(this, ViewModelInput);

    this.socket = socket;
    this.socket.onmessage = this._onMessage.bind(this);

    this.onMessageCallbacks = [];
    this.onErrorCallbacks = [];
  }

  _createClass(ViewModelInput, [{
    key: 'onUpdate',
    value: function onUpdate() {
      this.onMessageCallbacks.push(callback);
    }
  }, {
    key: 'onError',
    value: function onError() {
      this.onErrorCallbacks.push(callback);
    }
  }, {
    key: '_onMessage',
    value: function _onMessage() {
      m.beginComputation();
      _.each(this.onErrorCalbacks, function (cb) {
        return cb();
      });
      m.endComputation();
    }
  }], [{
    key: 'connect',
    value: function connect() {
      return connectToServer().then(function (socket) {
        return new ViewModelInput(socket);
      });
    }
  }]);

  return ViewModelInput;
})();