define(['exports'], function (exports) {
  'use strict';

  Object.defineProperty(exports, "__esModule", {
    value: true
  });
  exports.default = {
    view: function view(_ctrl, given) {
      var args = {
        isActive: given.isActive,
        message: given.message || 'no message',
        buttons: given.buttons || [{ text: 'OK', onclick: _.noop }]
      };
      return m('div', [this.backgroundFader(args.isActive), this.dialog(args)]);
    },
    backgroundFader: function backgroundFader(isActive) {
      if (isActive) {
        return m('.fader--active');
      } else {
        return m('.fader');
      }
    },
    dialog: function dialog(args) {
      if (!args.isActive) return [];
      return m('.dialog', [this.textArea(args.message), this.buttons(args.buttons)]);
    },
    textArea: function textArea(text) {
      return m('.dialog__text-area', [this.message(text)]);
    },
    message: function message(_message) {
      return m('h1.dialog__message', _message);
    },
    buttons: function buttons(_buttons) {
      return m('.dialog__buttons', _buttons.map(this.button.bind(this)));
    },
    button: function button(_button) {
      return m('button.dialog__button', {
        onclick: _button.onclick
      }, _button.text);
    }
  };
});
//# sourceMappingURL=dialog-component.js.map