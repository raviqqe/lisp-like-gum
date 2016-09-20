use std::result;



pub type Result<T> = result::Result<T, Vec<(ProcessorId, Message)>>;
