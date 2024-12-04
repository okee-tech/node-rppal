import test from "ava";

import { Gpio, Mode, Bias } from "../index.js";

const gpio = new Gpio();
const pIn = gpio.get(2);
pIn.mode = Mode.Input;
pIn.bias = Bias.PullDown;

const pOut = gpio.get(1);
pOut.mode = Mode.Output;

for (;;) {
  pOut.level = !pOut.level;
  console.log(`pOut.level: ${pOut.level}`);
  await new Promise((resolve) => setTimeout(resolve, 1000));
}
