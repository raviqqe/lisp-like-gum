use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

use address::{GlobalAddress, LocalAddress};
use functions::{eval, expand_macros, read};
use memory::Memory;
use memory::ThunkMemory;
use message::Message::*;
use network;
use reference::Ref;
use stored::Stored;
use thunk::Thunk;
use transceiver::Transceiver;
use weight::Weight;



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
    let m = &mut self.memory;

    let r = source_code.stored(m);
    let r = m.store(read(m, r).unwrap().into());
    let r = m.store(expand_macros(m, r).unwrap().into());
    let r = m.store(eval(m, r).unwrap().into());

    self.tasks.push_back(r);
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
          match address.object() {
            Some(o) => self.transceiver.send(Resume {
              to: from.local_address,
              address: GlobalAddress::new(self.id, address),
              object: o.into(),
            }),
            None => address.put_into_black_hole(from),
          }
        }
        Resume { to, address, object } => {
          self.memory.store_global(address, object.into());
          to.decre_waits();

          if to.is_ready() {
            self.tasks.push_back(self.memory.get_ref(to));
          }
        }

        Fish { from } => {
          if self.tasks.is_empty() {
            unimplemented!(); // throw fish to one of others
            sleep(Duration::new(0, 1));
          } else {
            unimplemented!(); // pass thunk to origin
          }
        }
        Schedule { task, neighbors } => {
          for (a, t) in neighbors {
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
    self.transceiver.send_at_random(Fish { from: self.id });
  }

  fn run_a_task(&mut self) {
    let r = self.tasks.pop_front();
    unimplemented!();
  }
}
