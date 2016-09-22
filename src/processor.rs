use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

use functions::{eval, expand_macros, read};
use memory::Memory;
use message::Message::*;
use network;
use reference::Ref;
use transceiver::Transceiver;



pub type ProcessorId = u64;

pub struct Processor {
  pub id: ProcessorId,
  memory: Memory,
  tasks: VecDeque<Ref>,
  transceiver: Transceiver,
  should_stop: bool,
}

impl Processor {
  pub fn new(id: ProcessorId, ps: Vec<&str>) -> Processor {
    Processor {
      id: id,
      memory: Memory::new(id),
      tasks: VecDeque::new(),
      transceiver: network::init(id, ps),
      should_stop: false,
    }
  }

  pub fn run_as_master(&mut self, source_code: &str) {
    let m = self.memory;
    self.tasks.extend(eval(m, expand_macros(m, read(m, source_code.into()))));
    self.run_loop()
  }

  pub fn run_as_slave(&mut self) {
    self.run_loop()
  }

  fn run_loop(&mut self) {
    while !self.should_stop {
      self.process_messages();

      if self.tasks.is_empty() {
        self.look_for_tasks();
        sleep(Duration::new(0, 1));
      } else {
        self.run_a_task();
      }
    }
  }

  fn process_messages(&mut self) {
    while self.transceiver.can_receive() {
      match self.transceiver.receive() {
        Fetch { from, address } => {
          if let Thunk::Object(ref o) = *address {
            self.transceiver.send(Resume {
              to: from.local_address,
              address: GlobalAddress::new(self.id, local_address),
              object: o.clone(),
            });
          } else {
            address.put_into_black_hole(from);
          }
        }
        Resume { to, address, object } => {
          self.memory.store_global(address, object.into());
          to.decre_waits();

          if to.is_ready() {
            self.tasks.push_back(to.into());
          }
        }

        Fish { from } => {
          if self.tasks.is_empty() {

          }

          sleep(Duration::new(0, 1));
        }
        Schedule { task, neighbors } => {
          for (a, t) in neightbors {
            self.memory.store_global(a, t);
          }

          self.tasks.push_back(self.memory.store(task));
        }

        AddWeight { address, delta } => address.add_weight(delta),
        SubWeight { address, delta } => address.sub_weight(delta),

        Finish => self.should_stop = true
      }
    }
  }

  fn look_for_tasks(&self) {
    self.transceiver.send_at_random(Fish { from: self.proc_id() });
  }

  fn run_a_task(&mut self) {
    let r = self.tasks.pop_front();
    unimplemented!();
  }

  pub fn add_weight(&mut self, a: LocalAddress, dw: Weight) {
    a.add_weight(dw);
  }

  pub fn sub_weight(&mut self, a: LocalAddress, dw: Weight) {
    a.sub_weight(dw);

    if a.is_orphan() {
      let _ = unsafe { Box::from_raw(a) };
    }
  }
}
