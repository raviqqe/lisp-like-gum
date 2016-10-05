use serde::ser::Serialize;
use serde::de::Deserialize;

use reference::Ref;



pub trait Object: Serialize + Deserialize {
  fn refs(&self) -> Vec<Ref>;
  fn nearby_refs(&self) -> Vec<Ref>;
}
