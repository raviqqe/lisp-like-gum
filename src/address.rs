use processor::ProcessorId;



pub type LocalAddress = u64;

pub struct GlobalAddress {
  proc_id: ProcessorId,
  local_address: LocalAddress,
}

impl GlobalAddress {
  fn new(id: ProcessorId, a: LocalAddress) -> Self {
    GlobalAddress { proc_id: id, local_address: a }
  }
}
