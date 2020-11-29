use super::super::flowmessage::FlowMessage;
use super::Publisher;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct JsonPublisher {}

impl JsonPublisher {
    pub fn new() -> JsonPublisher {
        JsonPublisher {}
    }
}

impl Publisher for JsonPublisher {
    fn box_clone(&self) -> Box<dyn Publisher> {
        Box::new(self.clone())
    }

    fn publish(&self, flowmessages: &Vec<FlowMessage>) -> Result<()> {
        for flowmessage in flowmessages {
            let serialized = serde_json::to_string(flowmessage)?;
            println!("{}", serialized);
        }
        Ok(())
    }
}
