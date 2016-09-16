pub struct Memory {
  id: ProcessorId,
  weights: BTreeMap<LocalAddress, Weight>
}

impl Memory {
  fn new(id: ProcessorId) -> Self {
    Memory { id: id, weights: BTreeMap::new() }
  }

  fn store(&mut self, t: Thunk) -> Ref {
    let r = Ref::new(self.id, Box::into_raw(Box::new()));

    self.weights.insert(r.local_address, r.weight);

    r
  }

  fn load<'a>(&self, r: Ref) -> Result<&'a Thunk> {
    if r.proc_id != self.id {
      return Err(vec![(r.proc_id, Fetch())])
    }

    Box::from_raw()
  }

  fn incre_weight(&mut self, w: Weight) {

  }

  fn decre_weight(&mut self, w: Weight) {

  }
}
