use processor::ProcessorId;
use transceiver;
use nanomsg;
use nanomsg::{Socket, Protocol};



pub type Transceiver = transceiver::Transceiver<Receiver, Sender>;

pub type Address<'a> = &'a str;


pub fn init(id: ProcessorId, as_: Vec<Address>) -> Transceiver {
  let r = Receiver::new(as_[id as usize]);
  let mut ss = Vec::new();

  for a in as_ {
    ss.push(Sender::new(a))
  }

  Transceiver::new(r, ss)
}


pub struct Receiver {
  socket: Socket,
}

impl Receiver {
  fn new(a: Address) -> Self {
    let mut s = Socket::new(Protocol::Pull).unwrap();
    let _ = s.connect(a).unwrap();

    Receiver { socket: s }
  }
}

impl transceiver::Receiver for Receiver {
  fn receive(&mut self) -> Option<Vec<u8>> {
    let mut v = Vec::new();
    let n = match self.socket.nb_read_to_end(&mut v) {
      Ok(n) => n,
      Err(nanomsg::Error::TryAgain) => return None,
      Err(e) => panic!("{}", e),
    };

    assert_eq!(n, v.len());

    Some(v)
  }
}


pub struct Sender {
  socket: Socket,
}

impl Sender {
  fn new(a: Address) -> Self {
    let mut s = Socket::new(Protocol::Push).unwrap();
    let _ = s.bind(a).unwrap();

    Sender {
      socket: s,
    }
  }
}

impl transceiver::Sender for Sender {
  fn send(&mut self, data: Vec<u8>) {
    unimplemented!()
  }
}
