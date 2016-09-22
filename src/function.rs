use processor::ProcessorId;
use reference::Ref;
use memory::ThunkMemory;
use message::Message;
use thunk::ThunkValue;



pub type Func<T> where T: ThunkMemory = Fn (&T, Ref) -> FuncResult;
pub type FuncResult = Result<ThunkValue, Vec<(ProcessorId, Message)>>;
