#[derive(Debug)]
struct Packet {
    contents: PacketContentsType,
}

#[derive(Debug)]
enum PacketContentsType {
    Array(Vec<PacketContentsType>),
    Value(u32),
}

impl From<&str> for Packet {
    fn from(text: &str) -> Self {
        let mut stack: Vec<PacketContentsType> = vec![];
        let mut current: Option<PacketContentsType> = None;
        for c in text.chars() {
            if let Some(parsed) = c.to_digit(10) {
                match current.get_or_insert(PacketContentsType::Array(vec![])) {
                    PacketContentsType::Array(x) => x.push(PacketContentsType::Value(parsed)),
                    _ => (),
                }
                continue;
            }
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
    fn inner_correct_order((left, right): (&PacketContentsType, &PacketContentsType)) -> bool {
        // Try to take the next element and test it. If there's no more left elements, it's okay. If there are no more right elements, it's not okay.
        match (left, right) {
            (PacketContentsType::Value(l), PacketContentsType::Value(r)) => l < r,
            (PacketContentsType::Array(_), PacketContentsType::Value(r)) => {
                PacketPair::inner_correct_order((left, &PacketContentsType::Array(vec![PacketContentsType::Value(*r)])))
            },
            (PacketContentsType::Value(l), PacketContentsType::Array(_)) => {
                PacketPair::inner_correct_order((&PacketContentsType::Array(vec![PacketContentsType::Value(*l)]), right))
            },
            (PacketContentsType::Array(l), PacketContentsType::Array(r)) => {
                let mut l_iter = l.iter();
                let mut r_iter = r.iter();
                let mut matching = true;
                loop {
                    if !matching {
                        return false;
                    }
                    matching = matching && match (l_iter.next(), r_iter.next()) {
                        // If two values, check together
                        (Some(PacketContentsType::Value(l_val)), Some(PacketContentsType::Value(r_val))) => {
                            if l_val > r_val {
                                return false;
                            } else if l_val < r_val {
                                return true;
                            }
                            true
                        },
                        // If both out of entries, return matching!
                        (None, None) => return matching,
                        // If left is out of entries, we are good!
                        (None, _) => return true,
                        // If right is out of entries, we are not good!
                        (_, None) => return false,
                        // Otherwise, recurse
                        (Some(l_next), Some(r_next)) => PacketPair::inner_correct_order((&l_next, &r_next)),
                    }
                }
            },
        }
    }

    fn is_in_correct_order(&self) -> bool {
        PacketPair::inner_correct_order((&self.left.contents, &self.right.contents))
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
}

fn main() {
    let raw = include_str!("../input.txt");
    let pair_list = PacketPairList::from(raw);
    println!("A: {}", pair_list.sum_of_indices_of_correct_order());
}
