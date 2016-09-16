pub enum Thunk {
  Value(Box<Object>),
  App { func: Ref, arg: Ref, black_hole: Vec<Ref> },
}
