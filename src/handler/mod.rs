pub mod netflow_v5;
pub use netflow_v5::NetflowV5Handler;

pub mod netflow_v9;
pub use netflow_v9::NetflowV9Handler;

use super::flowmessage::FlowMessage;
use anyhow::Result;
use std::{fmt::Display, net::SocketAddr};

pub trait Handler: Send + Display {
    fn box_clone(&self) -> Box<dyn Handler>;
    fn handle(&self, buf: &Vec<u8>, size: usize, addr: SocketAddr) -> Result<Vec<FlowMessage>>;
}

impl Clone for Box<dyn Handler> {
    fn clone(&self) -> Box<dyn Handler> {
        self.box_clone()
    }
}
