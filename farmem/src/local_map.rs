use std::collections::BTreeMap;
use std::ops::Index;

use local_id::LocalId;
use local_id_manager::LocalIdManager;
use local_address::LocalAddress;



#[derive(Debug, Default)]
pub struct LocalMap {
  id_to_address: BTreeMap<LocalId, LocalAddress>,
  address_to_id: BTreeMap<LocalAddress, LocalId>,
  id_manager: LocalIdManager,
}

impl LocalMap {
  pub fn new() -> Self {
    LocalMap::default()
  }

  pub fn map(&mut self, a: LocalAddress) -> LocalId {
    let i = self.id_manager.new_id();
    self.id_to_address.insert(i, a);
    self.address_to_id.insert(a, i);
    i
  }

  pub fn unmap(&mut self, i: LocalId) {
    let a = self.id_to_address.remove(&i).unwrap();
    let _ = self.address_to_id.remove(&a);
    self.id_manager.return_id(i);
  }
}

impl Index<LocalId> for LocalMap {
  type Output = LocalAddress;

  fn index(&self, i: LocalId) -> &Self::Output {
    self.id_to_address.index(&i)
  }
}

impl Index<LocalAddress> for LocalMap {
  type Output = LocalId;

  fn index(&self, a: LocalAddress) -> &Self::Output {
    self.address_to_id.index(&a)
  }
}
