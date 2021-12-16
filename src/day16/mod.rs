use itertools::Itertools;

pub fn solver(star: u8) -> fn(String) -> i128 {
    match star {
        1 => star1,
        2 => star2,
        _ => panic!("Unknown star!"),
    }
}

struct Packet {
    version: i128,
    ptype: i128,
    packets: Vec<Packet>,
    value: i128,
    length: usize,
}

impl Packet {
    fn parse(bit_str: &mut std::str::Chars) -> Packet {
        let version: String = bit_str.take(3).collect();
        let version = i128::from_str_radix(&version, 2).unwrap();
        let ptype: String = bit_str.take(3).collect();
        let ptype = i128::from_str_radix(&ptype, 2).unwrap();

        match ptype {
            4 => {
                let mut bits: Vec<char> = Vec::new();
                let mut nibbles = 0;
                loop {
                    let flag = bit_str.next().unwrap();
                    bits.extend(bit_str.take(4));
                    nibbles += 1;
                    if flag == '0' {
                        break;
                    }
                }
                let bits: String = bits.into_iter().collect();
                let value = i128::from_str_radix(&bits, 2).unwrap();              

                Packet { 
                    version: version,
                    ptype: ptype,
                    packets: Vec::new(),
                    value: value,
                    length: 6 + nibbles * 5,
                }
            }
            _ => {
                let mut packets = Vec::new();

                let length_type = bit_str.next().unwrap();
                if length_type == '0' {
                    let expected_length: String = bit_str.take(15).collect();
                    let expected_length = usize::from_str_radix(&expected_length, 2).unwrap();

                    let mut actual_length = 0;
                    while actual_length < expected_length {
                        let packet = Packet::parse(bit_str);
                        actual_length += packet.length;
                        packets.push(packet);
                    }

                    if actual_length != expected_length {
                        panic!("Unexpected packet segment length, expected {} but was {}", expected_length, actual_length);
                    }
                    
                    let length = 22 + packets.iter().map(|p| p.length).sum::<usize>(); 

                    Packet { 
                        version: version,
                        ptype: ptype,
                        packets: packets,
                        value: -1,
                        length: length,
                    }
                } else {
                    let packet_count: String = bit_str.take(11).collect();
                    let packet_count = usize::from_str_radix(&packet_count, 2).unwrap();

                    for _ in 0..packet_count {
                        packets.push(Packet::parse(bit_str));
                    }

                    let length = 18 + packets.iter().map(|p| p.length).sum::<usize>(); 

                    Packet { 
                        version: version,
                        ptype: ptype,
                        packets: packets,
                        value: -1,
                        length: length,
                    }
                }
            }
        }        
    }

    fn evaluate(&self) -> i128 {
        match self.ptype {
            0 => self.packets.iter().map(|p| p.evaluate()).sum(),
            1 => self.packets.iter().map(|p| p.evaluate()).product(),
            2 => self.packets.iter().map(|p| p.evaluate()).min().unwrap(),
            3 => self.packets.iter().map(|p| p.evaluate()).max().unwrap(),
            4 => self.value,
            5 => if self.packets[0].evaluate() > self.packets[1].evaluate() { 1 } else { 0 },
            6 => if self.packets[0].evaluate() < self.packets[1].evaluate() { 1 } else { 0 },
            7 => if self.packets[0].evaluate() == self.packets[1].evaluate() { 1 } else { 0 },
            _ => panic!("Unknown packet type {}", self.ptype),
        }
    }
}

fn star1(input: String) -> i128 {
    version_sum_for_input(&input)
}

fn hex_str_to_binary(hex: &str) -> String {
    hex.chars()
        .map(hex_char_to_binary)
        .intersperse("".to_string())
        .collect()
}

fn hex_char_to_binary(hex: char) -> String {
    let n = u8::from_str_radix(&hex.to_string(), 16).unwrap();
    format!("{:04b}", n)
}

fn version_sum(packet: &Packet) -> i128 {
    packet.version + packet.packets.iter().map(|p| version_sum(p)).sum::<i128>()
}

fn version_sum_for_input(input: &str) -> i128 {
    let bit_str = hex_str_to_binary(&input);
    let packet = Packet::parse(&mut bit_str.chars());
    version_sum(&packet)
}

fn star2(input: String) -> i128 {
    evaluate(&input)
}

fn evaluate(input: &str) -> i128 {
    let bit_str = hex_str_to_binary(&input);
    let packet = Packet::parse(&mut bit_str.chars());
    packet.evaluate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_string() {
        assert_eq!(hex_str_to_binary("D2FE28"), "110100101111111000101000");
    }

    #[test]
    fn convert_char() {
        assert_eq!(hex_char_to_binary('D'), "1101");
        assert_eq!(hex_char_to_binary('2'), "0010");
    }

    #[test]
    fn parse_value() {
        let packet = Packet::parse(&mut "110100101111111000101000".chars());
        assert_eq!(packet.length, 21);
        assert_eq!(packet.version, 6);
        assert_eq!(packet.ptype, 4);
        assert_eq!(packet.value, 2021);

        let packet = Packet::parse(&mut "11010001010".chars());
        assert_eq!(packet.length, 11);
        assert_eq!(packet.value, 10);
    }

    #[test]
    fn parse_nested_type_0() {
        let packet = Packet::parse(&mut "00111000000000000110111101000101001010010001001000000000".chars());
        assert_eq!(packet.length, 49);
        assert_eq!(packet.version, 1);
        assert_eq!(packet.ptype, 6);
        assert_eq!(packet.packets.len(), 2);
        assert_eq!(packet.packets[0].value, 10);
        assert_eq!(packet.packets[1].value, 20);
    }

    #[test]
    fn parse_nested_type_1() {
        let packet = Packet::parse(&mut "11101110000000001101010000001100100000100011000001100000".chars());
        assert_eq!(packet.length, 51);
        assert_eq!(packet.version, 7);
        assert_eq!(packet.ptype, 3);
        assert_eq!(packet.packets.len(), 3);
        assert_eq!(packet.packets[0].value, 1);
        assert_eq!(packet.packets[1].value, 2);
        assert_eq!(packet.packets[2].value, 3);
    }

    #[test]
    fn version_sum() {
        assert_eq!(version_sum_for_input("8A004A801A8002F478"), 16);
        assert_eq!(version_sum_for_input("620080001611562C8802118E34"), 12);
        assert_eq!(version_sum_for_input("C0015000016115A2E0802F182340"), 23);
        assert_eq!(version_sum_for_input("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate("C200B40A82"), 3);
        assert_eq!(evaluate("04005AC33890"), 54);
        assert_eq!(evaluate("880086C3E88112"), 7);
        assert_eq!(evaluate("CE00C43D881120"), 9);
        assert_eq!(evaluate("D8005AC2A8F0"), 1);
        assert_eq!(evaluate("F600BC2D8F"), 0);
        assert_eq!(evaluate("9C005AC2F8F0"), 0);
        assert_eq!(evaluate("9C0141080250320F1802104A08"), 1);
    }
}
