// main.js
let rppal;

try {
  rppal = require("./index.js");
} catch (err) {
  global.__RPPAL_MOCK_WARNED__ ??= false;
  if (!global.__RPPAL_MOCK_WARNED__) {
    console.warn("[RPPAL] Failed to load native implementation: ", err);
    console.warn(
      "[RPPAL] Using mock implementation - GPIO operations will be simulated"
    );
    global.__RPPAL_MOCK_WARNED__ = true;
  }

  rppal = require("./mock/index.js");
}

module.exports.Gpio = rppal.Gpio;
module.exports.Pin = rppal.Pin;
module.exports.Level = rppal.Level;
module.exports.Mode = rppal.Mode;
module.exports.Bias = rppal.Bias;
module.exports.I2C = rppal.I2C;
