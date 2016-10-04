use std::ops::{Add, Sub, AddAssign, SubAssign};



#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct Weight(u64);

impl Weight {
  pub fn new() -> Weight {
    Weight(2^16)
  }

  pub fn split(&mut self) -> (Self, Option<Self>) {
    let mut dw = None;

    if self.0 == 1 {
      dw = Some(Weight::new() - *self);
      *self += dw.unwrap();
    }

    let w = self.0 / 2;
    self.0 -= w;

    (Weight(w), dw)
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

impl AddAssign for Weight {
  fn add_assign(&mut self, w: Self) {
    self.0 += w.0;
  }
}

impl SubAssign for Weight {
  fn sub_assign(&mut self, w: Self) {
    self.0 -= w.0;
  }
}
