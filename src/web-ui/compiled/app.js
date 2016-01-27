define(['server', 'dialog-component', 'util', 'layout'], function (_server, _dialogComponent, _util, _layout) {
  'use strict';

  var _server2 = _interopRequireDefault(_server);

  var _dialogComponent2 = _interopRequireDefault(_dialogComponent);

  var Layout = _interopRequireWildcard(_layout);

  function _interopRequireWildcard(obj) {
    if (obj && obj.__esModule) {
      return obj;
    } else {
      var newObj = {};

      if (obj != null) {
        for (var key in obj) {
          if (Object.prototype.hasOwnProperty.call(obj, key)) newObj[key] = obj[key];
        }
      }

      newObj.default = obj;
      return newObj;
    }
  }

  function _interopRequireDefault(obj) {
    return obj && obj.__esModule ? obj : {
      default: obj
    };
  }

  var _slicedToArray = (function () {
    function sliceIterator(arr, i) {
      var _arr = [];
      var _n = true;
      var _d = false;
      var _e = undefined;

      try {
        for (var _i = arr[Symbol.iterator](), _s; !(_n = (_s = _i.next()).done); _n = true) {
          _arr.push(_s.value);

          if (i && _arr.length === i) break;
        }
      } catch (err) {
        _d = true;
        _e = err;
      } finally {
        try {
          if (!_n && _i["return"]) _i["return"]();
        } finally {
          if (_d) throw _e;
        }
      }

      return _arr;
    }

    return function (arr, i) {
      if (Array.isArray(arr)) {
        return arr;
      } else if (Symbol.iterator in Object(arr)) {
        return sliceIterator(arr, i);
      } else {
        throw new TypeError("Invalid attempt to destructure non-iterable instance");
      }
    };
  })();

  var server = {
    commandOutput: null,
    vmInput: null
  };
  var vm = {
    local: {
      error: m.prop({
        isActive: false
      })
    },
    remote: m.prop({
      series: [],
      charts: []
    })
  };
  var OrionApp = {
    controller: function controller() {
      return {
        createHistogram: function createHistogram(name) {
          server.commandOutput.createHistogram(name);
        }
      };
    },
    view: function view(ctrl) {
      var series = vm.remote().series,
          charts = vm.remote().charts;
      this.ctrl = ctrl;
      return [Layout.sidebar([m('.vertical-list', series.map(this.renderSeries.bind(this, ctrl)))]), Layout.primary(charts.map(this.renderChart))];
    },
    renderSeries: function renderSeries(ctrl, seriesVm) {
      return m('.series-label', {
        onclick: (0, _util.args)(ctrl.createHistogram, seriesVm.name)
      }, [m('.series-label__name', seriesVm.name), m('.series-label__value', seriesVm.current_value.value)]);
    },
    renderChart: function renderChart(chartVm) {
      return m('.chart', [m('.chart__title', chartVm.title), m('.chart__body', [m('canvas', {
        config: this.configChart.bind(this.chartVm)
      })])]);
    },
    configChart: function configChart(chartVm, canvas) {
      resizeCanvasToParent(canvas);
      ChartRenderer.render(chartVm, canvas);
    }
  };
  (0, _server2.default)().then(function (_ref) {
    var _ref2 = _slicedToArray(_ref, 2);

    var vmInput = _ref2[0];
    var commandOutput = _ref2[1];
    server.commandOutput = commandOutput;
    server.vmInput = vmInput;
    vmInput.onUpdate(vm.remote);
    m.mount(document.getElementById('app'), OrionApp);
  });
});
//# sourceMappingURL=app.js.map