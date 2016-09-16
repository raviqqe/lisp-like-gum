use processor::Processor;
use reference::Ref;
use result::Result;
use thunk::Thunk;



pub type Function = Fn (Processor, Ref) -> Result<Thunk>;
