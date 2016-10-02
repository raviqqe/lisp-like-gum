use serde::ser::Serialize;
use serde::de::Deserialize;



pub trait Object: Serialize + Deserialize {}
