// mock/index.js
const gpio = require("./gpio.js");

global.__RPPAL_MOCK_WARNED__ = global.__RPPAL_MOCK_WARNED__ || false;
if (!global.__RPPAL_MOCK_WARNED__) {
  console.warn(
    "[RPPAL] Using mock implementation - GPIO operations will be simulated"
  );
  global.__RPPAL_MOCK_WARNED__ = true;
}
module.exports = {
  ...gpio,
};
