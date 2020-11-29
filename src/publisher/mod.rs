use super::flowmessage::FlowMessage;
use anyhow::Result;

pub mod print;
pub use print::PrintPublisher;

pub mod json;
pub use json::JsonPublisher;

pub mod csv;
pub use self::csv::CsvPublisher;

pub trait Publisher: Send {
    fn box_clone(&self) -> Box<dyn Publisher>;
    fn publish(&self, flowmessages: &Vec<FlowMessage>) -> Result<()>;
}

impl Clone for Box<dyn Publisher> {
    fn clone(&self) -> Box<dyn Publisher> {
        self.box_clone()
    }
}
