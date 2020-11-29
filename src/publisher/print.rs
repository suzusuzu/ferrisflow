use super::super::flowmessage::FlowMessage;
use super::Publisher;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct PrintPublisher {}

impl PrintPublisher {
    pub fn new() -> PrintPublisher {
        PrintPublisher {}
    }
}

impl Publisher for PrintPublisher {
    fn box_clone(&self) -> Box<dyn Publisher> {
        Box::new(self.clone())
    }

    fn publish(&self, flowmessages: &Vec<FlowMessage>) -> Result<()> {
        println!("{:?}", flowmessages);
        Ok(())
    }
}
