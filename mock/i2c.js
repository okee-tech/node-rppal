// mock/i2c.js
const DEBUG = process.env.RPPAL_MOCK_DEBUG === "1";

const log = (msg) => {
  if (DEBUG) console.debug(`[RPPAL Mock] ${msg}`);
};

class Capabilities {
  constructor() {
    this.addr10Bit = true;
    this.i2CBlockRead = true;
    this.i2CBlockWrite = true;
    this.smbusQuickCommand = true;
    this.smbusReceiveByte = true;
    this.smbusSendByte = true;
    this.smbusReadByte = true;
    this.smbusWriteByte = true;
    this.smbusReadWord = true;
    this.smbusWriteWord = true;
    this.smbusProcessCall = true;
    this.smbusBlockRead = true;
    this.smbusBlockWrite = true;
    this.smbusBlockProcessCall = true;
    this.smbusPec = true;
    this.smbusHostNotify = true;
  }
}

class I2C {
  constructor(bus = 1) {
    this.bus = bus;
    this.clockSpeed = 100000; // Default to 100kHz
    this.addr10Bit = false;
    this.timeoutMillis = null;
    this.capabilities = new Capabilities();
    this.deviceStorage = new Map();
  }

  async getCapabilities() {
    return this.capabilities;
  }

  async getBus() {
    return this.bus;
  }

  async getClockSpeed() {
    return this.clockSpeed;
  }

  async getAddr10Bit() {
    return this.addr10Bit;
  }

  async setAddr10Bit(newValue) {
    this.addr10Bit = newValue;
  }

  async getTimeoutMillis() {
    return this.timeoutMillis;
  }

  async setTimeoutMillis(newValue) {
    if (newValue < 0) throw new Error("Timeout value cannot be negative");

    this.timeoutMillis = newValue;
  }

  async read(address, bufferSize = 1) {
    this.validateAddress(address);

    // Return mock data from device storage or generate mock data
    const deviceData = this.deviceStorage.get(address) || [];
    return deviceData
      .slice(0, bufferSize)
      .concat(Array(Math.max(0, bufferSize - deviceData.length)).fill(0));
  }

  async write(address, data) {
    this.validateAddress(address);
    this.validateData(data);

    // Store the written data for later reading
    this.deviceStorage.set(address, [...data]);
  }

  async writeRead(address, writeData, readBufferSize = 1) {
    await this.write(address, writeData);
    return this.read(address, readBufferSize);
  }

  async blockRead(address, command, bufferSize = 1) {
    this.validateAddress(address);
    this.validateCommand(command);

    // Generate mock data based on command and address
    const mockData = Array(bufferSize)
      .fill(0)
      .map((_, i) => (command + i) % 256);

    return mockData;
  }

  async blockWrite(address, command, data) {
    this.validateAddress(address);
    this.validateCommand(command);
    this.validateData(data);

    // Store the command and data for potential later reading
    const combinedData = [command, ...data];
    this.deviceStorage.set(address, combinedData);
  }

  validateAddress(address) {
    if (!Number.isInteger(address)) {
      throw new Error("Address must be an integer");
    }

    const maxAddr = this.addr10Bit ? 1023 : 127;
    if (address < 0 || address > maxAddr) {
      throw new Error(
        `Address must be between 0 and ${maxAddr} for ${
          this.addr10Bit ? "10-bit" : "7-bit"
        } addressing`
      );
    }
  }

  validateCommand(command) {
    if (!Number.isInteger(command) || command < 0 || command > 255) {
      throw new Error("Command must be an integer between 0 and 255");
    }
  }

  validateData(data) {
    if (!Array.isArray(data) || data.length === 0) {
      throw new Error("Data must be a non-empty array of numbers");
    }

    if (
      !data.every((byte) => Number.isInteger(byte) && byte >= 0 && byte <= 255)
    ) {
      throw new Error("All data bytes must be integers between 0 and 255");
    }
  }

  // Method to simulate I2C errors or specific device behaviors
  simulateError(errorType) {
    switch (errorType) {
      case "timeout":
        throw new Error("I2C operation timed out");
      case "nack":
        throw new Error("Device did not acknowledge");
      case "bus_error":
        throw new Error("Bus error occurred");
      default:
        throw new Error("Unknown error type");
    }
  }

  // Method to reset the mock device storage
  resetDevices() {
    this.deviceStorage.clear();
  }
}

module.exports = { I2C, Capabilities };
