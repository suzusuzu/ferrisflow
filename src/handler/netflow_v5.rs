use super::super::flowmessage::FlowMessage;
use super::super::flowmessage::FlowMessageBuilder;
use super::Handler;
use byteorder::{BigEndian, ReadBytesExt};
use chrono::Utc;
use std::net::Ipv4Addr;
use std::{fmt::Display, io::Cursor};

use anyhow::{anyhow, Result};
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct NetflowV5Handler {}

impl NetflowV5Handler {
    pub fn new() -> NetflowV5Handler {
        NetflowV5Handler {}
    }
}

impl Display for NetflowV5Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NetflowV5Handler")
    }
}

impl Handler for NetflowV5Handler {
    fn box_clone(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }

    fn handle(
        &self,
        buf: &Vec<u8>,
        _size: usize,
        exporter_addr: SocketAddr,
    ) -> Result<Vec<FlowMessage>> {
        let mut rdr = Cursor::new(buf.as_slice());
        let datetime = Utc::now();
        let version = rdr.read_u16::<BigEndian>()?;
        if version != 5 {
            return Err(anyhow!(
                "NetflowV5Handler does not support version {}",
                version
            ));
        }
        let count = rdr.read_u16::<BigEndian>()?;
        let sys_up_time = rdr.read_u32::<BigEndian>()?;
        let unix_secs = rdr.read_u32::<BigEndian>()?;
        let unix_nsecs = rdr.read_u32::<BigEndian>()?;
        let flow_sequence = rdr.read_u32::<BigEndian>()?;
        let engine_type = rdr.read_u8()?;
        let engine_id = rdr.read_u8()?;
        let sampling_interval = rdr.read_u16::<BigEndian>()?;

        let mut flowmessages = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let mut builder = FlowMessageBuilder::default();

            let src_addr = Ipv4Addr::from(rdr.read_u32::<BigEndian>()?);
            let dst_addr = Ipv4Addr::from(rdr.read_u32::<BigEndian>()?);
            let next_hop = Ipv4Addr::from(rdr.read_u32::<BigEndian>()?);
            let input = rdr.read_u16::<BigEndian>()?;
            let output = rdr.read_u16::<BigEndian>()?;
            let dpkts = rdr.read_u32::<BigEndian>()?;
            let d0ctets = rdr.read_u32::<BigEndian>()?;
            let first = rdr.read_u32::<BigEndian>()?;
            let last = rdr.read_u32::<BigEndian>()?;
            let src_port = rdr.read_u16::<BigEndian>()?;
            let dst_port = rdr.read_u16::<BigEndian>()?;
            let _ = rdr.read_u8()?;
            let tcp_flags = rdr.read_u8()?;
            let proto = rdr.read_u8()?;
            let tos = rdr.read_u8()?;
            let src_as = rdr.read_u16::<BigEndian>()?;
            let dst_as = rdr.read_u16::<BigEndian>()?;
            let src_mask = rdr.read_u8()?;
            let dst_mask = rdr.read_u8()?;
            let _ = rdr.read_u16::<BigEndian>()?;

            builder
                .datetime(datetime.to_string())
                .exporter_addr(exporter_addr)
                .version(version)
                .sys_up_time(sys_up_time)
                .unix_secs(unix_secs)
                .unix_nsecs(unix_nsecs)
                .flow_sequence(flow_sequence)
                .engine_type(engine_type)
                .engine_id(engine_id)
                .sampling_interval(sampling_interval as u32)
                .ipv4_src_addr(src_addr)
                .ipv4_dst_addr(dst_addr)
                .ipv4_next_hop(next_hop)
                .input(input)
                .output(output)
                .dpkts(dpkts)
                .d0ctets(d0ctets)
                .first(first)
                .last(last)
                .src_port(src_port)
                .dst_port(dst_port)
                .tcp_flags(tcp_flags)
                .protocol(proto)
                .tos(tos)
                .src_as(src_as as u32)
                .dst_as(dst_as as u32)
                .src_mask(src_mask)
                .dst_mask(dst_mask)
                .ip_protocol_version(4u8);

            flowmessages.push(builder.build().unwrap());
        }
        Ok(flowmessages)
    }
}
