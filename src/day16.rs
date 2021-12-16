use std::cmp::Ordering;

struct Packet {
    version: u64,
    kind: PacketKind,
}

enum PacketKind {
    Literal(u64),
    Operator {
        type_id: u64,
        subpackets: Vec<Packet>,
    },
}

fn main() {
    let input = include_str!("day16input.txt");
    let bits: Vec<u8> = input
        .chars()
        .filter_map(|c| {
            let num = u8::from_str_radix(&format!("{}", c), 16).ok()?;
            Some(
                [0b1000, 0b0100, 0b0010, 0b0001]
                    .into_iter()
                    .map(move |x| if num & x != 0 { 1 } else { 0 }),
            )
        })
        .flatten()
        .collect();

    let (_, packet) = parse_packet(&bits, 0);
    println!("{}", sum(&packet));

    // 2 star
    println!("{}", eval(&packet));
}

fn parse_packet(bits: &[u8], mut pointer: usize) -> (usize, Packet) {
    let version = read(bits, pointer, 3);
    let type_id = read(bits, pointer + 3, 3);
    pointer += 6;

    if type_id == 4 {
        let mut value = 0;
        loop {
            let five = read(bits, pointer, 5);
            pointer += 5;
            value <<= 4;
            value += five & 0b1111;
            if five & 0b10000 == 0 {
                break;
            }
        }
        (
            pointer,
            Packet {
                version,
                kind: PacketKind::Literal(value),
            },
        )
    } else {
        let length_id = read(bits, pointer, 1);
        pointer += 1;

        let mut subpackets = Vec::new();

        if length_id == 0 {
            let length_in_bits = read(bits, pointer, 15) as usize;
            pointer += 15;
            let end = pointer + length_in_bits;
            while pointer < end {
                let (new_pointer, packet) = parse_packet(bits, pointer);
                subpackets.push(packet);
                pointer = new_pointer;
            }
        } else {
            let subpackets_count = read(bits, pointer, 11) as usize;
            pointer += 11;
            for _ in 0..subpackets_count {
                let (new_pointer, packet) = parse_packet(bits, pointer);
                subpackets.push(packet);
                pointer = new_pointer;
            }
        }
        (
            pointer,
            Packet {
                version,
                kind: PacketKind::Operator {
                    type_id,
                    subpackets,
                },
            },
        )
    }
}

fn read(bits: &[u8], pointer: usize, count: usize) -> u64 {
    let mut result = 0;
    for bit in bits.iter().skip(pointer).take(count) {
        result <<= 1;
        result += *bit as u64;
    }
    result
}

fn sum(packet: &Packet) -> u64 {
    packet.version
        + (match &packet.kind {
            PacketKind::Literal(_) => 0,
            PacketKind::Operator { subpackets, .. } => {
                subpackets.iter().map(|packet| sum(packet)).sum()
            }
        })
}

fn eval(packet: &Packet) -> u64 {
    match &packet.kind {
        PacketKind::Literal(value) => *value as u64,
        PacketKind::Operator {
            subpackets,
            type_id,
        } => {
            let mut values = subpackets.iter().map(|packet| eval(packet));
            match type_id {
                0 => values.sum(),
                1 => values.product(),
                2 => values.min().unwrap_or_default(),
                3 => values.max().unwrap_or_default(),
                comparison_type => {
                    let cmp = values
                        .next()
                        .zip(values.next())
                        .map(|(a, b)| a.cmp(&b))
                        .unwrap_or(Ordering::Equal);
                    match (comparison_type, cmp) {
                        (5, Ordering::Greater) => 1,
                        (6, Ordering::Less) => 1,
                        (7, Ordering::Equal) => 1,
                        (_, _) => 0,
                    }
                }
            }
        }
    }
}
