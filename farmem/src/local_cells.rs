use std::any::Any;
use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};

use local_cell::LocalCell;
use local_id::LocalId;
use local_id_manager::LocalIdManager;



#[derive(Debug, Default)]
pub struct LocalLocalCells {
  local_cells: BTreeMap<LocalId, LocalCell>,
  id_manager: LocalIdManager,
}

impl LocalLocalCells {
  pub fn new() -> Self {
    LocalLocalCells::default()
  }

  pub fn store<T: Any>(&mut self, o: T) -> LocalId {
    let i = self.id_manager.new_id();
    self.local_cells.insert(i, LocalCell::new(o));
    i
  }

  pub fn delete(&mut self, i: LocalId) {
    let _ = self.local_cells.remove(&i);
    self.id_manager.return_id(i);
  }
}

impl Index<LocalId> for LocalLocalCells {
  type Output = LocalCell;

  fn index(&self, i: LocalId) -> &Self::Output {
    self.local_cells.index(&i)
  }
}

impl IndexMut<LocalId> for LocalLocalCells {
  fn index_mut(&mut self, i: LocalId) -> &mut LocalCell {
    self.local_cells.get_mut(&i).unwrap()
  }
}
