let rppal;

try {
  rppal = require("./index.js");
} catch {
  rppal = require("./mock");
}

export default rppal;
