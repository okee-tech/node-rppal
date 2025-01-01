use napi::bindgen_prelude::*;

use rppal;
use tokio;

#[napi]
pub struct I2C(rppal::i2c::I2c);

#[napi]
pub struct Capabilities(rppal::i2c::Capabilities);

#[napi]
impl I2C {
  #[napi(constructor)]
  pub fn new(bus: Option<u8>) -> Result<Self> {
    match bus {
      Some(bus) => rppal::i2c::I2c::with_bus(bus),
      None => rppal::i2c::I2c::new(),
    }
    .map(Self)
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to initialize I2C: {}", e),
      )
    })
  }

  #[napi(getter, js_name = "capabilities")]
  pub fn bus(&self) -> Capabilities {
    Capabilities(self.0.capabilities())
  }

  // #[napi]
  // pub async fn read(&self, address: u8, len: usize) -> Result<Vec<u8>> {
  //   let mut buf = vec![0; len];
  //   self.0.read(address, &mut buf).await?;
  //   Ok(buf)
  // }
}

#[napi]
impl Capabilities {
  #[napi(getter)]
  pub fn addr_10bit(&self) -> bool {
    self.0.addr_10bit()
  }
  #[napi(getter)]
  pub fn i2c_block_read(&self) -> bool {
    self.0.i2c_block_read()
  }
  #[napi(getter)]
  pub fn i2c_block_write(&self) -> bool {
    self.0.i2c_block_write()
  }
  #[napi(getter)]
  pub fn smbus_quick_command(&self) -> bool {
    self.0.smbus_quick_command()
  }
  #[napi(getter)]
  pub fn smbus_receive_byte(&self) -> bool {
    self.0.smbus_receive_byte()
  }
  #[napi(getter)]
  pub fn smbus_send_byte(&self) -> bool {
    self.0.smbus_send_byte()
  }
  #[napi(getter)]
  pub fn smbus_read_byte(&self) -> bool {
    self.0.smbus_read_byte()
  }
  #[napi(getter)]
  pub fn smbus_write_byte(&self) -> bool {
    self.0.smbus_write_byte()
  }
  #[napi(getter)]
  pub fn smbus_read_word(&self) -> bool {
    self.0.smbus_read_word()
  }
  #[napi(getter)]
  pub fn smbus_write_word(&self) -> bool {
    self.0.smbus_write_word()
  }
  #[napi(getter)]
  pub fn smbus_process_call(&self) -> bool {
    self.0.smbus_process_call()
  }
  #[napi(getter)]
  pub fn smbus_block_read(&self) -> bool {
    self.0.smbus_block_read()
  }
  #[napi(getter)]
  pub fn smbus_block_write(&self) -> bool {
    self.0.smbus_block_write()
  }
  #[napi(getter)]
  pub fn smbus_block_process_call(&self) -> bool {
    self.0.smbus_block_process_call()
  }
  #[napi(getter)]
  pub fn smbus_pec(&self) -> bool {
    self.0.smbus_pec()
  }
  #[napi(getter)]
  pub fn smbus_host_notify(&self) -> bool {
    self.0.smbus_host_notify()
  }
}
