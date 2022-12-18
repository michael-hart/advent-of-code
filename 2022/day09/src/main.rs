use std::collections::HashSet;

#[derive(Debug)]
struct Instruction {
    dir: Pair,
    steps: u32,
}

impl From<&str> for Instruction {
    fn from(text: &str) -> Self {
        let dir = match text.chars().next().unwrap() {
            'R' => Pair::new(1, 0),
            'L' => Pair::new(-1, 0),
            'U' => Pair::new(0, 1),
            'D' => Pair::new(0, -1),
            _ => Pair::new(0, 0),
        };
        let steps = text.chars().skip(2).collect::<String>().parse().unwrap();
        Instruction { dir, steps }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pair {
    x: i32,
    y: i32,
}

impl Pair {
    fn new(x: i32, y: i32) -> Pair {
        Pair { x, y }
    }

    fn add(&mut self, other: &Pair) {
        self.x += other.x;
        self.y += other.y;
    }

    fn too_far(&self, other: &Pair) -> bool {
        self.x.abs_diff(other.x) > 1 || self.y.abs_diff(other.y) > 1
    }
}

fn clamp_1(x: i32) -> i32 {
    x.min(1).max(-1)
}

struct Rope {
    knots: Vec<Pair>,
}

impl Rope {
    fn new(size: usize) -> Self {
        let mut knots = vec![];
        for _ in 0..size {
            knots.push(Pair::new(0, 0));
        }
        Self { knots }
    }

    fn get_num_tail_visited(&mut self, instructions: &[Instruction]) -> usize {
        let mut visited: HashSet<Pair> = HashSet::new();
        visited.insert(Pair::new(0, 0));
        for instr in instructions {
            for _ in 0..instr.steps {
                self.move_head(&instr.dir);
                visited.insert(*self.tail_position());
            }
        }
        visited.len()
    }

    fn move_head(&mut self, direction: &Pair) {
        self.knots.first_mut().unwrap().add(direction);
        self.update_knots();
    }

    fn update_knots(&mut self) {
        for idx in 0..self.knots.len() - 1 {
            let mut iter = self.knots.iter_mut();
            let current = iter.nth(idx).unwrap();
            let next = iter.next().unwrap();
            if current.too_far(next) {
                let diff_x = clamp_1(&current.x - &next.x);
                let diff_y = clamp_1(&current.y - &next.y);
                next.add(&Pair::new(diff_x, diff_y));
            }

        }

    }

    fn tail_position(&self) -> &Pair {
        self.knots.last().unwrap()
    }
}

fn main() {
    let raw = include_str!("../input.txt");
    let instructions = raw.lines().map(Instruction::from).collect::<Vec<Instruction>>();
    println!("A: {}", Rope::new(2).get_num_tail_visited(&instructions));
    println!("B: {}", Rope::new(10).get_num_tail_visited(&instructions));
}
