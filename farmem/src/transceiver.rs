use mpi::topology::{Rank, SystemCommunicator};
use mpi::traits::*;
use rand::random;
use serde_cbor::de;
use serde_cbor::ser;

use memory_id::MemoryId;
use message::Message;



pub struct Transceiver {
  world: SystemCommunicator,
}

impl Transceiver {
  pub fn new(w: SystemCommunicator) -> Self {
    Transceiver { world: w }
  }

  pub fn send(&self, i: MemoryId, m: Message) {
    self.world.process_at_rank(i.into()).send(&ser::to_vec(&m).unwrap()[..])
  }

  pub fn send_at_random(&self, m: Message) {
    let s = self.world.size();
    self.send(((random::<Rank>() % s + s) % s).into(), m)
  }

  pub fn receive(&self) -> Option<Message> {
    let any = self.world.any_process();

    if any.immediate_probe().is_none() {
      return None
    }

    de::from_slice(any.receive_vec::<u8>().0.as_ref()).unwrap()
  }
}
