;
(function () {
    function removeUnit(str) {
        return str.replace(/[^0-9.]/g, '');
    }
    function resizeCanvasToParent(canvas) {
        var parentStyle = getComputedStyle(canvas.parentElement);
        canvas.width = removeUnit(parentStyle.width);
        canvas.height = removeUnit(parentStyle.height);
    }
    function drawCircle(context, opts) {
        context.arc(opts.centerX, opts.centerY, opts.radius, 0, Math.PI * 2);
    }
    var vm = {
        local: {},
        remote: {
            series: [
                { name: 'fps', value: 10 },
                { name: 'numEntities', value: 10 },
                { name: 'score', value: 10 }
            ],
            charts: [
                {
                    title: 'fps vs. time',
                    xMin: 0,
                    xMax: 10,
                    yMin: 0,
                    yMax: 10,
                    points: m.prop([
                        { x: 0, y: Math.random() * 10 },
                        { x: Math.random() * 10, y: Math.random() * 10 },
                        { x: Math.random() * 10, y: Math.random() * 10 },
                        { x: Math.random() * 10, y: Math.random() * 10 },
                        { x: Math.random() * 10, y: Math.random() * 10 },
                        { x: Math.random() * 10, y: Math.random() * 10 },
                        { x: Math.random() * 10, y: Math.random() * 10 },
                        { x: Math.random() * 10, y: Math.random() * 10 },
                        { x: 10, y: Math.random() * 10 },
                    ])
                },
            ]
        }
    };
    function ChartRenderer(vm) {
        this.vm = vm;
        this.CONFIG = {
            axesGutterPx: 25,
            axesColor: '#999',
            axesWidth: 1,
            pointColor: '#8e44ad',
            lineColor: '#9b59b6',
            lineWidth: 1,
            pointRadius: 3
        };
    }
    ChartRenderer.render = function (vm, canvas) {
        new ChartRenderer(vm).render(canvas);
    };
    ChartRenderer.prototype.render = function (canvas) {
        this.canvas = canvas;
        this._clear();
        this._drawAxes();
        this._drawLines();
    };
    ChartRenderer.prototype._clear = function () {
        this.canvas.getContext('2d').clearRect(0, 0, 1000, 1000);
    };
    ChartRenderer.prototype._drawAxes = function () {
        var canvas = this.canvas, gutter = this.CONFIG.axesGutterPx, context = canvas.getContext('2d');
        context.beginPath();
        context.translate(0.5, 0.5);
        context.moveTo(gutter, 0);
        context.lineTo(gutter, canvas.height - gutter);
        context.lineTo(canvas.width - gutter, canvas.height - gutter);
        context.strokeStyle = this.CONFIG.axesColor;
        context.lineWidth = this.CONFIG.axesWidth;
        context.stroke();
    };
    ChartRenderer.prototype._drawLines = function () {
        var canvas = this.canvas, context = canvas.getContext('2d');
        context.fillStyle = this.CONFIG.pointColor;
        context.strokeStyle = this.CONFIG.lineColor;
        context.lineWidth = this.CONFIG.lineWidth;
        var points = _.sortBy(this.vm.points(), 'x'), self = this;
        points.forEach(function (point) {
            context.beginPath();
            drawCircle(context, {
                centerX: self._xValueToPixels(point.x),
                centerY: self._yValueToPixels(point.y),
                radius: self.CONFIG.pointRadius
            });
            context.fill();
        });
        context.beginPath();
        points.forEach(function (point) {
            context.lineTo(self._xValueToPixels(point.x), self._yValueToPixels(point.y));
        });
        context.stroke();
    };
    ChartRenderer.prototype._xValueToPixels = function (x) {
        var gutter = this.CONFIG.axesGutterPx, percent = (x - this.vm.xMin) / (this.vm.xMax - this.vm.xMin);
        return gutter + ((this.canvas.width - gutter) * percent);
    };
    ChartRenderer.prototype._yValueToPixels = function (y) {
        var gutter = this.CONFIG.axesGutterPx, percent = (y - this.vm.yMin) / (this.vm.yMax - this.vm.yMin), downwards = (this.canvas.height - gutter) * percent;
        // Because y goes downwards instead of upwards by default.
        return (this.canvas.height - downwards) - gutter;
    };
    var OrionApp = {
        controller: function () {
            ViewModelInput.connect().then(function (input) {
                input.onUpdate(function (update) {
                    console.log('Got viewmodel update', update);
                });
                input.onError(function () {
                    console.log('Error! Could not open view model input');
                });
            });
        },
        view: function () {
            var self = this, series = vm.remote.series, charts = vm.remote.charts;
            return [
                m('.l-sidebar', [
                    m('.vertical-list', series.map(this.renderSeries.bind(this)))
                ]),
                m('.l-primary', charts.map(this.renderChart.bind(this)))
            ];
        },
        renderSeries: function (seriesVm) {
            return m('.vertical-list__item', [
                m('.series-label.u-unselectable', [
                    m('.series-label__name', seriesVm.name),
                    m('.series-label__value', seriesVm.value)
                ])
            ]);
        },
        renderChart: function (chartVm) {
            return m('.chart', [
                m('h1.chart__title', chartVm.title),
                m('.chart__body', [
                    m('canvas', { config: this.configChart.bind(this, chartVm) })
                ])
            ]);
        },
        configChart: function (chartVm, canvas, isInitialized) {
            if (!isInitialized) {
                canvas.parentElement.addEventListener('resize', function () {
                    resizeCanvasToParent(canvas);
                    ChartRenderer.render(chartVm, canvas);
                });
            }
            resizeCanvasToParent(canvas);
            ChartRenderer.render(chartVm, canvas);
        }
    };
    m.mount(document.getElementById('app'), OrionApp);
}());
