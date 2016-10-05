use demand::Demand;
use reference::Ref;



#[derive(Debug)]
pub enum Notice {
  Demand(Demand),
  Feed(Ref),
}
