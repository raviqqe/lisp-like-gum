use std::any::TypeId;
use std::mem::size_of;



lazy_static!{
  pub static ref TYPE_ID_SIZE: usize = ((size_of::<TypeId>() + 7) % 8) * 8;
}
