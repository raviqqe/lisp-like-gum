use processor::ProcessorId;
use transceiver;



pub type Transceiver = transceiver::Transceiver<Receiver, Sender>;

pub fn init(id: ProcessorId, procs: Vec<&str>) -> Transceiver {
  for _ in 0..procs.len() {
    unimplemented!()
  }

  unimplemented!()
  // Transceiver::new()
}

pub struct Sender {
}

impl transceiver::Sender for Sender {
  fn send(&self, data: Vec<u8>) {
    unimplemented!()
  }
}

pub struct Receiver {
}

impl transceiver::Receiver for Receiver {
  fn receive(&self) -> Vec<u8> {
    unimplemented!()
  }

  fn can_receive(&self) -> bool {
    unimplemented!()
  }
}
