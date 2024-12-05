// main.js
let rppal;

try {
  rppal = require("./index.js");
} catch {
  rppal = require("./mock/index.js");
}

module.exports.Gpio = rppal.Gpio;
module.exports.Pin = rppal.Pin;
module.exports.Level = rppal.Level;
module.exports.Mode = rppal.Mode;
module.exports.Bias = rppal.Bias;
