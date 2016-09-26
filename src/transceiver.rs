use processor::ProcessorId;
use message::Message;
use rand::random;



pub struct Transceiver {
  proc_id: ProcessorId,
}

impl Transceiver {
  pub fn new(id: ProcessorId) -> Self {
    Transceiver { proc_id: id }
  }

  pub fn send(&self, m: Message) {
    unimplemented!()
  }

  pub fn send_at_random(&self, m: Message) {
    // let i = random() % self.peers.len();
    unimplemented!()
  }

  pub fn receive(&self) -> Message {
    unimplemented!()
  }

  pub fn can_receive(&self) -> bool {
    unimplemented!()
  }
}
