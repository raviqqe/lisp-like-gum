pub enum Message {
  Fetch { from: Ref, to: Ref },
  Resume { to: Ref, from: Ref, object: Box<Object> },

  Fish { from: ProcessorId },
  Schedule { task: Thunk, thunks: Vec<Thunk> },

  Ack { to: Ref },
  Finish,
}
