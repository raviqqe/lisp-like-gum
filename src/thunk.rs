use address::GlobalAddress;
use object::Object;
use reference::Ref;



pub type Waits = u64;

type BlackHole = Vec<GlobalAddress>;

#[derive(Debug)]
pub enum Thunk {
  Object(Box<Object>),
  App {
    app: App,
    black_hole: BlackHole,
    waits: Waits,
  },
}

impl Thunk {
  pub fn update(&mut self, v: ThunkValue) -> Vec<GlobalAddress> {
    assert!(!self.is_object());

    match v {
      ThunkValue::Object(o) => self.update_with_object(o),
      ThunkValue::App(a) => {
        self.update_with_app(a);
        Vec::new()
      }
    }
  }

  fn update_with_object(&mut self, o: Box<Object>) -> Vec<GlobalAddress> {
    let gas = self.take_from_black_hole();

    *self = Thunk::Object(o);

    gas
  }

  fn update_with_app(&mut self, a: App) {
    match *self {
      Thunk::App { ref mut app, waits, .. } => {
        assert_eq!(waits, 0);
        *app = a;
      },
      _ => unreachable!(),
    }
  }

  pub fn is_object(&self) -> bool {
    match *self {
      Thunk::Object(_) => true,
      Thunk::App { .. } => false,
    }
  }

  pub fn set_waits(&mut self, w: Waits) {
    match *self {
      Thunk::App { ref mut waits, .. } => *waits = w,
      _ => unreachable!(),
    }
  }

  pub fn decre_waits(&mut self) {
    match *self {
      Thunk::App { ref mut waits, .. } => *waits -= 1,
      _ => unreachable!(),
    }
  }

  pub fn is_ready(&self) -> bool {
    match *self {
      Thunk::App { waits, .. } => waits == 0,
      _ => unreachable!(),
    }
  }

  pub fn put_into_black_hole(&mut self, a: GlobalAddress) {
    match *self {
      Thunk::App { ref mut waits, .. } => *waits -= 1,
      _ => unreachable!(),
    }
  }

  fn take_from_black_hole(&mut self) -> Vec<GlobalAddress> {
    match *self {
      Thunk::App { ref mut black_hole, .. } => {
        black_hole.split_off(black_hole.len())
      }
      _ => unreachable!(),
    }
  }
}

impl From<Box<Object>> for Thunk {
  fn from(o: Box<Object>) -> Self {
    Thunk::Object(o)
  }
}

pub struct App {
  func: Ref,
  arg: Ref,
}

pub enum ThunkValue {
  Object(Box<Object>),
  App(App),
}
