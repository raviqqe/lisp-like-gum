use std::any::Any;
use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};

use cell::{Cell, CellState};
use global_address::GlobalAddress;
use local_id::LocalId;
use local_id_manager::LocalIdManager;



#[derive(Debug, Default)]
pub struct LocalCells {
  cells: BTreeMap<LocalId, Cell>,
  id_manager: LocalIdManager,
}

impl LocalCells {
  pub fn new() -> Self {
    LocalCells::default()
  }

  pub fn store<T: Any>(&mut self, o: T) -> LocalId {
    let i = self.id_manager.new_id();
    self.cells.insert(i, Cell::new(o));
    i
  }

  pub fn mark_moving(&mut self, i: LocalId) -> CellState {
    self.cells.get_mut(&i).unwrap().mark_moving()
  }

  pub fn mark_moved(&mut self, i: LocalId, a: GlobalAddress) {
    self.cells.get_mut(&i).unwrap().mark_moved(a)
  }

  pub fn delete(&mut self, i: LocalId) {
    let _ = self.cells.remove(&i);
    self.id_manager.return_id(i);
  }
}

impl Index<LocalId> for LocalCells {
  type Output = Cell;

  fn index(&self, i: LocalId) -> &Self::Output {
    self.cells.index(&i)
  }
}

impl IndexMut<LocalId> for LocalCells {
  fn index_mut(&mut self, i: LocalId) -> &mut Cell {
    self.cells.get_mut(&i).unwrap()
  }
}
