use processor::ProcessorId;
use message::Message;
use rand::random;



pub struct Transceiver {
  proc_id: ProcessorId,
}

impl Transceiver {
  fn new(id: ProcessorId) -> Self {
    Transceiver { proc_id: id }
  }

  fn send(&self, m: Message) {
    unimplemented!()
  }

  fn send_at_random(&self, m: Message) {
    // let i = random() % self.peers.len();
    unimplemented!()
  }

  fn receive(&self) -> Message {
    unimplemented!()
  }
}
