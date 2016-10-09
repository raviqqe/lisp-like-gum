use std::collections::BTreeMap;
use std::ops::Index;

use global_address::GlobalAddress;
use global_cell::GlobalCell;
use global_cell::GlobalCell::*;



#[derive(Debug, Default)]
pub struct GlobalCells {
  cells: BTreeMap<GlobalAddress, GlobalCell>,
}

impl GlobalCells {
  pub fn new() -> Self {
    GlobalCells::default()
  }

  pub fn store(&mut self, a: GlobalAddress, c: GlobalCell)  {
    if let Some(Local { .. }) = self.cells.insert(a, c) {
      panic!("")
    }
  }

  pub fn is_local(&self, a: GlobalAddress) -> bool {
    if let Some(&Local { .. }) = self.cells.get(&a) {
      true
    } else {
      false
    }
  }

  pub fn delete(&mut self, i: GlobalAddress) {
    let _ = self.cells.remove(&i);
  }
}

impl Index<GlobalAddress> for GlobalCells {
  type Output = GlobalCell;

  fn index(&self, a: GlobalAddress) -> &Self::Output {
    self.cells.index(&a)
  }
}
