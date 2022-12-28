use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Packet {
    contents: PacketContentsType,
}

#[derive(Debug, Eq, PartialEq)]
enum PacketContentsType {
    Array(Vec<PacketContentsType>),
    Value(u32),
}

impl PacketContentsType {
    fn pkt_cmp(&self, other: &PacketContentsType) -> Ordering {
        match (self, other) {
            (PacketContentsType::Value(l), PacketContentsType::Value(r)) => l.cmp(r),
            (PacketContentsType::Array(_), PacketContentsType::Value(r)) => {
                PacketContentsType::pkt_cmp(&self, &PacketContentsType::Array(vec![PacketContentsType::Value(*r)]))
            },
            (PacketContentsType::Value(l), PacketContentsType::Array(_)) => {
                PacketContentsType::pkt_cmp(&PacketContentsType::Array(vec![PacketContentsType::Value(*l)]), other)
            },
            (PacketContentsType::Array(l), PacketContentsType::Array(r)) => {
                let mut l_iter = l.iter();
                let mut r_iter = r.iter();
                loop {
                    match (l_iter.next(), r_iter.next()) {
                        (None, None) => break Ordering::Equal,
                        (None, _) => break Ordering::Less,
                        (_, None) => break Ordering::Greater,
                        (Some(l_inner), Some(r_inner)) => {
                            let result = PacketContentsType::pkt_cmp(l_inner, r_inner);
                            if result != Ordering::Equal {
                                break result;
                            }
                        }
                    }
                }
            },
        }
    }
}

impl PartialOrd for PacketContentsType {
    fn ge(&self, other: &Self) -> bool {
        PacketContentsType::pkt_cmp(self, other) != Ordering::Less
    }
    fn gt(&self, other: &Self) -> bool {
        PacketContentsType::pkt_cmp(self, other) == Ordering::Greater
    }
    fn le(&self, other: &Self) -> bool {
        PacketContentsType::pkt_cmp(self, other) != Ordering::Greater
    }
    fn lt(&self, other: &Self) -> bool {
        PacketContentsType::pkt_cmp(self, other) == Ordering::Less
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(PacketContentsType::pkt_cmp(self, other))
    }
}

impl Ord for PacketContentsType {
    fn cmp(&self, other: &Self) -> Ordering {
        PacketContentsType::pkt_cmp(self, other)
    }
}

impl From<&str> for Packet {
    fn from(text: &str) -> Self {
        let mut stack: Vec<PacketContentsType> = vec![];
        let mut current: Option<PacketContentsType> = None;
        let mut char_buf = text.chars().collect::<VecDeque<char>>();

        while char_buf.len() > 0 {
            let num_buf: String = char_buf.iter().take_while(|x| x.to_digit(10).is_some()).collect();
            if num_buf.len() > 0 {
                for _ in 0..num_buf.len() {
                    char_buf.pop_front();
                }
                let parsed = num_buf.parse::<u32>().expect("Number is too large to parse as u32!");
                match current.get_or_insert(PacketContentsType::Array(vec![])) {
                    PacketContentsType::Array(x) => x.push(PacketContentsType::Value(parsed)),
                    _ => (),
                }
                continue;
            }
            let c = char_buf.pop_front().expect("No way to get this error; while loop condition checks length");
            match c {
                '[' => {
                    // current.take().and_then(|x| stack.push(PacketContentsType::Array(x)));
                    if let Some(current_inner) = current {
                        stack.push(current_inner);
                    }
                    current = Some(PacketContentsType::Array(vec![]));
                },
                ']' => {
                    current = match stack.pop() {
                        Some(PacketContentsType::Array(mut stack_inner)) => {
                            stack_inner.push(current.take().unwrap());
                            Some(PacketContentsType::Array(stack_inner))
                        },
                        _ => return Self { contents: current.unwrap() },
                    };
                },
                _ => (),
            }
        }

        panic!("Malformed packet!");
    }
}

struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn is_in_correct_order(&self) -> bool {
        self.left.contents.cmp(&self.right.contents) != Ordering::Greater
    }

    fn from(left_line: &str, right_line: &str) -> Self {
        Self { left: Packet::from(left_line), right: Packet::from(right_line) }
    }
}

struct PacketPairList {
    pairs: Vec<PacketPair>,
}

impl From<&str> for PacketPairList {
    fn from(text: &str) -> Self {
        let mut lines = text.lines();
        let mut pairs = vec![];
        while let (Some(left_line), Some(right_line)) = (lines.next(), lines.next()) {
            // Skip the blank line, if any
            lines.next();
            pairs.push(PacketPair::from(left_line, right_line));
        }

        PacketPairList { pairs }
    }
}

impl PacketPairList {
    fn sum_of_indices_of_correct_order(&self) -> usize {
        self.pairs.iter()
            .enumerate()
            .filter_map(|(idx, x)| if x.is_in_correct_order() { Some(idx + 1) } else { None })
            .sum()
    }

    fn multiplied_indices_of_dividers(&self) -> Option<u32> {
        // Build vector of all packets
        let mut all_packets = vec![];
        self.pairs.iter().for_each(|pair| {
            all_packets.push(&pair.left.contents);
            all_packets.push(&pair.right.contents);
        });

        // Add divider packets
        let divider = |val| PacketContentsType::Array(vec![PacketContentsType::Array(vec![PacketContentsType::Value(val)])]);
        let div_2 = divider(2);
        let div_6 = divider(6);
        all_packets.push(&div_2);
        all_packets.push(&div_6);

        // Sort
        all_packets.sort();

        // Find indices of dividers in list; multiply together
        let idx_2 = all_packets.iter().position(|x| x == &&div_2);
        let idx_6 = all_packets.iter().position(|x| x == &&div_6);
        match (idx_2, idx_6) {
            (Some(l), Some(r)) => Some((l + 1) as u32 * (r + 1) as u32),
            _ => None,
        }
    }
}

fn main() {
    let raw = include_str!("../input.txt");
    let pair_list = PacketPairList::from(raw);
    println!("A: {}", pair_list.sum_of_indices_of_correct_order());
    println!("B: {:?}", pair_list.multiplied_indices_of_dividers());
}
