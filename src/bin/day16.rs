use bitvec::prelude::*;
use hex::FromHex;

fn main() {
    let tvec = Vec::from_hex("8A004A801A8002F478").unwrap();
    let transmission = tvec.view_bits::<Msb0>();
    let packet = Packet::new(transmission);
    println!("{:#?}", packet)
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
    fn new(bits: &BitSlice<Msb0, u8>) -> (usize, Self) {
        let (version, type_id) = (to_u8(&bits[0..3]), to_u8(&bits[3..6]));
        Self::inner_new(version, type_id, &bits[6..])
    }
    fn inner_new(version: u8, type_id: u8, inner_bits: &BitSlice<Msb0, u8>) -> (usize, Self) {
        dbg!(inner_bits);
        let inner = if type_id == 4 {
            let literal = Self::parse_literal(inner_bits);
            (literal.0, Inner::Literal(literal.1))
        } else {
            let children = Self::parse_children(inner_bits);
            (children.0, Inner::Children(children.1))
        };
        (
            6 + inner.0,
            Self {
                version,
                type_id,
                inner: inner.1,
            },
        )
    }
    fn parse_literal(bits: &BitSlice<Msb0, u8>) -> (usize, usize) {
        let mut vec: BitVec<Msb0, u8> = BitVec::new();
        for chunk in bits.chunks(5) {
            vec.extend_from_bitslice(&chunk[1..]);
            if !chunk[0] {
                break;
            }
        }
        (
            vec.len(),
            usize::from_str_radix(&bitslice_to_string(vec.as_bitslice()), 2).unwrap(),
        )
    }
    fn parse_children(bits: &BitSlice<Msb0, u8>) -> (usize, Vec<Packet>) {
        dbg!(bits);
        let mut packets = vec![];
        let mut total_length = 0;
        if !bits[0] {
            total_length += 15;
            let bitlength =
                u16::from_str_radix(&bitslice_to_string(&dbg!(bits)[1..16]), 2).unwrap();
            let mut bits = &bits[16..][..bitlength as usize];
            while bits.len() > 3 {
                let (length, packet) = Self::new(bits);
                packets.push(packet);
                total_length += length;
                bits = bits.split_at(length + 1).1;
            }
        } else {
            total_length += 11;
            let inner_packetcount =
                u16::from_str_radix(&bitslice_to_string(&bits[1..12]), 2).unwrap();
            let mut bits = &bits[12..];
            for _ in 0..inner_packetcount {
                let (length, packet) = Self::new(bits);
                total_length += length;
                packets.push(packet);
                bits = bits.split_at(length + 1).1;
            }
        }
        (total_length, packets)
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
