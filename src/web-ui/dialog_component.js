// Example:

let DialogComponent = {
  view: function (_ctrl, given) {
    let args = {
      isActive: given.isActive,
      message: given.message || 'no message',
      buttons: given.buttons || [{ text: 'OK', onclick: _.noop }] 
    }
    return m('div', [
      this.backgroundFader(args.isActive),
      this.dialog(args)
    ])
  },

  backgroundFader: function (isActive) {
    if (isActive)  {
      return m('.fader--active')
    } else {
      return m('.fader')
    }
  },

  dialog: function (args) {
    if (!args.isActive) return []
    return m('.dialog', [
      this.textArea(args.message), 
      this.buttons(args.buttons)
    ])
  },

  textArea: function (text) {
    return m('.dialog__text-area', [
      this.message(text)
    ])
  },

  message: function (message) {
    return m('h1.dialog__message', message)
  },

  buttons: function (buttons) {
    return m('.dialog__buttons', 
      buttons.map(this.button.bind(this))
    )
  },

  button: function (button) {
    return m('button.dialog__button', {
      onclick: button.onclick
    }, button.text)
  },
}
