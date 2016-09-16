pub struct Memory {
  id: ProcessorId,
  weights: BTreeMap<LocalAddress, Weight>
}

impl Memory {
  fn new(id: ProcessorId) -> Self {
    Memory { id: id, weights: BTreeMap::new() }
  }

  fn store(&mut self, t: Thunk) -> Ref {

  }

  fn load<'a>(&self, r: Ref) -> &'a Thunk {

  }

  fn incre_weight(&mut self, w: Weight) {

  }

  fn decre_weight(&mut self, w: Weight) {

  }
}
