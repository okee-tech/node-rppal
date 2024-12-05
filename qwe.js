// const qwe = require("./main.js");
// console.log(qwe);

// const gpio = new Gpio();
// const pin = gpio.get(4);
// pin.mode = 1;

const { Gpio } = require("./main.js");
const gpio = new Gpio();
const pin = gpio.get(4);
pin.mode = 1;
pin.setPwm(1, 1);
