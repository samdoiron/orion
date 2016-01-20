define(['view-model-input', 'dialog-component', 'layout'], function (_viewModelInput, _dialogComponent, _layout) {
  'use strict';

  var _viewModelInput2 = _interopRequireDefault(_viewModelInput);

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

  function showMessage(opts) {
    vm.local.error({
      isActive: true,
      message: opts.message,
      buttons: (opts.buttons || []).map(function (button) {
        var oldClick = button.onclick;

        button.onclick = function () {
          oldClick();
          vm.local.error({
            isActive: false
          });
        };

        return button;
      })
    });
  }

  function hideMessage() {
    vm.local.error({
      isActive: false
    });
  }

  function verticalList(items) {
    return m('.vertical-list', items.map(function (item) {
      return m('.vertical-list__item', item);
    }));
  }

  function errorDialog() {
    return m.component(_dialogComponent2.default, vm.local.error());
  }

  function bem(name, values) {
    var children = _.map(values, function (subVal, subName) {
      return m('.' + name + '__#{subName}', subVal);
    });

    return m('.' + name, children);
  }

  var OrionApp = {
    controller: function controller() {},
    view: function view() {
      var series = vm.remote().series,
          charts = vm.remote().charts;
      return [Layout.sidebar([verticalList(series.map(this.renderSeries))]), Layout.primary(charts.map(this.renderChart)), errorDialog()];
    },
    renderSeries: function renderSeries(seriesVm) {
      return bem('series-label', {
        name: seriesVm.name,
        value: seriesVm.current_value.value
      });
    },
    renderChart: function renderChart(chartVm) {
      return bem('chart', {
        title: chartVm.title,
        body: [m('canvas', {
          config: this.configChart.bind(this, chartVm)
        })]
      });
    },
    configChart: function configChart(chartVm, canvas) {
      resizeCanvasToParent(canvas);
      ChartRenderer.render(chartVm, canvas);
    }
  };
  var connectTimeoutMs = 100;

  function connectToVMInput() {
    _viewModelInput2.default.connect().then(function (vmInput) {
      vmInput.onUpdate(vm.remote);
    }, function () {
      setTimeout(connectToVMInput, connectTimeoutMs);
      connectTimeoutMs *= 1.5;
    });
  }

  connectToVMInput();
  m.mount(document.getElementById('app'), OrionApp);
});
//# sourceMappingURL=app.js.map