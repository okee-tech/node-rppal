# @okee-tech/rppal

Node.js bindings for the [rppal](https://github.com/golemparts/rppal) Rust crate to control Raspberry Pi peripherals.

## Requirements

- Raspberry Pi hardware
- Node.js 16.0.0 or higher

## Installation

```bash
npm install @okee-tech/rppal
```

## Features

- GPIO (General Purpose Input/Output)
- GPIO Software PWM

<!--
- PWM (Pulse Width Modulation)
- I2C (Inter-Integrated Circuit)
- SPI (Serial Peripheral Interface)
- UART (Universal Asynchronous Receiver/Transmitter) -->

## Usage

### GPIO Example

```javascript
import { Gpio, Mode, Bias } from "@okee-tech/rppal";

const gpio = new Gpio();
const pOut = gpio.get(2);
const pIn = gpio.get(3);
const pPwm = gpio.get(4);
pOut.mode = Mode.Output;
pIn.mode = Mode.Input;
pIn.bias = Bias.PullDown;
pPwm.mode = Mode.Output;

pPwm.setPwm(10, 0.5);
for (;;) {
  pOut.value = pOut.value ? 0 : 1;
  console.log("Input value: ", pIn.value);

  await new Promise((resolve) => setTimeout(resolve, 200));
}
```

## License

MIT

## Contributing

Issues and pull requests are welcome on GitHub.
