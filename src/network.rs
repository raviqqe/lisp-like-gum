use processor::ProcessorId;
use transceiver::Transceiver;



pub fn init(my_id: ProcessorId, id_to_address: HashMap<ProcessorId, String>)
    -> Tranceiver {
  Transceiver::new(my_id)
}
