use network;


pub type ProcessorId = u64;

pub struct Processor {
  id: ProcessorId,
  memory: Memory,
  transceiver: Transceiver,
}

impl Processor {
  fn new(id: ProcessorId) -> Processor {
    Processor { id: id, memory: Memory::new(id), transceiver: network::init() }
  }

  fn run(&mut self) {

  }
}
