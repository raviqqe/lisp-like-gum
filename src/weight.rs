use std::ops::{Add, Sub, Div};



#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Weight {
  value: u64,
}

impl Weight {
  pub fn new(v: u64) -> Weight {
    Weight { value: v }
  }

  pub fn split(&mut self) -> Option<Self> {
    if self.value == 1 {
      return None
    }

    let w = self / 2;
    *self -= w;

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

impl Div<u64> for Weight {
  type Output = Self;

  fn div(self, rhs: u64) -> Self::Output {
    Weight { value: self.value / rhs }
  }
}
