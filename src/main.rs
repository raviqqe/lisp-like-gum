#[macro_use]
extern crate log as metalog;

mod packet;
mod processor;
mod thunk;
mod log;



fn main() {
  log::init("debug");
}
