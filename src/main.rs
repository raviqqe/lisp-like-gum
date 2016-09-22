extern crate docopt;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log as metalog;
extern crate num;
extern crate rand;

mod address;
mod function;
mod functions;
mod log;
mod memory;
mod message;
mod network;
mod object;
mod processor;
mod reference;
mod thunk;
mod transceiver;
mod weight;
mod weighted;

use std::fs::File;
use std::str::FromStr;

use doctop::{ArgvMap, Docopt};

use processor::Processor;



fn get_args() -> ArgvMap {
  let usage = "
Lisp?

Usage:
  lisp [-l <log_level>] [-p <proc_id>] <filename>
  lisp (-h | --help)

Options:
  -l, --log-level <log_level>  Set log level. Valid values are \"error\"
                               (default), \"warn\", \"info\", \"debug\",
                               \"trace\", and \"off\".
  -p, --proc-id <proc_id>
  -h, --help  Show help.
";

  Docopt::new(usage).and_then(|d| d.parse()).unwrap_or_else(|e| e.exit())
}

fn main() {
  let args = get_args();

  log::init(args.get_str("--log-level"));

  // read_config_file

  let mut p = Processor::new(parse_proc_id(args.get_str("--proc-id")),
                             vec!["tcp://127.0.0.1:1996"]);

  if p.id == 0 {
    p.run_as_master(&read_file(args.get_str("<filename>")));
  } else {
    p.run_as_slave();
  }
}

fn read_file(f: &str) -> String {
  let mut s = String::new();

  File::open(f).unwrap().read_to_string(&mut s).unwrap();

  s
}

fn parse_proc_id(s: &str) -> u64 {
  if s == "" {
    0
  } else {
    u64::from_str(s).unwrap()
  }
}
