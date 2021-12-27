use bitvec::prelude::*;
use hex::FromHex;

fn main() {
    let tvec = Vec::from_hex("D2FE28").unwrap();
    let transmission = tvec.view_bits::<Msb0>();
    let (version, type_id) = (to_u8(&transmission[0..3]), to_u8(&transmission[3..6]));
    let packet = Packet::new(version, type_id, &transmission[6..]);
    println!("{:?}", packet)
}
fn to_u8(slice: &BitSlice<Msb0, u8>) -> u8 {
    u8::from_str_radix(&bitslice_to_string(slice), 2).unwrap()
}
#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    inner: Inner,
}
impl Packet {
    fn new(version: u8, type_id: u8, inner_bits: &BitSlice<Msb0, u8>) -> Self {
        let inner = if type_id == 4 {
            Inner::Literal(Self::parse_literal(inner_bits))
        } else {
            Inner::Children(Self::parse_children(inner_bits))
        };
        Self {
            version,
            type_id,
            inner,
        }
    }
    fn parse_literal(bits: &BitSlice<Msb0, u8>) -> usize {
        let mut vec: BitVec<Msb0, u8> = BitVec::new();
        for chunk in bits.chunks(5) {
            vec.extend_from_bitslice(&chunk[1..]);
            if !chunk[0] {
                break;
            }
        }
        usize::from_str_radix(&bitslice_to_string(vec.as_bitslice()), 2).unwrap()
    }
    fn parse_children(bits: &BitSlice<Msb0, u8>) -> Vec<Packet> {
        todo!()
    }
}
#[derive(Debug)]
enum Inner {
    Literal(usize),
    Children(Vec<Packet>),
}

fn bitslice_to_string(bits: &BitSlice<Msb0, u8>) -> String {
    bits.to_string()
        .replace(", ", "")
        .strip_prefix('[')
        .unwrap()
        .strip_suffix(']')
        .unwrap()
        .to_string()
}
