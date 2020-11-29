use super::super::flowmessage::FlowMessage;
use super::Publisher;
use anyhow::Result;
use csv::{Writer, WriterBuilder};
use std::io::stdout;

#[derive(Debug, Clone)]
pub struct CsvPublisher {}

impl CsvPublisher {
    pub fn new(header_none: bool) -> CsvPublisher {
        if !header_none {
            let fields = FlowMessage::as_field_name_array();
            println!(
                "{}",
                fields
                    .iter()
                    .map(|x| x.name().to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            );
        }
        CsvPublisher {}
    }
}

impl Publisher for CsvPublisher {
    fn box_clone(&self) -> Box<dyn Publisher> {
        let mut wtr = Writer::from_writer(stdout());
        wtr.flush().unwrap();
        Box::new(self.clone())
    }

    fn publish(&self, flowmessages: &Vec<FlowMessage>) -> Result<()> {
        let mut wtr = WriterBuilder::new()
            .has_headers(false)
            .from_writer(stdout());
        for flowmessage in flowmessages {
            wtr.serialize(flowmessage)?;
        }
        wtr.flush()?;
        Ok(())
    }
}