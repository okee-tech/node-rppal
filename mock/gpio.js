// mock/gpio.js
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
    console.log(`Mock: Pin ${this.#pin} value → ${newValue}`);
    this.#value = newValue;
  }

  get mode() {
    return this.#mode;
  }
  set mode(newMode) {
    console.log(`Mock: Pin ${this.#pin} mode → ${newMode}`);
    this.#mode = newMode;
  }

  get bias() {
    return this.#bias;
  }
  set bias(newBias) {
    console.log(`Mock: Pin ${this.#pin} bias → ${newBias}`);
    this.#bias = newBias;
  }

  setPwm(frequency, duty) {
    console.log(
      `Mock: Pin ${this.#pin} PWM → freq: ${frequency}, duty: ${duty}`
    );
    this.#pwm = { frequency, duty, enabled: true };
  }

  clearPwm() {
    console.log(`Mock: Pin ${this.#pin} PWM cleared`);
    this.#pwm = { frequency: 0, duty: 0, enabled: false };
  }

  get softPwm() {
    return this.#pwm;
  }
}

export class Gpio {
  #pins = new Map();

  get(pin) {
    if (!this.#pins.has(pin)) {
      this.#pins.set(pin, new Pin(pin));
    }
    return this.#pins.get(pin);
  }
}
