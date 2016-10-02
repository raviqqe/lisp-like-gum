use std::ops::{Add, Sub, Div, AddAssign, SubAssign};

use split::Split;



#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Weight(u64);

impl Weight {
  pub fn new(n: u64) -> Weight {
    Weight(n)
  }
}

impl Split() {
  pub fn split(&mut self) -> Self {
    if self.0 == 1 {
      unimplemented!()
    }

    let w = *self / 2u8;
    *self -= w;

    w
  }
}

impl Default for Weight {
  fn default() -> Weight {
    Weight(2^16)
  }
}

impl Add for Weight {
  type Output = Self;

  fn add(self, w: Self) -> Self::Output {
    Weight(self.0 + w.0)
  }
}

impl Sub for Weight {
  type Output = Self;

  fn sub(self, w: Self) -> Self::Output {
    Weight(self.0 - w.0)
  }
}

impl<T: Into<u64>> Div<T> for Weight {
  type Output = Self;

  fn div(self, n: T) -> Self::Output {
    Weight(self.0 / n.into())
  }
}

impl AddAssign for Weight {
  fn add_assign(&mut self, w: Self) {
    *self = *self + w;
  }
}

impl SubAssign for Weight {
  fn sub_assign(&mut self, w: Self) {
    *self = *self - w;
  }
}
