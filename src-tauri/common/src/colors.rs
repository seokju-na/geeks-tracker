use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RGB {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl RGB {
  pub fn white() -> Self {
    Self {
      r: 0xff,
      g: 0xff,
      b: 0xff,
    }
  }

  pub fn black() -> Self {
    Self {
      r: 0x00,
      g: 0x00,
      b: 0x00,
    }
  }

  pub fn to_hex(&self) -> String {
    format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
  }
}
