use bitvec::prelude::*;
use hex::FromHex;

fn main() {
    let tvec = Vec::from_hex("A0016C880162017C3686B18A3D4780").unwrap();
    let transmission = tvec.view_bits::<Msb0>();
    let packet = Packet::new(transmission);
    println!("{:#?}\n{}", packet, packet.1.version_sum())
}
#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    inner: Inner,
}
impl Packet {
    fn new(bits: &BitSlice<Msb0, u8>) -> (usize, Self) {
        let (version, type_id) = (
            bitslice_to_usize(&bits[0..3]) as u8,
            bitslice_to_usize(&bits[3..6]) as u8,
        );
        Self::inner_new(version, type_id, &bits[6..])
    }
    fn inner_new(version: u8, type_id: u8, inner_bits: &BitSlice<Msb0, u8>) -> (usize, Self) {
        eprintln!("here");
        dbg!(&inner_bits);
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
        (vec.len(), bitslice_to_usize(vec.as_bitslice()))
    }
    fn parse_children(bits: &BitSlice<Msb0, u8>) -> (usize, Vec<Packet>) {
        // dbg!(bits);
        let mut packets = vec![];
        let mut total_length = 0;
        if !bits[0] {
            total_length += 16;
            let bitlength = bitslice_to_usize(&bits[1..16]);
            let mut bits = &bits[16..][..bitlength];
            while bits.len() > 3 {
                let (length, packet) = Self::new(bits);
                packets.push(packet);
                total_length += length;
                bits = &bits[length + 1..];
            }
        } else {
            total_length += 12;
            let inner_packetcount = bitslice_to_usize(&bits[1..12]);
            let mut bits = &bits[12..];
            for _ in 0..inner_packetcount {
                let (length, packet) = Self::new(bits);
                total_length += length;
                packets.push(packet);
                bits = &bits[length + 1..];
            }
        }
        (total_length, packets)
    }
    fn version_sum(&self) -> usize {
        self.version as usize
            + match &self.inner {
                Inner::Literal(_) => 0,
                Inner::Children(children) => children.iter().map(|child| child.version_sum()).sum(),
            }
    }
}
#[derive(Debug)]
enum Inner {
    Literal(usize),
    Children(Vec<Packet>),
}
fn bitslice_to_usize(bits: &BitSlice<Msb0, u8>) -> usize {
    bits.iter()
        .by_val()
        .fold(0, |acc, curr| acc * 2 + curr as usize)
}
