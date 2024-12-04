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
- PWM (Pulse Width Modulation)
- I2C (Inter-Integrated Circuit)
- SPI (Serial Peripheral Interface)
- UART (Universal Asynchronous Receiver/Transmitter)

## Usage

### GPIO Example

```javascript
import { Gpio, Mode, Bias } from "@okee-tech/rppal";

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
```

## License

MIT

## Contributing

Issues and pull requests are welcome on GitHub.
