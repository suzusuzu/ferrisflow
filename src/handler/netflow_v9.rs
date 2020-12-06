use super::super::util::bytes_to_usize;

use super::super::template_cache::{Field, TemplateCache, TemplateCacheKey, TemplateCacheValue};

use super::super::option_cache::{FlowDatas, OptionCache, OptionCacheKey};

use anyhow::{anyhow, Result};
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use chrono::Utc;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;
use std::sync::RwLock;
use std::{fmt::Display, io::Cursor};

use super::super::flowmessage::FlowMessage;
use super::super::flowmessage::FlowMessageBuilder;
use super::Handler;

#[derive(Debug, Clone)]
pub struct NetflowV9Handler {
    pub template_cache: Arc<RwLock<TemplateCache>>,
    pub option_cache: Arc<RwLock<OptionCache>>,
}

impl NetflowV9Handler {
    pub fn new() -> NetflowV9Handler {
        NetflowV9Handler {
            template_cache: Arc::new(RwLock::new(TemplateCache::new())),
            option_cache: Arc::new(RwLock::new(OptionCache::new())),
        }
    }
}

impl Display for NetflowV9Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NetflowV9Handler")
    }
}

impl Handler for NetflowV9Handler {
    fn box_clone(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }

    fn handle(
        &self,
        buf: &Vec<u8>,
        size: usize,
        exporter_addr: SocketAddr,
    ) -> Result<Vec<FlowMessage>> {
        let mut rdr = Cursor::new(buf.as_slice());
        let datetime = Utc::now();
        let version = rdr.read_u16::<BigEndian>()?;
        if version != 9 {
            return Err(anyhow!(
                "NetflowV9Handler does not support version {}",
                version
            ));
        }
        let _count = rdr.read_u16::<BigEndian>()?;
        let sys_up_time = rdr.read_u32::<BigEndian>()?;
        let unix_secs = rdr.read_u32::<BigEndian>()?;
        let seq_number = rdr.read_u32::<BigEndian>()?;
        let source_id = rdr.read_u32::<BigEndian>()?;

        let mut flowmessages = Vec::new();

        while (size as u64 - rdr.position()) > 0 {
            let flowset_id = rdr.read_u16::<BigEndian>()?;
            let length = rdr.read_u16::<BigEndian>()?;
            if flowset_id == 0 {
                let mut buf_data = vec![0u8; length as usize - 4];
                rdr.read_exact(&mut buf_data)?;
                let mut rdr_data = Cursor::new(buf_data.as_slice());

                let rdr_data_len = buf_data.len() as u64;
                while (rdr_data_len - rdr_data.position()) >= 4 {
                    let template_id = rdr_data.read_u16::<BigEndian>()?;
                    let field_count = rdr_data.read_u16::<BigEndian>()?;
                    let mut fields = Vec::with_capacity(field_count as usize);
                    for _ in 0..field_count {
                        let field_type = rdr_data.read_u16::<BigEndian>()?;
                        let field_length = rdr_data.read_u16::<BigEndian>()?;
                        if field_length == 0 {
                            return Err(anyhow!("field length 0 error"));
                        }
                        fields.push(Field::new(field_type, field_length))
                    }
                    let k = TemplateCacheKey::new(
                        exporter_addr.ip().to_string(),
                        source_id,
                        template_id,
                        version,
                    );
                    let v = TemplateCacheValue::new(fields, Vec::new(), false);
                    let mut template_cache = self.template_cache.write().unwrap();
                    template_cache.insert(k, v);
                }
            } else if flowset_id == 1 {
                let mut buf_data = vec![0u8; length as usize - 4];
                rdr.read_exact(&mut buf_data)?;
                let mut rdr_data = Cursor::new(buf_data.as_slice());

                let rdr_data_len = buf_data.len() as u64;
                while (rdr_data_len - rdr_data.position()) >= 4 {
                    let template_id = rdr_data.read_u16::<BigEndian>()?;
                    let option_scope_length = rdr_data.read_u16::<BigEndian>()?;
                    let option_length = rdr_data.read_u16::<BigEndian>()?;

                    let option_scope_cnt = (option_scope_length / 4) as usize;
                    let option_cnt = (option_length / 4) as usize;

                    let mut scope_fields = Vec::with_capacity(option_scope_cnt);
                    let mut fields = Vec::with_capacity(option_cnt);

                    for _ in 0..option_scope_cnt {
                        let field_type = rdr_data.read_u16::<BigEndian>()?;
                        let field_length = rdr_data.read_u16::<BigEndian>()?;
                        if field_length == 0 {
                            return Err(anyhow!("field length 0 error"));
                        }
                        scope_fields.push(Field::new(field_type, field_length))
                    }

                    for _ in 0..option_cnt {
                        let field_type = rdr_data.read_u16::<BigEndian>()?;
                        let field_length = rdr_data.read_u16::<BigEndian>()?;
                        if field_length == 0 {
                            return Err(anyhow!("field length 0 error"));
                        }
                        fields.push(Field::new(field_type, field_length))
                    }

                    let k = TemplateCacheKey::new(
                        exporter_addr.ip().to_string(),
                        source_id,
                        template_id,
                        version,
                    );
                    let v = TemplateCacheValue::new(fields, scope_fields, true);
                    let mut template_cache = self.template_cache.write().unwrap();
                    template_cache.insert(k, v);
                }
            } else if flowset_id >= 256 {
                let k = TemplateCacheKey::new(
                    exporter_addr.ip().to_string(),
                    source_id,
                    flowset_id,
                    version,
                );
                let template_cache = self.template_cache.read().unwrap();
                if !template_cache.contains_key(&k) {
                    return Err(anyhow!("not found template template_id = {}", flowset_id));
                }
                let v = template_cache.get(&k).unwrap();

                let field_length_sum = v.fields.iter().map(|x| x.length as u64).sum::<u64>();

                let mut buf_data = vec![0; length as usize - 4];
                rdr.read_exact(&mut buf_data)?;
                let buf_data_len = buf_data.len() as u64;
                let mut rdr_data = Cursor::new(buf_data.as_slice());

                while (buf_data_len - rdr_data.position()) >= field_length_sum {
                    let mut scope_datas = FlowDatas::new();
                    for o in v.scope_fields.iter() {
                        let mut buf_data = vec![0u8; o.length as usize];
                        rdr_data.read_exact(&mut buf_data)?;
                        scope_datas.insert(o.type_, buf_data);
                    }

                    let mut datas = FlowDatas::new();
                    for o in v.fields.iter() {
                        let mut buf_data = vec![0u8; o.length as usize];
                        rdr_data.read_exact(&mut buf_data)?;
                        datas.insert(o.type_, buf_data);
                    }

                    if v.is_option {
                        let mut option_cache = self.option_cache.write().unwrap();
                        let k = OptionCacheKey::new(exporter_addr.ip().to_string());
                        option_cache.insert(k, datas);
                    } else {
                        let mut builder = FlowMessageBuilder::default();
                        builder
                            .datetime(datetime.to_string())
                            .exporter_addr(exporter_addr)
                            .version(version)
                            .sys_up_time(sys_up_time)
                            .flow_sequence(seq_number)
                            .unix_secs(unix_secs);
                        builder = add_builder(builder, &datas);

                        let option_cache = self.option_cache.read().unwrap();
                        let k = OptionCacheKey::new(exporter_addr.ip().to_string());
                        if option_cache.contains_key(&k) {
                            let option_datas = option_cache.get(&k).unwrap();
                            builder = add_builder(builder, option_datas);
                        }

                        flowmessages.push(builder.build().unwrap());
                    }
                }
            }
        }
        Ok(flowmessages)
    }
}

fn add_builder(mut builder: FlowMessageBuilder, datas: &FlowDatas) -> FlowMessageBuilder {
    for (type_, data) in datas {
        match type_ {
            1u16 => {
                if let Ok(in_bytes) = bytes_to_usize(&data) {
                    builder.in_bytes(in_bytes);
                }
            }
            2u16 => {
                if let Ok(in_pkts) = bytes_to_usize(&data) {
                    builder.in_pkts(in_pkts);
                }
            }
            3u16 => {
                if let Ok(flows) = bytes_to_usize(&data) {
                    builder.flows(flows);
                }
            }
            4u16 => {
                builder.protocol(data[0]);
            }
            5u16 => {
                builder.tos(data[0]);
            }
            6u16 => {
                builder.tcp_flags(data[0]);
            }
            7u16 => {
                builder.src_port(BigEndian::read_u16(data.as_slice()));
            }
            8u16 => {
                builder.ipv4_src_addr(Ipv4Addr::from(BigEndian::read_u32(data.as_slice())));
            }
            9u16 => {
                builder.src_mask(data[0]);
            }
            10u16 => {
                if let Ok(input_snmp) = bytes_to_usize(&data) {
                    builder.input_snmp(input_snmp);
                }
            }
            11u16 => {
                builder.dst_port(BigEndian::read_u16(data.as_slice()));
            }
            12u16 => {
                builder.ipv4_dst_addr(Ipv4Addr::from(BigEndian::read_u32(data.as_slice())));
            }
            13u16 => {
                builder.dst_mask(data[0]);
            }
            14u16 => {
                if let Ok(output_snmp) = bytes_to_usize(&data) {
                    builder.output_snmp(output_snmp);
                }
            }
            15u16 => {
                builder.ipv4_next_hop(Ipv4Addr::from(BigEndian::read_u32(data.as_slice())));
            }
            16u16 => {
                if data.len() == 2 {
                    builder.src_as(BigEndian::read_u16(data.as_slice()) as u32);
                } else if data.len() == 4 {
                    builder.src_as(BigEndian::read_u32(data.as_slice()));
                }
            }
            17u16 => {
                if data.len() == 2 {
                    builder.dst_as(BigEndian::read_u16(data.as_slice()) as u32);
                } else if data.len() == 4 {
                    builder.dst_as(BigEndian::read_u32(data.as_slice()));
                }
            }
            18u16 => {
                builder.bgp_ipv4_next_hop(Ipv4Addr::from(BigEndian::read_u32(data.as_slice())));
            }
            19u16 => {
                if let Ok(mul_dst_pkts) = bytes_to_usize(&data) {
                    builder.mul_dst_pkts(mul_dst_pkts);
                }
            }
            20u16 => {
                if let Ok(mul_dst_bytes) = bytes_to_usize(&data) {
                    builder.mul_dst_bytes(mul_dst_bytes);
                }
            }
            21u16 => {
                builder.last_switched(BigEndian::read_u32(data.as_slice()));
            }
            22u16 => {
                builder.first_switched(BigEndian::read_u32(data.as_slice()));
            }
            23u16 => {
                if let Ok(out_bytes) = bytes_to_usize(&data) {
                    builder.out_bytes(out_bytes);
                }
            }
            24u16 => {
                if let Ok(out_pkts) = bytes_to_usize(&data) {
                    builder.out_pkts(out_pkts);
                }
            }
            27u16 => {
                builder.ipv6_src_addr(Ipv6Addr::from(BigEndian::read_u128(data.as_slice())));
            }
            28u16 => {
                builder.ipv6_dst_addr(Ipv6Addr::from(BigEndian::read_u128(data.as_slice())));
            }
            29u16 => {
                builder.ipv6_src_mask(data[0]);
            }
            30u16 => {
                builder.ipv6_dst_mask(data[0]);
            }
            31u16 => {
                if let Ok(ipv6_flow_label) = bytes_to_usize(&data) {
                    builder.ipv6_flow_label(ipv6_flow_label);
                }
            }
            32u16 => {
                builder.icmp_type(BigEndian::read_u16(data.as_slice()));
            }
            33u16 => {
                builder.mul_igmp_type(data[0]);
            }
            34u16 => {
                builder.sampling_interval(BigEndian::read_u32(data.as_slice()));
            }
            35u16 => {
                builder.sampling_algorithm(data[0]);
            }
            36u16 => {
                builder.flow_active_timeout(BigEndian::read_u16(data.as_slice()));
            }
            37u16 => {
                builder.flow_inactive_timeout(BigEndian::read_u16(data.as_slice()));
            }
            38u16 => {
                builder.engine_type(data[0]);
            }
            39u16 => {
                builder.engine_id(data[0]);
            }
            40u16 => {
                builder.engine_id(data[0]);
            }
            41u16 => {
                if let Ok(total_bytes_exp) = bytes_to_usize(&data) {
                    builder.total_bytes_exp(total_bytes_exp);
                }
            }
            42u16 => {
                if let Ok(total_pkts_exp) = bytes_to_usize(&data) {
                    builder.total_pkts_exp(total_pkts_exp);
                }
            }
            46u16 => {
                builder.mpls_top_label(data[0]);
            }
            47u16 => {
                builder.mpls_top_label_ip_addr(BigEndian::read_u32(data.as_slice()));
            }
            48u16 => {
                builder.flow_sampler_id(data[0]);
            }
            49u16 => {
                builder.flow_sampler_mode(data[0]);
            }
            50u16 => {
                builder.flow_sampler_random_interval(BigEndian::read_u32(data.as_slice()));
            }
            55u16 => {
                builder.dst_tos(data[0]);
            }
            56u16 => {
                builder.src_mac(BigEndian::read_u48(data.as_slice()));
            }
            57u16 => {
                builder.dst_mac(BigEndian::read_u48(data.as_slice()));
            }
            58u16 => {
                builder.src_vlan(BigEndian::read_u16(data.as_slice()));
            }
            59u16 => {
                builder.dst_vlan(BigEndian::read_u16(data.as_slice()));
            }
            60u16 => {
                builder.ip_protocol_version(data[0]);
            }
            61u16 => {
                builder.direction(data[0]);
            }
            62u16 => {
                builder.ipv6_next_hop(Ipv6Addr::from(BigEndian::read_u128(data.as_slice())));
            }
            63u16 => {
                builder.bgp_ipv6_next_hop(Ipv6Addr::from(BigEndian::read_u128(data.as_slice())));
            }
            64u16 => {
                builder.ipv6_option_headers(BigEndian::read_u32(data.as_slice()));
            }
            70u16 => {
                builder.mpls_label_1(BigEndian::read_u24(data.as_slice()));
            }
            71u16 => {
                builder.mpls_label_2(BigEndian::read_u24(data.as_slice()));
            }
            72u16 => {
                builder.mpls_label_3(BigEndian::read_u24(data.as_slice()));
            }
            73u16 => {
                builder.mpls_label_4(BigEndian::read_u24(data.as_slice()));
            }
            74u16 => {
                builder.mpls_label_5(BigEndian::read_u24(data.as_slice()));
            }
            75u16 => {
                builder.mpls_label_6(BigEndian::read_u24(data.as_slice()));
            }
            76u16 => {
                builder.mpls_label_7(BigEndian::read_u24(data.as_slice()));
            }
            77u16 => {
                builder.mpls_label_8(BigEndian::read_u24(data.as_slice()));
            }
            78u16 => {
                builder.mpls_label_9(BigEndian::read_u24(data.as_slice()));
            }
            79u16 => {
                builder.mpls_label_10(BigEndian::read_u24(data.as_slice()));
            }
            _ => {}
        }
    }
    builder
}
