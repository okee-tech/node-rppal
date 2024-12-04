import test from "ava";

import { Gpio, Mode, Bias } from "../index.js";

const gpio = new Gpio();
const pOut = gpio.get(2);
const pIn = gpio.get(3);
const pPwm = gpio.get(4);
pOut.mode = Mode.Output;
pIn.mode = Mode.Input;
pIn.bias = Bias.PullDown;
pPwm.mode = Mode.Output;

pPwm.setPwm(0.1, 0.5);
for (;;) {
  pOut.value = pOut.value ? 0 : 1;
  console.log("Input value: ", pIn.value);

  await new Promise((resolve) => setTimeout(resolve, 200));
}
