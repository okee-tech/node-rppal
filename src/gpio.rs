use napi::bindgen_prelude::*;

use rppal;

#[napi]
pub struct Gpio(rppal::gpio::Gpio);

#[napi]
pub struct Pin(rppal::gpio::IoPin, rppal::gpio::Bias);

#[napi]
#[repr(u8)]
pub enum Level {
  Low = 0,
  High = 1,
}

impl From<rppal::gpio::Level> for Level {
  fn from(level: rppal::gpio::Level) -> Self {
    match level {
      rppal::gpio::Level::Low => Level::Low,
      rppal::gpio::Level::High => Level::High,
    }
  }
}

impl Into<rppal::gpio::Level> for Level {
  fn into(self) -> rppal::gpio::Level {
    match self {
      Level::Low => rppal::gpio::Level::Low,
      Level::High => rppal::gpio::Level::High,
    }
  }
}

#[napi]
pub enum Mode {
  Input,
  Output,
  Alt0,
  Alt1,
  Alt2,
  Alt3,
  Alt4,
  Alt5,
  Alt6,
  Alt7,
  Alt8,
  Null,
}

impl From<rppal::gpio::Mode> for Mode {
  fn from(mode: rppal::gpio::Mode) -> Self {
    match mode {
      rppal::gpio::Mode::Input => Mode::Input,
      rppal::gpio::Mode::Output => Mode::Output,
      rppal::gpio::Mode::Alt0 => Mode::Alt0,
      rppal::gpio::Mode::Alt1 => Mode::Alt1,
      rppal::gpio::Mode::Alt2 => Mode::Alt2,
      rppal::gpio::Mode::Alt3 => Mode::Alt3,
      rppal::gpio::Mode::Alt4 => Mode::Alt4,
      rppal::gpio::Mode::Alt5 => Mode::Alt5,
      rppal::gpio::Mode::Alt6 => Mode::Alt6,
      rppal::gpio::Mode::Alt7 => Mode::Alt7,
      rppal::gpio::Mode::Alt8 => Mode::Alt8,
      rppal::gpio::Mode::Null => Mode::Null,
    }
  }
}

impl Into<rppal::gpio::Mode> for Mode {
  fn into(self) -> rppal::gpio::Mode {
    match self {
      Mode::Input => rppal::gpio::Mode::Input,
      Mode::Output => rppal::gpio::Mode::Output,
      Mode::Alt0 => rppal::gpio::Mode::Alt0,
      Mode::Alt1 => rppal::gpio::Mode::Alt1,
      Mode::Alt2 => rppal::gpio::Mode::Alt2,
      Mode::Alt3 => rppal::gpio::Mode::Alt3,
      Mode::Alt4 => rppal::gpio::Mode::Alt4,
      Mode::Alt5 => rppal::gpio::Mode::Alt5,
      Mode::Alt6 => rppal::gpio::Mode::Alt6,
      Mode::Alt7 => rppal::gpio::Mode::Alt7,
      Mode::Alt8 => rppal::gpio::Mode::Alt8,
      Mode::Null => rppal::gpio::Mode::Null,
    }
  }
}

#[napi]
pub enum Bias {
  Off,
  PullDown,
  PullUp,
}

impl From<rppal::gpio::Bias> for Bias {
  fn from(bias: rppal::gpio::Bias) -> Self {
    match bias {
      rppal::gpio::Bias::Off => Bias::Off,
      rppal::gpio::Bias::PullDown => Bias::PullDown,
      rppal::gpio::Bias::PullUp => Bias::PullUp,
    }
  }
}

impl Into<rppal::gpio::Bias> for Bias {
  fn into(self) -> rppal::gpio::Bias {
    match self {
      Bias::Off => rppal::gpio::Bias::Off,
      Bias::PullDown => rppal::gpio::Bias::PullDown,
      Bias::PullUp => rppal::gpio::Bias::PullUp,
    }
  }
}

#[napi]
impl Gpio {
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    rppal::gpio::Gpio::new().map(Self).map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Was not able to obtain GPIO device, {}", e),
      )
    })
  }

  #[napi]
  pub fn get(&self, pin: u8) -> Result<Pin> {
    self
      .0
      .get(pin)
      .map(|p| Pin(p.into_io(rppal::gpio::Mode::Null), rppal::gpio::Bias::Off))
      .map_err(|e| {
        Error::new(
          Status::GenericFailure,
          format!("Was not able to obtain GPIO pin, {}", e),
        )
      })
  }
}

#[napi]
impl Pin {
  #[napi(getter)]
  pub fn pin(&self) -> u8 {
    self.0.pin()
  }

  #[napi(getter, js_name = "level")]
  pub fn get_level(&self) -> Level {
    self.0.read().into()
  }

  #[napi(setter, js_name = "level")]
  pub fn set_level(&mut self, new_level: Level) -> Result<()> {
    if self.0.mode() == rppal::gpio::Mode::Input {
      return Err(Error::new(
        Status::GenericFailure,
        "Cannot set level on input pin",
      ));
    }

    self.0.write(new_level.into());
    Ok(())
  }

  #[napi(getter, js_name = "mode")]
  pub fn get_mode(&self) -> Mode {
    self.0.mode().into()
  }

  #[napi(setter, js_name = "mode")]
  pub fn set_mode(&mut self, new_mode: Mode) {
    self.0.set_mode(new_mode.into())
  }

  #[napi(getter, js_name = "bias")]
  pub fn get_bias(&self) -> Bias {
    self.1.into()
  }

  #[napi(setter, js_name = "bias")]
  pub fn set_bias(&mut self, new_bias: Bias) {
    self.0.set_bias(new_bias.into())
  }
}
