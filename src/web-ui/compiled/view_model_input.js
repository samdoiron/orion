define([], function () {
  'use strict';

  function _classCallCheck(instance, Constructor) {
    if (!(instance instanceof Constructor)) {
      throw new TypeError("Cannot call a class as a function");
    }
  }

  var _createClass = (function () {
    function defineProperties(target, props) {
      for (var i = 0; i < props.length; i++) {
        var descriptor = props[i];
        descriptor.enumerable = descriptor.enumerable || false;
        descriptor.configurable = true;
        if ("value" in descriptor) descriptor.writable = true;
        Object.defineProperty(target, descriptor.key, descriptor);
      }
    }

    return function (Constructor, protoProps, staticProps) {
      if (protoProps) defineProperties(Constructor.prototype, protoProps);
      if (staticProps) defineProperties(Constructor, staticProps);
      return Constructor;
    };
  })();

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
      value: function onUpdate(callback) {
        this.onMessageCallbacks.push(callback);
      }
    }, {
      key: 'onError',
      value: function onError(callback) {
        this.onErrorCallbacks.push(callback);
      }
    }, {
      key: '_onMessage',
      value: function _onMessage(message) {
        m.startComputation();
        var vm = JSON.parse(message.data);
        console.log('%c VM', 'color: hotpink', vm);
        this.onMessageCallbacks.forEach(function (x) {
          return x(vm);
        });
        m.endComputation();
      }
    }], [{
      key: 'connect',
      value: function connect() {
        return connectToServer().then(function (socket) {
          window.socket = socket;
          return new ViewModelInput(socket);
        });
      }
    }]);

    return ViewModelInput;
  })();
});
//# sourceMappingURL=view_model_input.js.map