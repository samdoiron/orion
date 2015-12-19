function removeUnit(str) {
  return str.replace(/[^0-9.]/g, '');
}

function resizeCanvasToParent(canvas) {
  let parentStyle = getComputedStyle(canvas.parentElement);
  canvas.width = removeUnit(parentStyle.width);
  canvas.height = removeUnit(parentStyle.height);
}

function drawCircle(context, opts) {
  context.arc(opts.centerX, opts.centerY, opts.radius, 0, Math.PI * 2)
}

class ChartRenderer {
  static render() {
    new ChartRenderer(vm).render(canvas)
  }

  constructor(vm) {
    this.vm = vm
    this.CONFIG = {
      axesGutterPx: 25,
      axesColor: '#999',
      axesWidth: 1,
      pointColor: '#8e44ad',
      lineColor: '#9b59b6',
      lineWidth: 1,
      pointRadius: 3
    }
  }

  render() {
    this.canvas = canvas;
    this._clear()
    this._drawAxes()
    this._drawData()
  }

  _clear() {
    this.canvas.getContext('2d').clearRect(0, 0, 1000, 1000)
  }

  _drawAxes() {
    let canvas = this.canvas,
      gutter = this.CONFIG.axesGutterPx,
      context = canvas.getContext('2d');

    context.beginPath();
    context.translate(0.5, 0.5)
    context.moveTo(gutter, 0)
    context.lineTo(gutter, canvas.height - gutter)
    context.lineTo(canvas.width - gutter, canvas.height - gutter)
    context.strokeStyle = this.CONFIG.axesColor
    context.lineWidth = this.CONFIG.axesWidth;
    context.stroke()
  }

  _drawData() {
    let canvas = this.canvas,
      context = canvas.getContext('2d');

    context.fillStyle = this.CONFIG.pointColor
    context.strokeStyle = this.CONFIG.lineColor
    context.lineWidth = this.CONFIG.lineWidth

    let points = _.sortBy(this.vm.points(), 'x')

    points.forEach(point => {
      context.beginPath()
      drawCircle(context, {
        centerX: this._xValueToPixels(point.x),
        centerY: this._yValueToPixels(point.y),
        radius: this.CONFIG.pointRadius
      })
      context.fill();
    })

    context.beginPath();
    points.forEach(point => {
      context.lineTo(
        this._xValueToPixels(point.x),
        this._yValueToPixels(point.y)
      )
    })
    context.stroke();
 }

 _xValueToPixels(x) {
    let gutter = this.CONFIG.axesGutterPx,
      percent = (x - this.vm.xMin) / (this.vm.xMax - this.vm.xMin);
    return gutter + ((this.canvas.width - gutter) * percent);
 }

 _yValuetoPixels(y) {
   let gutter = this.CONFIG.axesGutterPx,
     percent = (y - this.vm.yMin) / (this.vm.yMax - this.vm.yMin),
     downwards = (this.canvas.height - gutter) * percent
   // Because y goes downwards instead of upwards by default.
   return (this.canvas.height - downwards) - gutter
 }
}
