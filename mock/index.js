// mock/index.js
const gpio = require("./gpio.js");
const i2c = require("./i2c.js");

module.exports = {
  ...gpio,
  ...i2c,
};
