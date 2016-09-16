use processor::ProcessorId;



pub struct Transceiver {
  proc_id: ProcessorId,
}


impl Transceiver {
  fn new(id: ProcessorId) -> Self {
    Tranceiver { proc_id: id }
  }

  fn send(&self, m: Message) {
  }

  fn receive(&self) -> Message {
    Finish
  }
}
