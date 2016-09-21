#[macro_use]
extern crate log as metalog;
extern crate rand;

// mod address;
// mod function;
mod log;
// mod memory;
// mod message;
// mod object;
mod processor;
// mod reference;
// mod result;
// mod thunk;
// mod transceiver;
// mod weight;

use processor::Processor;



fn main() {
  log::init("debug");

  Processor::new().run();
}
