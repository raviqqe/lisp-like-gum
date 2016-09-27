use processor::ProcessorId;
use message::Message;
use rand::random;



pub struct Transceiver<R: Receiver, S: Sender> {
  receiver: R,
  senders: Vec<S>,
}

impl<R: Receiver, S: Sender> Transceiver<R, S> {
  pub fn new(r: R, ss: Vec<S>) -> Self {
    Transceiver { receiver: r, senders: ss }
  }

  pub fn send(&self, i: ProcessorId, m: Message) {
    self.senders[i as usize].send(m.into())
  }

  pub fn send_at_random(&self, m: Message) {
    self.send(random::<ProcessorId>() % self.senders.len() as ProcessorId, m)
  }

  pub fn receive(&self) -> Message {
    self.receiver.receive().into()
  }

  pub fn can_receive(&self) -> bool {
    self.receiver.can_receive()
  }
}

pub trait Sender {
  fn send(&self, data: Vec<u8>);
}

pub trait Receiver {
  fn receive(&self) -> Vec<u8>;
  fn can_receive(&self) -> bool;
}
