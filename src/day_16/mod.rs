use crate::shared::{Day, PartSolution};

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u32,
    packet_type: u32,
    inside: PacketInside,
}

struct RemainingStream<T> {
    what_you_wanted: T,
    buffer: u32,
    relevant_bits: u32,
    remaining_nibbles: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
enum PacketInside {
    Literal(u32),
    Operator(Vec<Packet>),
}

fn pop_add_and_shift(nibbles: &mut Vec<u8>, buffer: &mut u32) {
    match nibbles.pop() {
        Some(nibble) => {
            println!("{:X}", nibble);

            *buffer <<= 4;
            *buffer |= nibble as u32;
        }
        None => {
            println!("You fooled me, empty operator!");
        }
    };
}

fn parse_literal_packet(
    mut buffer: u32,
    mut additional_offset: u32,
    mut nibbles: Vec<u8>,
) -> RemainingStream<u32> {
    let mut number: u32 = 0;

    loop {
        if additional_offset >= 5 {
            let shift = additional_offset - 5;

            additional_offset -= 5;

            let have_more = (buffer >> (shift + 4)) & 0b0001 == 0b0001;
            let number_nibble = (buffer >> shift) & 0b0000_1111;

            number <<= 4;
            number |= number_nibble;

            if !have_more {
                return RemainingStream {
                    what_you_wanted: number,
                    relevant_bits: shift,
                    buffer,
                    remaining_nibbles: nibbles,
                };
            }
        }

        pop_add_and_shift(&mut nibbles, &mut buffer);
        additional_offset += 4;
    }
}

fn parse_operator_packet_0(
    mut buffer: u32,
    mut relevant_bits: u32,
    mut nibbles: Vec<u8>,
) -> RemainingStream<Vec<Packet>> {
    let mut packets = Vec::new();

    // fetch 15 more bits
    for _ in 0..4 {
        pop_add_and_shift(&mut nibbles, &mut buffer);
        relevant_bits += 4;
    }

    relevant_bits -= 15;

    let bitlength = (buffer >> relevant_bits) & 0b0111_1111_1111_1111;

    // now we grab enough nibbles to satisfy what we have in our buffer + bitlength
    // relevant bits are bits we already have, so we remove them from
    // the ones we need to take
    if relevant_bits > bitlength {
        println!("z");
    }
    let bits_to_take = bitlength - relevant_bits;

    let mut bits_taken = 0;

    let mut new_nibbles = Vec::new();

    while bits_taken < bits_to_take {
        new_nibbles.push(nibbles.pop().unwrap());
        bits_taken += 4;
    }

    new_nibbles.reverse();

    while !new_nibbles.is_empty() {
        let x = parse_packet(buffer & 0b1111, relevant_bits, new_nibbles);

        packets.push(x.what_you_wanted);
        buffer = x.buffer;
        relevant_bits = x.relevant_bits;
        new_nibbles = x.remaining_nibbles;
    }

    return RemainingStream {
        what_you_wanted: packets,
        relevant_bits,
        buffer,
        remaining_nibbles: nibbles,
    };
}

fn parse_operator_packet_1(
    mut buffer: u32,
    mut relevant_bits: u32,
    mut nibbles: Vec<u8>,
) -> RemainingStream<Vec<Packet>> {
    let mut packets = Vec::new();

    // fetch 11 more bits
    for _ in 0..3 {
        pop_add_and_shift(&mut nibbles, &mut buffer);
        relevant_bits += 4;
    }

    relevant_bits -= 11;

    let sub_packets = (buffer >> relevant_bits) & 0b0000_0111_1111_1111;

    for _ in 0..sub_packets {
        let x = parse_packet(buffer & 0b1111, relevant_bits, nibbles);

        packets.push(x.what_you_wanted);
        buffer = x.buffer;
        relevant_bits = x.relevant_bits;
        nibbles = x.remaining_nibbles;
    }

    return RemainingStream {
        what_you_wanted: packets,
        relevant_bits,
        buffer,
        remaining_nibbles: nibbles,
    };
}

fn parse_packet(
    mut buffer: u32,
    mut relevant_bits: u32,
    mut nibbles: Vec<u8>,
) -> RemainingStream<Packet> {
    // first things first, it's a new packet.

    println!("Buffer {:b}", buffer);

    // let's get the version, first 3 bits
    pop_add_and_shift(&mut nibbles, &mut buffer);

    println!("Buffer {:b}", buffer);

    // irrelevant_bits += 4;

    // buffer is
    // 1101
    // ^^^
    // selected is what we need for version number
    let version = (buffer >> (1 + relevant_bits)) & 0b0111;

    // our last bit is the first of the package type, with the next 2 in the next nibble
    pop_add_and_shift(&mut nibbles, &mut buffer);

    println!("Buffer {:b}", buffer);

    // buffer is
    // 11010010
    //    ^^^
    // selected is what we need for version number
    let packet_type = (buffer >> (2 + relevant_bits)) & 0b0111;

    // if it's an operator
    let length_type_id = (buffer >> (1 + relevant_bits)) & 0b0001;

    let inside = if packet_type == 4 {
        // literal packet
        let result = parse_literal_packet(
            buffer,
            relevant_bits + 2, /* 2 because we 'used' 6 in this function */
            nibbles,
        );

        relevant_bits = result.relevant_bits;
        buffer = result.buffer;
        nibbles = result.remaining_nibbles;

        PacketInside::Literal(result.what_you_wanted)
    } else if length_type_id == 0 {
        let result = parse_operator_packet_0(
            buffer,
            relevant_bits + 1, /* 2 because we 'used' 7 in this function */
            nibbles,
        );

        relevant_bits = result.relevant_bits;
        buffer = result.buffer;
        nibbles = result.remaining_nibbles;

        PacketInside::Operator(result.what_you_wanted)
    } else if length_type_id == 1 {
        let result = parse_operator_packet_1(
            buffer,
            relevant_bits + 1, /* 2 because we 'used' 7 in this function */
            nibbles,
        );

        relevant_bits = result.relevant_bits;
        buffer = result.buffer;
        nibbles = result.remaining_nibbles;

        PacketInside::Operator(result.what_you_wanted)
    } else {
        panic!(
            "Packet type ({}) / length type id ({}) not supported",
            packet_type, length_type_id,
        );
    };

    println!("{}, {}", version, packet_type);

    RemainingStream {
        what_you_wanted: Packet {
            version,
            packet_type,
            inside,
        },
        relevant_bits,
        buffer,
        remaining_nibbles: nibbles,
    }
}

fn parse_packet_string(packet_string: &String) -> Packet {
    let mut hex = packet_string
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect::<Vec<_>>();

    hex.reverse();

    let result = parse_packet(0, 0, hex);

    // assert_eq!(0, result.remaining_nibbles.len());

    result.what_you_wanted
}

fn calculate_history_sum(packet: &Packet) -> u32 {
    return packet.version
        + match &packet.inside {
            PacketInside::Literal(_) => 0,
            PacketInside::Operator(v) => v.iter().map(|p| calculate_history_sum(p)).sum::<u32>(),
        };
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let translated = parse_packet_string(&lines[0]);

        PartSolution::U32(calculate_history_sum(&translated))
    }

    fn part_2(&self) -> PartSolution {
        todo!();
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use crate::{
            day_16::{calculate_history_sum, parse_packet_string, Packet, PacketInside, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(604));
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

            assert_eq!(6, calculate_history_sum(&translated));
        }

        #[test]
        fn example_operator_length_type_id_1() {
            let example_packet = "38006F45291200".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 1,
                    packet_type: 6,
                    inside: PacketInside::Operator(vec![
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

            assert_eq!(9, calculate_history_sum(&translated));
        }

        #[test]
        fn example_operator_length_type_id_2() {
            let example_packet = "EE00D40C823060".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 7,
                    packet_type: 3,
                    inside: PacketInside::Operator(vec![
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

            assert_eq!(14, calculate_history_sum(&translated));
        }

        #[test]
        fn example_3() {
            let example_packet = "8A004A801A8002F478".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 4,
                    packet_type: 2,
                    inside: PacketInside::Operator(vec![Packet {
                        version: 1,
                        packet_type: 2,
                        inside: PacketInside::Operator(vec![Packet {
                            version: 5,
                            packet_type: 2,
                            inside: PacketInside::Operator(vec![Packet {
                                version: 6,
                                packet_type: 4,
                                inside: PacketInside::Literal(15)
                            }])
                        }])
                    }])
                },
                translated
            );

            assert_eq!(16, calculate_history_sum(&translated));
        }

        #[test]
        fn example_4() {
            let example_packet = "620080001611562C8802118E34".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 3,
                    packet_type: 0,
                    inside: PacketInside::Operator(vec![
                        Packet {
                            version: 0,
                            packet_type: 0,
                            inside: PacketInside::Operator(vec![
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
                            inside: PacketInside::Operator(vec![
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

            assert_eq!(12, calculate_history_sum(&translated));
        }

        #[test]
        fn example_5() {
            let example_packet = "C0015000016115A2E0802F182340".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 6,
                    packet_type: 0,
                    inside: PacketInside::Operator(vec![
                        Packet {
                            version: 0,
                            packet_type: 0,
                            inside: PacketInside::Operator(vec![
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
                            inside: PacketInside::Operator(vec![
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

            assert_eq!(23, calculate_history_sum(&translated));
        }

        #[test]
        fn example_6() {
            let example_packet = "A0016C880162017C3686B18A3D4780".to_owned();

            let translated = parse_packet_string(&example_packet);

            assert_eq!(
                Packet {
                    version: 5,
                    packet_type: 0,
                    inside: PacketInside::Operator(vec![Packet {
                        version: 1,
                        packet_type: 0,
                        inside: PacketInside::Operator(vec![Packet {
                            version: 3,
                            packet_type: 0,
                            inside: PacketInside::Operator(vec![
                                Packet {
                                    version: 1,
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

            assert_eq!(31, calculate_history_sum(&translated));
        }
    }
}
