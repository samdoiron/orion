'use strict';

var _createClass = (function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; })();

function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }

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

var ChartRenderer = (function () {
  function ChartRenderer(vm) {
    _classCallCheck(this, ChartRenderer);

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

  _createClass(ChartRenderer, [{
    key: 'render',
    value: function render() {
      this.canvas = canvas;
      this._clear();
      this._drawAxes();
      this._drawData();
    }
  }, {
    key: '_clear',
    value: function _clear() {
      this.canvas.getContext('2d').clearRect(0, 0, 1000, 1000);
    }
  }, {
    key: '_drawAxes',
    value: function _drawAxes() {
      var canvas = this.canvas,
          gutter = this.CONFIG.axesGutterPx,
          context = canvas.getContext('2d');

      context.beginPath();
      context.translate(0.5, 0.5);
      context.moveTo(gutter, 0);
      context.lineTo(gutter, canvas.height - gutter);
      context.lineTo(canvas.width - gutter, canvas.height - gutter);
      context.strokeStyle = this.CONFIG.axesColor;
      context.lineWidth = this.CONFIG.axesWidth;
      context.stroke();
    }
  }, {
    key: '_drawData',
    value: function _drawData() {
      var _this = this;

      var canvas = this.canvas,
          context = canvas.getContext('2d');

      context.fillStyle = this.CONFIG.pointColor;
      context.strokeStyle = this.CONFIG.lineColor;
      context.lineWidth = this.CONFIG.lineWidth;

      var points = _.sortBy(this.vm.points(), 'x');

      points.forEach(function (point) {
        context.beginPath();
        drawCircle(context, {
          centerX: _this._xValueToPixels(point.x),
          centerY: _this._yValueToPixels(point.y),
          radius: _this.CONFIG.pointRadius
        });
        context.fill();
      });

      context.beginPath();
      points.forEach(function (point) {
        context.lineTo(_this._xValueToPixels(point.x), _this._yValueToPixels(point.y));
      });
      context.stroke();
    }
  }, {
    key: '_xValueToPixels',
    value: function _xValueToPixels(x) {
      var gutter = this.CONFIG.axesGutterPx,
          percent = (x - this.vm.xMin) / (this.vm.xMax - this.vm.xMin);
      return gutter + (this.canvas.width - gutter) * percent;
    }
  }, {
    key: '_yValuetoPixels',
    value: function _yValuetoPixels(y) {
      var gutter = this.CONFIG.axesGutterPx,
          percent = (y - this.vm.yMin) / (this.vm.yMax - this.vm.yMin),
          downwards = (this.canvas.height - gutter) * percent;
      // Because y goes downwards instead of upwards by default.
      return this.canvas.height - downwards - gutter;
    }
  }]);

  return ChartRenderer;
})();