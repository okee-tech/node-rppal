// mock/gpio.js
const DEBUG = process.env.RPPAL_MOCK_DEBUG === "1";

const log = (msg) => {
  if (DEBUG) console.debug(`[RPPAL Mock] ${msg}`);
};

export const Level = {
  Low: 0,
  High: 1,
};

export const Mode = {
  Input: 0,
  Output: 1,
  Alt0: 2,
  Alt1: 3,
  Alt2: 4,
  Alt3: 5,
  Alt4: 6,
  Alt5: 7,
  Alt6: 8,
  Alt7: 9,
  Alt8: 10,
  Null: 11,
};

export const Bias = {
  Off: 0,
  PullDown: 1,
  PullUp: 2,
};

class Pin {
  #pin;
  #value = Level.Low;
  #mode = Mode.Input;
  #bias = Bias.Off;
  #pwm = { frequency: 0, duty: 0, enabled: false };

  constructor(pin) {
    this.#pin = pin;
  }

  get pin() {
    return this.#pin;
  }

  get value() {
    return this.#value;
  }
  set value(newValue) {
    log(`Pin ${this.#pin} value → ${newValue}`);
    this.#value = newValue;
  }

  get mode() {
    return this.#mode;
  }
  set mode(newMode) {
    log(`Pin ${this.#pin} mode → ${newMode}`);
    this.#mode = newMode;
  }

  get bias() {
    return this.#bias;
  }
  set bias(newBias) {
    log(`Pin ${this.#pin} bias → ${newBias}`);
    this.#bias = newBias;
  }

  setPwm(frequency, duty) {
    log(`Pin ${this.#pin} PWM → freq: ${frequency}, duty: ${duty}`);
    this.#pwm = { frequency, duty, enabled: true };
  }

  clearPwm() {
    log(`Pin ${this.#pin} PWM cleared`);
    this.#pwm = { frequency: 0, duty: 0, enabled: false };
  }

  get softPwm() {
    return this.#pwm;
  }
}

class Gpio {
  #pins = new Map();

  constructor() {}

  get(pin) {
    if (!this.#pins.has(pin)) {
      this.#pins.set(pin, new Pin(pin));
    }
    return this.#pins.get(pin);
  }
}

module.exports = { Gpio, Level, Mode, Bias };
