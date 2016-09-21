use std::collections::VecDeque;

use config::PROC_ID;
use config::ProcessorId;
use functions::{eval, expand_macros, read};
use memory::Memory;
use message::Message::*;
use network;
use rand::random;
use reference::Ref;



pub struct Processor {
  id: ProcessorId,
  memory: Memory,
  tasks: VecDeque<Ref>,
  transceiver: Transceiver,
  should_stop: bool,
}

impl Processor {
  pub fn new() -> Processor {
    Processor {
      tasks: VecDeque::new(),
      transceiver: network::init(),
      should_stop: false,
    }
  }

  pub fn run_as_master(&mut self, source_code: &str) {
    self.tasks.extend(eval(expand_macros(read(source_code.into()))));
    self.run_loop()
  }

  pub fn run_as_slave(&mut self) {
    self.run_loop()
  }

  fn proc_id(&self) -> ProcessorId {
    *PROC_ID
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
        Fetch { from, address } => {
          from.GlobalAddress
        }
        Resume { to, address, object } => {
          to.decre_waits();

          address;

          if to.is_ready() {
            self.tasks.push_back(to.into());
          }
        }

        Fish { from } => {
        }
        Schedule { task, neighbors } => {
          self.tasks.push_back(t.into());
        }
        Finish => {
          self.should_stop = true;
        }
      }
    }
  }

  fn look_for_tasks(&self) {
    self.transceiver.send(random() % self.transceiver.num_receivers(),
                          Fish { from: self.proc_id() });
  }

  fn run_a_task(&mut self) {
    let t = self.tasks.pop_front();
    unimplemented!();
  }

  fn delete_ref(&mut self, r: Ref) {
    assert_eq!(r.global_address.proc_id, self.proc_id);
    let a = r.local_address();
    a.weight -= r.weight;

    if a.weight == 0 {
      let _ = unsafe { Box::from_raw(a) };
    }
  }
}
