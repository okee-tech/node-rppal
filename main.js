// main.js
let rppal;

try {
  rppal = require("./index.js");
} catch {
  rppal = require("./mock/index.js");
}

module.exports = rppal;
