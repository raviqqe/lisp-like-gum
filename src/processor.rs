use network;


pub type ProcessorId = u64;

pub struct Processor {
  id: ProcessorId,
  memory: Memory,
  transceiver: Transceiver,
}

impl Processor {
  fn new(id: ProcessorId, ps: HashMap<Processor, String>) -> Processor {
    Processor {
      id: id,
      memory: Memory::new(id),
      transceiver: network::init(id, ps),
    }
  }

  fn run(&mut self) {

  }
}
