import ViewModelInput from 'view-model-input'; 
import DialogComponent from 'dialog-component';
import * as Layout from 'layout';

let vm = { local: {
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
    buttons: (opts.buttons || []).map(button => {
      let oldClick = button.onclick;
      button.onclick = () => {
        oldClick();
        vm.local.error({ isActive: false });
      }
      return button;
    })
  })
}

function hideMessage() {
  vm.local.error({ isActive: false });
}

function verticalList(items) {
  return m('.vertical-list', items.map(item => {
    return m('.vertical-list__item', item);
  }));
}

function errorDialog() {
    return m.component(DialogComponent, vm.local.error());
}

function bem(name, values) {
  let children = _.map(values, (subVal, subName) => {
    return m(`.${name}__#{subName}`, subVal);
  });
  return m(`.${name}`, children);
}

let OrionApp = {
  controller: function () {
  },

  view() {
    let series = vm.remote().series,
      charts = vm.remote().charts;

    return [
      Layout.sidebar([
        verticalList(series.map(this.renderSeries)),
      ]),
      Layout.primary(charts.map(this.renderChart)),
      errorDialog()
    ]
  },

  renderSeries(seriesVm) {
    return bem('series-label', {
      name: seriesVm.name,
      value: seriesVm.current_value.value
    })
  },

  renderChart(chartVm) {
    return bem('chart', {
      title: chartVm.title,
      body: [
        m('canvas', { config: this.configChart.bind(this, chartVm) })
      ]
    });
  },

  configChart(chartVm, canvas) { 
    resizeCanvasToParent(canvas);
    ChartRenderer.render(chartVm, canvas);
  },
};

let connectTimeoutMs = 100;
function connectToVMInput() {
  ViewModelInput.connect().then(vmInput => {
    // Only update 60 times / second, even if we get viewmodels more often.
    vmInput.onUpdate(vm.remote);
  }, () => {
    setTimeout(connectToVMInput, connectTimeoutMs);
    connectTimeoutMs *= 1.5;
  });
}

connectToVMInput();

m.mount(document.getElementById('app'), OrionApp);
