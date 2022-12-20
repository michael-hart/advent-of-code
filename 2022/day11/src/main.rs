use lazy_static::lazy_static;
use regex::{Regex, Captures};

const RE_IN: &str = "Monkey \\d+:\\s+Starting items: (.*)\\s+Operation: new = (.*)\\s+Test: divisible by (.*)\\s+If true: .* (.*)\\s+If false: .* (.*)";

lazy_static! {
    static ref RE: Regex = Regex::new(RE_IN).expect("Could not compile regex!");
}

struct Monkey {
    items: Vec<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    test_divisor: i64,
    true_dest: usize,
    false_dest: usize,
    items_inspected: usize,
}

impl From<Captures<'_>> for Monkey {
    fn from(caps: Captures) -> Self {
        let starting_items_raw = caps.get(1).map(|x| x.as_str()).unwrap();
        let operation_raw = caps.get(2).map(|x| x.as_str()).unwrap();
        let test_divisor = caps.get(3).map(|x| x.as_str().trim().parse::<i64>().unwrap()).unwrap();
        let true_monkey = caps.get(4).map(|x| x.as_str().trim().parse::<usize>().unwrap()).unwrap();
        let false_monkey = caps.get(5).map(|x| x.as_str().trim().parse::<usize>().unwrap()).unwrap();

        let items = starting_items_raw.trim().split(", ").filter_map(|x| x.parse::<i64>().ok()).collect();

        let mut operation_part_iter = operation_raw.split_whitespace();
        operation_part_iter.next();
        let op_operator = operation_part_iter.next().unwrap();
        let op_right_operand = operation_part_iter.next().unwrap();

        let op: Box<dyn Fn(i64) -> i64> = match op_operator {
            "+" => match op_right_operand {
                "old" => Box::new(move |x| x + x),
                _ => {
                    let right = op_right_operand.parse::<i64>().unwrap();
                    Box::new(move |x| x + right)
                },
            },
            "*" => match op_right_operand {
                "old" => Box::new(move |x| x * x),
                _ => {
                    let right = op_right_operand.parse::<i64>().unwrap();
                    Box::new(move |x| x * right)
                },
            },
            _ => panic!("Unknown operator!!"),
        };

        Monkey { items, op, test_divisor, true_dest: true_monkey, false_dest: false_monkey, items_inspected: 0 }
    }
}

impl Monkey {
    fn tick(&mut self, divide_worry: bool, modulo: i64) -> Vec<(usize, i64)> {
        let mut results = vec![];
        // println!("{:?}", &self.items);
        while let Some(item) = self.items.pop() {
            self.items_inspected += 1;
            let mut new_worry = (*self.op)(item) % modulo;
            if divide_worry {
                new_worry = new_worry / 3;
            }
            if new_worry % self.test_divisor == 0 {
                results.push((self.true_dest, new_worry));
            } else {
                results.push((self.false_dest, new_worry))
            }
        }
        results
    }
}

fn do_round(monkeys: &mut Vec<Monkey>, divide_worry: bool, modulo: i64) {
    for idx in 0..monkeys.len() {
        let next_for_monkey = monkeys.get_mut(idx).unwrap().tick(divide_worry, modulo);
        for (dest_idx, val) in next_for_monkey {
            monkeys.get_mut(dest_idx).unwrap().items.push(val);
        }
    }
}

fn main() {
    let raw = include_str!("../input.txt");

    let mut monkeys = RE.captures_iter(raw).map(Monkey::from).collect::<Vec<Monkey>>();
    (0..20).for_each(|_| do_round(&mut monkeys, true, 1));
    let mut inspected_items: Vec<usize> = monkeys.iter().map(|m| m.items_inspected).collect();
    inspected_items.sort();
    let part_a: usize = inspected_items.iter().skip(inspected_items.len() - 2).product();
    println!("A: {}", part_a);

    let mut monkeys = RE.captures_iter(raw).map(Monkey::from).collect::<Vec<Monkey>>();
    let modulo = monkeys.iter().map(|m| m.test_divisor).product();
    (0..10000).for_each(|_| do_round(&mut monkeys, false, modulo));
    let mut inspected_items: Vec<usize> = monkeys.iter().map(|m| m.items_inspected).collect();
    inspected_items.sort();
    // println!("Inspected: {:?}", inspected_items);
    let part_b: usize = inspected_items.iter().skip(inspected_items.len() - 2).product();
    println!("B: {}", part_b);
}
