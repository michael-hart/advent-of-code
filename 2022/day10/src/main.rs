#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl From<&str> for Instruction {
    fn from(text: &str) -> Self {
        if text == "noop" {
            return Instruction::Noop;
        }
        let num = text.split(" ").skip(1).next().unwrap().parse::<i32>().unwrap();
        Instruction::Addx(num)
    }
}

struct CPU {
    register: i32,
    cycle_count: i32,
}
impl CPU {
    fn new() -> Self {
        CPU { register: 1, cycle_count: 0 }
    }
    fn get_sum_signal_strengths(&mut self, instructions: &[Instruction]) -> i32 {
        let mut cumulative_strength = 0;
        for instr in instructions {
            match instr {
                Instruction::Noop => {
                    self.cycle_count += 1;
                    if self.cycle_count >= 20 && (self.cycle_count - 20) % 40 == 0 {
                        cumulative_strength += self.cycle_count * self.register;
                    }
                },
                Instruction::Addx(num) => {
                    self.cycle_count += 1;
                    if self.cycle_count >= 20 && (self.cycle_count - 20) % 40 == 0 {
                        cumulative_strength += self.cycle_count * self.register;
                    }
                    self.cycle_count += 1;
                    if self.cycle_count >= 20 && (self.cycle_count - 20) % 40 == 0 {
                        cumulative_strength += self.cycle_count * self.register;
                    }
                    self.register += num;
                }
            }
        }
        cumulative_strength
    }

    fn next_crt(&mut self, buffer: &mut String) {
        let len = buffer.len() as i32;
        let next_char = if len <= self.register + 1 && len >= self.register - 1 {
            '#'
        } else {
            '.'
        };

        if len == 40 {
            println!("{}", buffer);
            buffer.clear();
        }
        buffer.push(next_char);
    }

    fn draw_screen(&mut self, instructions: &[Instruction]) {
        let mut crt = "".to_owned();
        for instr in instructions {
            self.cycle_count += 1;
            self.next_crt(&mut crt);
            if let Instruction::Addx(num) = instr {
                self.cycle_count += 1;
                self.next_crt(&mut crt);
                self.register += num;
            }
        }
        println!("{}", crt);
    }
}

fn main() {
    let raw = include_str!("../input.txt");
    let instructions = raw.lines().map(Instruction::from).collect::<Vec<Instruction>>();
    println!("A: {}", CPU::new().get_sum_signal_strengths(&instructions));
    println!("B:");
    CPU::new().draw_screen(&instructions);
}
