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

use processor::Processor;



fn get_args() -> ArgvMap {
  let usage = "
Lisp?

Usage:
  lisp [-l <log_level>] [-s <proc_id>] <filename>
  lisp (-h | --help)

Options:
  -l, --log-level <log_level>  Set log level. Valid values are \"error\"
                               (default), \"warn\", \"info\", \"debug\",
                               \"trace\", and \"off\".
  -s, --slave <proc_id>
  -h, --help  Show help.
";

  Docopt::new(usage).and_then(|d| d.parse()).unwrap_or_else(|e| e.exit())
}

fn main() {
  let args = get_args();

  log::init(args.get_str("--log-level"));

  // read_config_file

  let p = Processor::new(0 /* args.get_bool("--slave").into() */,
                         vec!["tcp://127.0.0.1:1996"]);

  if p.id == 0 {
    p.run_as_master();
  } else {

  }
}

fn read_file(fname: &str) -> String {
  let mut s = String:new();

  File::open(fname).unwrap().read_to_string(&mut s).unwrap();

  s
}
