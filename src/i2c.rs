use napi::bindgen_prelude::*;

use rppal;
use std::sync::Arc;
use std::time::Duration;
use tokio;
use tokio::sync::Mutex;

#[napi]
pub struct I2CInner {
  i2c: rppal::i2c::I2c,
  is_10bit_address: bool,
  timeout: Option<Duration>,
}

#[napi]
pub struct I2C(Arc<Mutex<I2CInner>>);

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
    .map({
      |i2c| {
        Self(Arc::new(Mutex::new(I2CInner {
          i2c,
          is_10bit_address: false,
          timeout: None,
        })))
      }
    })
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to initialize I2C: {}", e),
      )
    })
  }

  #[napi]
  pub async fn get_capabilities(&self) -> Capabilities {
    let inner = self.0.lock().await;
    Capabilities(inner.i2c.capabilities())
  }

  #[napi]
  pub async fn get_bus(&self) -> u8 {
    let inner = self.0.lock().await;
    inner.i2c.bus()
  }

  #[napi]
  pub async fn get_clock_speed(&self) -> Result<u32> {
    let inner = self.0.lock().await;
    inner.i2c.clock_speed().map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to get clock speed: {}", e),
      )
    })
  }

  #[napi]
  pub async fn get_addr_10bit(&self) -> bool {
    let inner = self.0.lock().await;
    inner.is_10bit_address
  }

  #[napi]
  pub async fn set_addr_10bit(&self, new_value: bool) -> Result<()> {
    let mut inner: tokio::sync::MutexGuard<'_, I2CInner> = self.0.lock().await;
    inner.is_10bit_address = new_value;
    inner.i2c.set_addr_10bit(new_value).map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to set 10-bit address: {}", e),
      )
    })
  }

  #[napi]
  pub async fn get_timeout_millis(&self) -> Result<Option<u32>> {
    let inner = self.0.lock().await;
    let Some(timeout) = inner.timeout else {
      return Ok(None);
    };

    let millis = timeout.as_millis().try_into();
    match millis {
      Ok(millis) => Ok(Some(millis)),
      Err(_) => Err(Error::new(
        Status::GenericFailure,
        "Failed to convert timeout to milliseconds",
      )),
    }
  }

  #[napi]
  pub async fn set_timeout_millis(&self, new_value: u32) -> Result<()> {
    let mut inner = self.0.lock().await;
    let Ok(millis) = new_value.try_into() else {
      return Err(Error::new(
        Status::GenericFailure,
        "Failed to convert timeout to Duration",
      ));
    };

    let duration = Duration::from_millis(millis);
    inner.timeout = Some(duration);

    inner.i2c.set_timeout(new_value).map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to set timeout: {}", e),
      )
    })
  }

  #[napi]
  pub async fn read(&self, address: u16, buffer_size: Option<u32>) -> Result<Vec<u8>> {
    let i2c_ref = self.0.clone();
    tokio::task::spawn_blocking(move || {
      let mut inner = tokio::runtime::Handle::current().block_on(i2c_ref.lock());

      let buffer_size = buffer_size.unwrap_or(256);
      let Ok(buffer_size) = buffer_size.try_into() else {
        return Err(Error::new(
          Status::GenericFailure,
          "Failed to convert buffer size to usize",
        ));
      };
      let mut buffer = vec![0u8; buffer_size];

      inner.i2c.set_slave_address(address).map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to set slave address: {}", e),
        )
      })?;
      match inner.i2c.read(&mut buffer) {
        Ok(bytes_read) => {
          buffer.truncate(bytes_read);
          Ok(buffer)
        }
        Err(e) => Err(Error::new(
          Status::GenericFailure,
          format!("Failed to read from I2C: {}", e),
        )),
      }
    })
    .await
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to await read task: {}", e),
      )
    })?
  }

  #[napi]
  pub async fn write(&self, address: u16, data: Vec<u8>) -> Result<()> {
    let i2c_ref = self.0.clone();
    tokio::task::spawn_blocking(move || {
      let mut inner = tokio::runtime::Handle::current().block_on(i2c_ref.lock());

      inner.i2c.set_slave_address(address).map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to set slave address: {}", e),
        )
      })?;
      match inner.i2c.write(&data) {
        Ok(bytes_written) => {
          if bytes_written != data.len() {
            return Err(Error::new(
              Status::GenericFailure,
              format!(
                "Failed to write all bytes to I2C, only {} out of {} written",
                bytes_written,
                data.len()
              ),
            ));
          }
          Ok(())
        }
        Err(e) => Err(Error::new(
          Status::GenericFailure,
          format!("Failed to write to I2C: {}", e),
        )),
      }
    })
    .await
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to await write task: {}", e),
      )
    })?
  }

  #[napi]
  pub async fn write_read(
    &self,
    address: u16,
    write_data: Vec<u8>,
    read_buffer_size: Option<u32>,
  ) -> Result<Vec<u8>> {
    let i2c_ref = self.0.clone();
    tokio::task::spawn_blocking(move || {
      let mut inner = tokio::runtime::Handle::current().block_on(i2c_ref.lock());

      let read_buffer_size = read_buffer_size.unwrap_or(256);
      let Ok(read_buffer_size) = read_buffer_size.try_into() else {
        return Err(Error::new(
          Status::GenericFailure,
          "Failed to convert read buffer size to usize",
        ));
      };
      let mut read_buffer = vec![0u8; read_buffer_size];

      inner.i2c.set_slave_address(address).map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to set slave address: {}", e),
        )
      })?;
      match inner.i2c.write_read(&write_data, &mut read_buffer) {
        Ok(_) => Ok(read_buffer),
        Err(e) => Err(Error::new(
          Status::GenericFailure,
          format!("Failed to write-read to I2C: {}", e),
        )),
      }
    })
    .await
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to await write-read task: {}", e),
      )
    })?
  }

  #[napi]
  pub async fn block_read(
    &self,
    address: u16,
    command: u8,
    buffer_size: Option<u32>,
  ) -> Result<Vec<u8>> {
    let i2c_ref = self.0.clone();
    tokio::task::spawn_blocking(move || {
      let mut inner = tokio::runtime::Handle::current().block_on(i2c_ref.lock());

      let buffer_size = buffer_size.unwrap_or(256);
      let Ok(buffer_size) = buffer_size.try_into() else {
        return Err(Error::new(
          Status::GenericFailure,
          "Failed to convert buffer size to usize",
        ));
      };
      let mut buffer = vec![0u8; buffer_size];

      inner.i2c.set_slave_address(address).map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to set slave address: {}", e),
        )
      })?;
      match inner.i2c.block_read(command, &mut buffer) {
        Ok(_) => Ok(buffer),
        Err(e) => Err(Error::new(
          Status::GenericFailure,
          format!("Failed to block-read from I2C: {}", e),
        )),
      }
    })
    .await
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to await block-read task: {}", e),
      )
    })?
  }

  #[napi]
  pub async fn block_write(&self, address: u16, command: u8, data: Vec<u8>) -> Result<()> {
    let i2c_ref = self.0.clone();
    tokio::task::spawn_blocking(move || {
      let mut inner = tokio::runtime::Handle::current().block_on(i2c_ref.lock());

      inner.i2c.set_slave_address(address).map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to set slave address: {}", e),
        )
      })?;
      inner.i2c.block_write(command, &data).map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Failed to block-write to I2C: {}", e),
        )
      })
    })
    .await
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to await block-write task: {}", e),
      )
    })?
  }
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
