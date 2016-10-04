#![feature(plugin, custom_derive, type_ascription)]
#![plugin(serde_macros)]

#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
extern crate mpi;
extern crate nanomsg;
extern crate num;
extern crate rand;
extern crate serde;
extern crate serde_cbor;

mod global_address;
mod local_address;
mod memory;
mod message;
mod object;
mod reference;
mod serder;
mod serialized_object;
mod transceiver;
mod type_id;
mod weight;

// mod message;

pub use memory::Memory;
