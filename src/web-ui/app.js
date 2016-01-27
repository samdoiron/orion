import connectToServer from 'server';
import DialogComponent from 'dialog-component';
import { args } from 'util';
import * as Layout from 'layout';

let server = {
  commandOutput: null,
  vmInput: null
}

let vm = {
  local: {
    error: m.prop({
      isActive: false
    })
  },
  remote: m.prop({
    series: [],
    charts: []
  })
}

let OrionApp = {
  controller: function () {
    return {
      createHistogram: (name) => {
        server.commandOutput.createHistogram(name)
      }
    }
  },

  view(ctrl) {
    let series = vm.remote().series,
      charts = vm.remote().charts

    this.ctrl = ctrl
    return [
      Layout.sidebar([
        m('.vertical-list', series.map(this.renderSeries.bind(this, ctrl))),
      ]),
      Layout.primary(charts.map(this.renderChart))
    ]
  },

  renderSeries(ctrl, seriesVm) {
    return m('.series-label', {onclick: args(ctrl.createHistogram, seriesVm.name)}, [
      m('.series-label__name', seriesVm.name),
      m('.series-label__value', seriesVm.current_value.value)
    ])
  },

  renderChart(chartVm) {
    return m('.chart', [
      m('.chart__title', chartVm.title),
      m('.chart__body', [
        m('canvas', { config: this.configChart.bind(this. chartVm) })
      ])
    ])
  },

  configChart(chartVm, canvas) { 
    resizeCanvasToParent(canvas);
    ChartRenderer.render(chartVm, canvas);
  },
};

connectToServer().then(([vmInput, commandOutput]) => {
  server.commandOutput = commandOutput
  server.vmInput = vmInput
  vmInput.onUpdate(vm.remote)
  m.mount(document.getElementById('app'), OrionApp);
});

