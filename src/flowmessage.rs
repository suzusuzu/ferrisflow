use serde::{Deserialize, Serialize};

use derive_builder::Builder;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

use field_types::FieldName;

#[derive(Builder, Debug, Serialize, Deserialize, FieldName)]
pub struct FlowMessage {
    #[builder(setter(into, strip_option), default)]
    datetime: Option<String>,

    #[builder(setter(into, strip_option), default)]
    exporter_addr: Option<SocketAddr>,

    #[builder(setter(into, strip_option), default)]
    version: Option<u16>,

    #[builder(setter(into, strip_option), default)]
    sys_up_time: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    unix_secs: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    unix_nsecs: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    flow_sequence: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    engine_type: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    engine_id: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    sampling_interval: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    ipv4_src_addr: Option<Ipv4Addr>,

    #[builder(setter(into, strip_option), default)]
    ipv4_dst_addr: Option<Ipv4Addr>,

    #[builder(setter(into, strip_option), default)]
    ipv4_next_hop: Option<Ipv4Addr>,

    #[builder(setter(into, strip_option), default)]
    input: Option<u16>,

    #[builder(setter(into, strip_option), default)]
    output: Option<u16>,

    #[builder(setter(into, strip_option), default)]
    dpkts: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    d0ctets: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    first: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    last: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    src_port: Option<u16>,

    #[builder(setter(into, strip_option), default)]
    dst_port: Option<u16>,

    #[builder(setter(into, strip_option), default)]
    tcp_flags: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    tos: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    src_as: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    dst_as: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    src_mask: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    dst_mask: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    in_bytes: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    in_pkts: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    flows: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    protocol: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    input_snmp: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    output_snmp: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    bgp_ipv4_next_hop: Option<Ipv4Addr>,

    #[builder(setter(into, strip_option), default)]
    mul_dst_pkts: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    mul_dst_bytes: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    last_switched: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    first_switched: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    out_bytes: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    out_pkts: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    ipv6_src_addr: Option<Ipv6Addr>,

    #[builder(setter(into, strip_option), default)]
    ipv6_dst_addr: Option<Ipv6Addr>,

    #[builder(setter(into, strip_option), default)]
    ipv6_src_mask: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    ipv6_dst_mask: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    ipv6_flow_label: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    icmp_type: Option<u16>,

    #[builder(setter(into, strip_option), default)]
    mul_igmp_type: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    sampling_algorithm: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    flow_active_timeout: Option<u16>,

    #[builder(setter(into, strip_option), default)]
    flow_inactive_timeout: Option<u16>,

    #[builder(setter(into, strip_option), default)]
    total_bytes_exp: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    total_pkts_exp: Option<usize>,

    #[builder(setter(into, strip_option), default)]
    mpls_top_label: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    mpls_top_label_ip_addr: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    flow_sampler_id: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    flow_sampler_mode: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    flow_sampler_random_interval: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    dst_tos: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    src_mac: Option<u64>,

    #[builder(setter(into, strip_option), default)]
    dst_mac: Option<u64>,

    #[builder(setter(into, strip_option), default)]
    src_vlan: Option<u16>,

    #[builder(setter(into, strip_option), default)]
    dst_vlan: Option<u16>,

    #[builder(setter(into, strip_option), default)]
    ip_protocol_version: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    direction: Option<u8>,

    #[builder(setter(into, strip_option), default)]
    ipv6_next_hop: Option<Ipv6Addr>,

    #[builder(setter(into, strip_option), default)]
    bgp_ipv6_next_hop: Option<Ipv6Addr>,

    #[builder(setter(into, strip_option), default)]
    ipv6_option_headers: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    mpls_label_1: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    mpls_label_2: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    mpls_label_3: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    mpls_label_4: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    mpls_label_5: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    mpls_label_6: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    mpls_label_7: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    mpls_label_8: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    mpls_label_9: Option<u32>,

    #[builder(setter(into, strip_option), default)]
    mpls_label_10: Option<u32>,
}
