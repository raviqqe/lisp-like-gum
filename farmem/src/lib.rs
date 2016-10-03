#![feature(plugin, custom_derive, type_ascription)]
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

mod global_address;
mod local_address;
mod memory;
mod object;
mod reference;
mod weight;
mod cell;

// mod message;
