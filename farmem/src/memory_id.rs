use mpi::topology::Rank;
use rand::{Rng, Rand};



#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
         Serialize, Deserialize)]
pub struct MemoryId(u64);

impl MemoryId {
  pub fn new(i: u64) -> Self {
    MemoryId(i)
  }
}

impl From<MemoryId> for Rank {
  fn from(i: MemoryId) -> Self {
    i.0 as Rank
  }
}

impl From<Rank> for MemoryId {
  fn from(r: Rank) -> Self {
    MemoryId::new(r as u64)
  }
}

impl Rand for MemoryId {
  fn rand<R: Rng>(r: &mut R) -> Self {
    MemoryId(r.next_u64())
  }
}
