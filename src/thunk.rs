use object::Object;
use reference::Ref;
use weight::Weight;



pub type Waits = u64;

#[derive(Debug)]
pub enum Thunk {
  Object(Box<Object>),
  App {
    app: App,
    black_hole: Vec<GlobalAddress>,
    waits: Waits,
  },
}

impl Thunk {
  fn from_object(o: Box<Object>) -> (Self, ) {

  }

  fn update(&mut self, v: ThunkValue) {
    match v {
      ThunkValue::Object(o) => {
        self.value = ThunkEnum::Object(o);
      }
      ThunkValue::App(a) => {
        self.value = ThunkEnum::App {
          app: a,
          black_hole: BlackHole::new(),
          waits: 0,
        };
      },
    }
  }

  fn is_object(&self) -> bool {
    match self.value {
      ThunkEnum::Object(_) => true,
      ThunkEnum::App { .. } => false,
    }
  }

  fn add_weight(&mut self, w: Weight) {
    self.weight += w;
  }

  fn sub_weight(&mut self, w: Weight) -> bool {
    self.weight += w;
  }
}

type BlackHole = Vec<GlobalAddress>;

pub struct App {
  func: Ref,
  arg: Ref,
}

pub enum ThunkValue {
  Object(Box<Object>),
  App(App),
}
