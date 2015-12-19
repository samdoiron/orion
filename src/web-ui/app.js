;(function () {

  let vm = {
    local: {
      error: m.prop({
        isActive: false
      })
    },
    remote: {
      series: [
        // { name: 'fps', value: 10 },
        // { name: 'numEntities', value: 10 },
        // { name: 'score', value: 10 }
      ],
      charts: [
        // {
        //   title: 'fps vs. time',
        //   xMin: 0,
        //   xMax: 10,
        //   yMin: 0,
        //   yMax: 10,
        //   points: m.prop([
        //     {x: 0, y: Math.random() * 10},
        //     {x: Math.random() * 10, y: Math.random() * 10},
        //     {x: Math.random() * 10, y: Math.random() * 10},
        //     {x: Math.random() * 10, y: Math.random() * 10},
        //     {x: Math.random() * 10, y: Math.random() * 10},
        //     {x: Math.random() * 10, y: Math.random() * 10},
        //     {x: Math.random() * 10, y: Math.random() * 10},
        //     {x: Math.random() * 10, y: Math.random() * 10},
        //     {x: 10, y: Math.random() * 10},
        //   ])
        // },
      ]
    }
  };

  function showMessage(opts) {
    vm.local.error({
      isActive: true,
      message: opts.message,
      buttons: (opts.buttons || []).map(button => {
        let oldClick = button.onclick;
        button.onclick = () => {
          oldClick()
          vm.local.error({ isActive: false })
        }
        return button
      })
    })
  }

  function hideMessage() {
    vm.local.error({ isActive: false })
  }

  function connectToVMInput() {
    m.startComputation()
    ViewModelInput.connect().then(() => {
    }, () => {
      showMessage({
        message: 'Could not connect to VM input',
        buttons: [{
          text: 'Retry',
          onclick: connectToVMInput
        }]
      })
    })
    .then(m.endComputation)
  }

  connectToVMInput()

  let OrionApp = {
    controller: function () {
    },

    view: function () {
      let series = vm.remote.series,
        charts = vm.remote.charts

      return [
        m('.l-sidebar', [
          m('.vertical-list', series.map(this.renderSeries.bind(this)))
        ]),
        m('.l-primary', charts.map(this.renderChart.bind(this))),
        m.component(DialogComponent, vm.local.error())
      ]
    },

    renderSeries: function (seriesVm) {
      return m('.vertical-list__item', [
        m('.series-label.u-unselectable', [
          m('.series-label__name', seriesVm.name),
          m('.series-label__value', seriesVm.value)
        ])
      ])
    },

    renderChart: function (chartVm) {
      return m('.chart', [
        m('h1.chart__title', chartVm.title),
        m('.chart__body', [
          m('canvas', { config: this.configChart.bind(this, chartVm) })
        ])
      ])
    },

    configChart: function (chartVm, canvas, isInitialized) { 
      if (!isInitialized) {
        canvas.parentElement.addEventListener('resize',() => {
         resizeCanvasToParent(canvas);
         ChartRenderer.render(chartVm, canvas);
        })
      }
      resizeCanvasToParent(canvas);
      ChartRenderer.render(chartVm, canvas);
    },
  };

  m.mount(document.getElementById('app'), OrionApp);
}())
