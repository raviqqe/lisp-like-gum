use std::ops::{Add, Sub, Div};



#[derive(Copy, Clone, Debug)]
pub struct Weight {
  value: u64,
}

impl Weight {
  fn new(v: u64) {
    Weight { value: v }
  }

  fn split(&mut self) -> Option<Self> {
    if self.value == 1 {
      return None
    }

    let w = self / 2;
    self -= w;

    Some(w)
  }
}

impl Default for Weight {
  fn default() -> Weight {
    Weight { value: 2^16 }
  }
}

impl Add for Weight {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Weight { value: self.value + rhs.value }
  }
}

impl Sub for Weight {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Weight { value: self.value - rhs.value }
  }
}

impl Div for Weight {
  type Output = Self;

  fn div<T: Into<u8>>(self, rhs: T) -> Self::Output {
    Weight { value: self.value / rhs }
  }
}
