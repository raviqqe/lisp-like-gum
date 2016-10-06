#![feature(plugin, custom_derive, type_ascription)]
#![plugin(serde_macros)]

#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
extern crate mpi;
extern crate rand;
extern crate serde;
extern crate serde_cbor;

mod demand;
mod global_address;
mod load_error;
mod load_result;
mod local_address;
mod memory;
mod memory_id;
mod message;
mod notice;
mod object;
mod reference;
mod type_manager;
mod serialized_object;
mod transceiver;
mod type_;
mod type_id;
mod weight;

pub use memory::Memory;
