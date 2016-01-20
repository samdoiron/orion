export default {
  view(_ctrl, given) {
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

  backgroundFader(isActive) {
    if (isActive)  {
      return m('.fader--active')
    } else {
      return m('.fader')
    }
  },

  dialog(args) {
    if (!args.isActive) return []
    return m('.dialog', [
      this.textArea(args.message), 
      this.buttons(args.buttons)
    ])
  },

  textArea(text) {
    return m('.dialog__text-area', [
      this.message(text)
    ])
  },

  message(message) {
    return m('h1.dialog__message', message)
  },

  buttons(buttons) {
    return m('.dialog__buttons', 
      buttons.map(this.button.bind(this))
    )
  },

  button(button) {
    return m('button.dialog__button', {
      onclick: button.onclick
    }, button.text)
  },
};
