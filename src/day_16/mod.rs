use crate::shared::{Day, PartSolution};

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u16,
    packet_type: u16,
    inside: PacketInside,
}

struct RemainingStream<T> {
    parsed_contents: T,
    buffer: u16,
    relevant_bits: u8,
    remaining_nibbles: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
enum PacketInside {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThanThan(Vec<Packet>),
    Equal(Vec<Packet>),
}

const VERSION_BITS: u8 = 3;
const PACKET_TYPE_BITS: u8 = 3;
const LENGTH_TYPE_ID_BIT: u8 = 1;
const OPERATOR_PACKET_TYPE_0_SUBPACKET_LENGTH_BITS: u8 = 15;
const OPERATOR_PACKET_TYPE_1_SUBPACKET_COUNT_BITS: u8 = 11;

fn fetch_until_bits_in_buffer(
    buffer: &mut u16,
    nibbles: &mut Vec<u8>,
    bits_loaded_in_buffer: &mut u8,
    bits_needed_in_buffer: u8,
) -> u16 {
    while *bits_loaded_in_buffer < bits_needed_in_buffer {
        match nibbles.pop() {
            Some(nibble) => {
                *buffer <<= 4;
                *buffer |= u16::from(nibble);
            },
            None => {
                println!("You're asking more than we have left!");
            },
        };

        *bits_loaded_in_buffer += 4;
    }

    // the value is the buffer shifted (bits_loaded_in_buffer - bits_needed_in_buffer)
    // AND the bits needed turned into selector
    let value = (*buffer >> (*bits_loaded_in_buffer - bits_needed_in_buffer))
        & (u16::pow(2, u32::from(bits_needed_in_buffer)) - 1);

    // clean out the remainder of the buffer, not needed
    // *buffer &= u16::pow(2, u32::from(*bits_loaded_in_buffer - bits_needed_in_buffer)) - 1;

    // set the new amount of bits loaded in buffer
    *bits_loaded_in_buffer -= bits_needed_in_buffer;

    value
}

fn parse_literal_packet(
    mut buffer: u16,
    mut additional_offset: u8,
    mut nibbles: Vec<u8>,
) -> RemainingStream<u64> {
    // this one bit me, I used a u32
    // and the <<= operation doesn't complain
    // when overwriting data
    let mut number: u64 = 0;

    let mut have_more = true;

    while have_more {
        have_more =
            fetch_until_bits_in_buffer(&mut buffer, &mut nibbles, &mut additional_offset, 1) == 1;

        let number_nibble =
            fetch_until_bits_in_buffer(&mut buffer, &mut nibbles, &mut additional_offset, 4);

        number <<= 4;
        number |= u64::from(number_nibble);
    }

    RemainingStream {
        parsed_contents: number,
        relevant_bits: additional_offset,
        buffer,
        remaining_nibbles: nibbles,
    }
}

fn parse_operator_packet_0(
    mut buffer: u16,
    mut bits_in_buffer: u8,
    mut nibbles: Vec<u8>,
) -> RemainingStream<Vec<Packet>> {
    let mut packets = Vec::new();

    // fetch 15 more bits
    let bits_to_take = fetch_until_bits_in_buffer(
        &mut buffer,
        &mut nibbles,
        &mut bits_in_buffer,
        OPERATOR_PACKET_TYPE_0_SUBPACKET_LENGTH_BITS,
    );

    let mut bits_taken = u16::from(bits_in_buffer);

    let mut new_nibbles = Vec::new();

    while bits_taken < bits_to_take {
        new_nibbles.push(nibbles.pop().unwrap());
        bits_taken += 4;
    }

    new_nibbles.reverse();

    while !new_nibbles.is_empty() {
        let result = parse_packet(buffer, bits_in_buffer, new_nibbles);

        packets.push(result.parsed_contents);
        buffer = result.buffer;
        bits_in_buffer = result.relevant_bits;
        new_nibbles = result.remaining_nibbles;
    }

    RemainingStream {
        parsed_contents: packets,
        relevant_bits: bits_in_buffer,
        buffer,
        remaining_nibbles: nibbles,
    }
}

fn parse_operator_packet_1(
    mut buffer: u16,
    mut relevant_bits: u8,
    mut nibbles: Vec<u8>,
) -> RemainingStream<Vec<Packet>> {
    let mut packets = Vec::new();

    let sub_packets = fetch_until_bits_in_buffer(
        &mut buffer,
        &mut nibbles,
        &mut relevant_bits,
        OPERATOR_PACKET_TYPE_1_SUBPACKET_COUNT_BITS,
    );

    for _ in 0..sub_packets {
        let x = parse_packet(buffer, relevant_bits, nibbles);

        packets.push(x.parsed_contents);
        buffer = x.buffer;
        relevant_bits = x.relevant_bits;
        nibbles = x.remaining_nibbles;
    }

    RemainingStream {
        parsed_contents: packets,
        relevant_bits,
        buffer,
        remaining_nibbles: nibbles,
    }
}

fn parse_packet(
    mut buffer: u16,
    mut relevant_bits: u8,
    mut nibbles: Vec<u8>,
) -> RemainingStream<Packet> {
    // let's get the version, first 3 bits
    let version =
        fetch_until_bits_in_buffer(&mut buffer, &mut nibbles, &mut relevant_bits, VERSION_BITS);

    let packet_type = fetch_until_bits_in_buffer(
        &mut buffer,
        &mut nibbles,
        &mut relevant_bits,
        PACKET_TYPE_BITS,
    );

    let inside = if packet_type == 4 {
        // literal packet
        let result = parse_literal_packet(buffer, relevant_bits, nibbles);

        relevant_bits = result.relevant_bits;
        buffer = result.buffer;
        nibbles = result.remaining_nibbles;

        PacketInside::Literal(result.parsed_contents)
    } else {
        let length_type_id = fetch_until_bits_in_buffer(
            &mut buffer,
            &mut nibbles,
            &mut relevant_bits,
            LENGTH_TYPE_ID_BIT,
        );

        let result = if length_type_id == 0 {
            parse_operator_packet_0(buffer, relevant_bits, nibbles)
        } else {
            parse_operator_packet_1(buffer, relevant_bits, nibbles)
        };

        relevant_bits = result.relevant_bits;
        buffer = result.buffer;
        nibbles = result.remaining_nibbles;

        match packet_type {
            0 => PacketInside::Sum(result.parsed_contents),
            1 => PacketInside::Product(result.parsed_contents),
            2 => PacketInside::Minimum(result.parsed_contents),
            3 => PacketInside::Maximum(result.parsed_contents),
            5 => PacketInside::GreaterThan(result.parsed_contents),
            6 => PacketInside::LessThanThan(result.parsed_contents),
            7 => PacketInside::Equal(result.parsed_contents),
            _ => {
                panic!(
                    "Packet type ({}) / length type id ({}) not supported",
                    packet_type, length_type_id,
                );
            },
        }
    };

    RemainingStream {
        parsed_contents: Packet {
            version,
            packet_type,
            inside,
        },
        relevant_bits,
        buffer,
        remaining_nibbles: nibbles,
    }
}

fn parse_packet_string(packet_string: &str) -> Packet {
    let mut hex = packet_string
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect::<Vec<_>>();

    hex.reverse();

    let result = parse_packet(0, 0, hex);

    result.parsed_contents
}

fn calculate_version_sum(packet: &Packet) -> u32 {
    return u32::from(packet.version)
        + match &packet.inside {
            PacketInside::Literal(_) => 0,
            PacketInside::Sum(v)
            | PacketInside::Product(v)
            | PacketInside::Minimum(v)
            | PacketInside::Maximum(v)
            | PacketInside::GreaterThan(v)
            | PacketInside::LessThanThan(v)
            | PacketInside::Equal(v) => v.iter().map(calculate_version_sum).sum::<u32>(),
        };
}

fn calculate_deep_packet_value(packet: &Packet) -> u64 {
    return match &packet.inside {
        PacketInside::Literal(l) => *l,
        PacketInside::Sum(v) => v.iter().map(calculate_deep_packet_value).sum(),
        PacketInside::Product(v) => v.iter().map(calculate_deep_packet_value).product(),
        PacketInside::Minimum(v) => v.iter().map(calculate_deep_packet_value).min().unwrap(),
        PacketInside::Maximum(v) => v.iter().map(calculate_deep_packet_value).max().unwrap(),
        PacketInside::GreaterThan(v) => {
            assert_eq!(2, v.len());

            let l = calculate_deep_packet_value(&v[0]);
            let r = calculate_deep_packet_value(&v[1]);

            if l > r {
                1
            } else {
                0
            }
        },
        PacketInside::LessThanThan(v) => {
            assert_eq!(2, v.len());

            let l = calculate_deep_packet_value(&v[0]);
            let r = calculate_deep_packet_value(&v[1]);

            if l < r {
                1
            } else {
                0
            }
        },
        PacketInside::Equal(v) => {
            assert_eq!(2, v.len());
            let l = calculate_deep_packet_value(&v[0]);
            let r = calculate_deep_packet_value(&v[1]);

            if l == r {
                1
            } else {
                0
            }
        },
    };
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let translated = parse_packet_string(&lines[0]);

        PartSolution::U32(calculate_version_sum(&translated))
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let translated = parse_packet_string(&lines[0]);

        let total = calculate_deep_packet_value(&translated);

        PartSolution::U64(total)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use crate::{
            day_16::{calculate_version_sum, parse_packet_string, Packet, PacketInside, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(971));
        }

        #[test]
        fn example_literal_1() {
            let example_packet = "D2FE28".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 6,
                    packet_type: 4,
                    inside: PacketInside::Literal(2021),
                },
                translated
            );

            // assert_eq!(6, calculate_history_sum(&translated));
        }

        #[test]
        fn example_operator_length_type_id_1() {
            let example_packet = "38006F45291200".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 1,
                    packet_type: 6,
                    inside: PacketInside::LessThanThan(vec![
                        Packet {
                            version: 6,
                            packet_type: 4,
                            inside: PacketInside::Literal(10),
                        },
                        Packet {
                            version: 2,
                            packet_type: 4,
                            inside: PacketInside::Literal(20),
                        },
                    ]),
                },
                translated
            );

            // assert_eq!(9, calculate_history_sum(&translated));
        }

        #[test]
        fn example_operator_length_type_id_2() {
            let example_packet = "EE00D40C823060".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 7,
                    packet_type: 3,
                    inside: PacketInside::Maximum(vec![
                        Packet {
                            version: 2,
                            packet_type: 4,
                            inside: PacketInside::Literal(1),
                        },
                        Packet {
                            version: 4,
                            packet_type: 4,
                            inside: PacketInside::Literal(2),
                        },
                        Packet {
                            version: 1,
                            packet_type: 4,
                            inside: PacketInside::Literal(3),
                        },
                    ]),
                },
                translated
            );

            // assert_eq!(14, calculate_history_sum(&translated));
        }

        #[test]
        fn example_3() {
            let example_packet = "8A004A801A8002F478".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 4,
                    packet_type: 2,
                    inside: PacketInside::Minimum(vec![Packet {
                        version: 1,
                        packet_type: 2,
                        inside: PacketInside::Minimum(vec![Packet {
                            version: 5,
                            packet_type: 2,
                            inside: PacketInside::Minimum(vec![Packet {
                                version: 6,
                                packet_type: 4,
                                inside: PacketInside::Literal(15)
                            }])
                        }])
                    }])
                },
                translated
            );

            assert_eq!(16, calculate_version_sum(&translated));
        }

        #[test]
        fn example_4() {
            let example_packet = "620080001611562C8802118E34".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 3,
                    packet_type: 0,
                    inside: PacketInside::Sum(vec![
                        Packet {
                            version: 0,
                            packet_type: 0,
                            inside: PacketInside::Sum(vec![
                                Packet {
                                    version: 0,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(10)
                                },
                                Packet {
                                    version: 5,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(11)
                                }
                            ])
                        },
                        Packet {
                            version: 1,
                            packet_type: 0,
                            inside: PacketInside::Sum(vec![
                                Packet {
                                    version: 0,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(12)
                                },
                                Packet {
                                    version: 3,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(13)
                                }
                            ])
                        }
                    ])
                },
                translated
            );

            assert_eq!(12, calculate_version_sum(&translated));
        }

        #[test]
        fn example_5() {
            let example_packet = "C0015000016115A2E0802F182340".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 6,
                    packet_type: 0,
                    inside: PacketInside::Sum(vec![
                        Packet {
                            version: 0,
                            packet_type: 0,
                            inside: PacketInside::Sum(vec![
                                Packet {
                                    version: 0,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(10)
                                },
                                Packet {
                                    version: 6,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(11)
                                }
                            ])
                        },
                        Packet {
                            version: 4,
                            packet_type: 0,
                            inside: PacketInside::Sum(vec![
                                Packet {
                                    version: 7,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(12)
                                },
                                Packet {
                                    version: 0,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(13)
                                }
                            ])
                        }
                    ])
                },
                translated
            );

            assert_eq!(23, calculate_version_sum(&translated));
        }

        #[test]
        fn example_6() {
            let example_packet = "A0016C880162017C3686B18A3D4780".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 5,
                    packet_type: 0,
                    inside: PacketInside::Sum(vec![Packet {
                        version: 1,
                        packet_type: 0,
                        inside: PacketInside::Sum(vec![Packet {
                            version: 3,
                            packet_type: 0,
                            inside: PacketInside::Sum(vec![
                                Packet {
                                    version: 7,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(6)
                                },
                                Packet {
                                    version: 6,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(6)
                                },
                                Packet {
                                    version: 5,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(12)
                                },
                                Packet {
                                    version: 2,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(15)
                                },
                                Packet {
                                    version: 2,
                                    packet_type: 4,
                                    inside: PacketInside::Literal(15)
                                }
                            ])
                        }])
                    }])
                },
                translated
            );

            assert_eq!(31, calculate_version_sum(&translated));
        }
    }

    mod part_2 {
        use crate::{
            day_16::{calculate_deep_packet_value, parse_packet_string, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U64(831_996_589_851));
        }

        #[test]
        fn example_1() {
            let example_packet = "C200B40A82".to_owned();

            let translated = parse_packet_string(&example_packet);

            let total = calculate_deep_packet_value(&translated);

            assert_eq!(3, total);
        }

        #[test]
        fn example_2() {
            let example_packet = "04005AC33890".to_owned();

            let translated = parse_packet_string(&example_packet);

            let total = calculate_deep_packet_value(&translated);

            assert_eq!(54, total);
        }

        #[test]
        fn example_3() {
            let example_packet = "880086C3E88112".to_owned();

            let translated = parse_packet_string(&example_packet);

            let total = calculate_deep_packet_value(&translated);

            assert_eq!(7, total);
        }

        #[test]
        fn example_4() {
            let example_packet = "CE00C43D881120".to_owned();

            let translated = parse_packet_string(&example_packet);

            let total = calculate_deep_packet_value(&translated);

            assert_eq!(9, total);
        }

        #[test]
        fn example_5() {
            let example_packet = "D8005AC2A8F0".to_owned();

            let translated = parse_packet_string(&example_packet);

            let total = calculate_deep_packet_value(&translated);

            assert_eq!(1, total);
        }

        #[test]
        fn example_6() {
            let example_packet = "F600BC2D8F".to_owned();

            let translated = parse_packet_string(&example_packet);

            let total = calculate_deep_packet_value(&translated);

            assert_eq!(0, total);
        }

        #[test]
        fn example_7() {
            let example_packet = "9C005AC2F8F0".to_owned();

            let translated = parse_packet_string(&example_packet);

            let total = calculate_deep_packet_value(&translated);

            assert_eq!(0, total);
        }

        #[test]
        fn example_8() {
            let example_packet = "9C0141080250320F1802104A08".to_owned();

            let translated = parse_packet_string(&example_packet);

            let total = calculate_deep_packet_value(&translated);

            assert_eq!(1, total);
        }
    }
}
