import test from "ava";

import { Bias, Gpio, Mode } from "../index.js";

const gpio = new Gpio();
const p1 = gpio.get(1);

p1.bias = Bias.PullDown;
p1.mode = Mode.Output;
p1.level = 1;
