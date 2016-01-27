define(['exports'], function (exports) {
  'use strict';

  Object.defineProperty(exports, "__esModule", {
    value: true
  });
  exports.default = connect;

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
    webSocketServer: 'ws://localhost:1742',
    commandFieldSeparator: '\u001f',
    connectTimeoutMs: 100
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

  function connect() {
    return openWebsocket(CONFIG.webSocketServer).then(function (socket) {
      var vmInput = new ViewModelInput(socket);
      var commandOutput = new CommandOutput(socket);
      return [vmInput, commandOutput];
    }, function () {
      var deferred = m.deferred();
      setTimeout(function () {
        connect().then(deferred.resolve);
      }, CONFIG.connectTimeoutMs);
      return deferred.promise;
    });
  }

  var ViewModelInput = (function () {
    function ViewModelInput(socket) {
      _classCallCheck(this, ViewModelInput);

      this.socket = socket;
      this.socket.onmessage = this._onMessage.bind(this);
      this.socket.onclose = this._onClose.bind(this);
      this.onUpdateCallbacks = [];
      this.onCloseCallbacks = [];
      this.onErrorCallbacks = [];
      this.lastVm = null;
    }

    _createClass(ViewModelInput, [{
      key: 'onUpdate',
      value: function onUpdate(callback) {
        this.onUpdateCallbacks.push(callback);

        if (this.lastVm) {
          callback(lastVm);
        }
      }
    }, {
      key: 'onClose',
      value: function onClose(callback) {
        this.onCloseCallbacks.push(callback);
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
        this.lastVm = vm;
        console.log('%c VM', 'color: green', vm);
        this.onUpdateCallbacks.forEach(function (x) {
          return x(vm);
        });
        m.endComputation();
      }
    }, {
      key: '_onClose',
      value: function _onClose() {
        m.startComputation();
        this.onCloseCallbacks.forEach(function (x) {
          return x();
        });
        m.endComputation();
      }
    }]);

    return ViewModelInput;
  })();

  var CommandOutput = (function () {
    function CommandOutput(socket) {
      _classCallCheck(this, CommandOutput);

      this.socket = socket;
    }

    _createClass(CommandOutput, [{
      key: 'createHistogram',
      value: function createHistogram(name) {
        this._sendDoCommand('CreateHistogram', [name]);
      }
    }, {
      key: '_sendDoCommand',
      value: function _sendDoCommand(commandName, args) {
        this.socket.send(this._encodeDoCommand(commandName, args));
        console.log('%c COMMAND', 'color: hotpink', commandName, args);
      }
    }, {
      key: '_encodeDoCommand',
      value: function _encodeDoCommand(commandName, args) {
        return ['do', commandName].concat(args).join(CONFIG.commandFieldSeparator);
      }
    }]);

    return CommandOutput;
  })();
});
//# sourceMappingURL=server.js.map