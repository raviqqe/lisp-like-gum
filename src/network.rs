use processor::ProcessorId;
use transceiver::Transceiver;



pub fn init(id: ProcessorId, peers: Vec<&str>) -> Transceiver {
  Transceiver::new(id)
}
