#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate nanomsg;
extern crate num;
extern crate rand;
extern crate libc;
extern crate serde;
extern crate serde_cbor;

mod weight;
mod local_address;
mod global_address;
mod memory;

// mod object;
// mod function;
// mod functions;
// mod memory;
// mod message;
// mod network;
// mod processor;
// mod reference;
// mod stored;
// mod thunk;
// mod transceiver;
// mod weighted;
