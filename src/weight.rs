use std::ops::{Add, Sub};



pub struct Weight {
  value: u64,
}

impl Weight {
  fn new(v: u64) {
    Weight { value: v }
  }
}

impl Default for Weight {
  fn default() -> Weight {
    Weight { value: 2^16 }
  }
}

impl Add for Weight {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Weight { value: self.value + rhs.value }
  }
}

impl Sub for Weight {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Weight { value: self.value - rhs.value }
  }
}
