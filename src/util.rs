use anyhow::{anyhow, Result};
use byteorder::{BigEndian, ByteOrder};

pub fn bytes_to_usize(v: &Vec<u8>) -> Result<usize> {
    let v_len = v.len();
    match v_len {
        1 => Ok(v[0] as usize),
        2 => Ok(BigEndian::read_u16(v.as_slice()) as usize),
        3 => Ok(BigEndian::read_u24(v.as_slice()) as usize),
        4 => Ok(BigEndian::read_u32(v.as_slice()) as usize),
        6 => Ok(BigEndian::read_u48(v.as_slice()) as usize),
        8 => Ok(BigEndian::read_u64(v.as_slice()) as usize),
        16 => Ok(BigEndian::read_u128(v.as_slice()) as usize),
        _ => Err(anyhow!("read error")),
    }
}
