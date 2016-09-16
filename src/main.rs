#[macro_use]
extern crate log as metalog;

mod packet;
mod processor;
mod thunk;
mod log;

use processor::Processor;


fn main() {
  log::init("debug");
  Processor::new().run();
}
