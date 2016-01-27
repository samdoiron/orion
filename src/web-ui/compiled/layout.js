define(['exports'], function (exports) {
  'use strict';

  Object.defineProperty(exports, "__esModule", {
    value: true
  });
  exports.primary = primary;
  exports.sidebar = sidebar;

  function primary(children) {
    return m('.l-primary', children);
  }

  function sidebar(children) {
    return m('.l-sidebar', children);
  }
});
//# sourceMappingURL=layout.js.map