use std::collections::VecDeque;

use memory::Memory;
use message::Message::*;
use network;



pub type ProcessorId = u64;

pub struct Processor {
  id: ProcessorId,
  tasks: VecDeque<Ref>,
  memory: Memory,
  transceiver: Transceiver,
  should_stop: bool,
}

impl Processor {
  pub fn new(id: ProcessorId, ps: HashMap<Processor, String>) -> Processor {
    Processor {
      id: id,
      tasks: VecDeque::new(),
      memory: Memory::new(id),
      transceiver: network::init(id, ps),
      should_stop: false,
    }
  }

  pub fn run_as_master(&mut self, source_code: &str) {
    let (os, ts) = eval(expand_macros(read(source_code.into())));

    for t in os.clone().extend(ts) {
      self.memory.store(t)
    }

    self.tasks.extend(os);

    self.run_loop()
  }

  pub fn run_as_slave(&mut self) {
    self.run_loop()
  }

  fn run_loop(&mut self) {
    while !self.should_stop {
      self.process_messages();

      if self.tasks.is_empty() {
        self.look_for_tasks()
      } else {
        self.run_a_task()
      }
    }
  }

  fn process_messages(&mut self) {
    while self.transceiver.can_receive() {
      match self.transceiver.receive() {
        Fish { from } => {
        }
        Finish => {
          self.should_stop = true;
        }
        _ => println!("GOOD"),
      }
    }
  }

  fn look_for_tasks(&self) {
    let n = self.transceiver.num_receivers();
    self.transceiver.send(Fish { from: self.id })
  }

  fn run_a_task(&mut self) {
    let t = self.tasks.pop_front();
  }
}
