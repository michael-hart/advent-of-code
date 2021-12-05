mod life_support_meter;
mod power_meter;

use life_support_meter::LifeSupportMeter;
use power_meter::PowerMeter;

#[derive(Debug)]
struct Instruments {
    power: PowerMeter,
    life_support: LifeSupportMeter,
}

impl Instruments {
    fn new(width: usize) -> Instruments {
        Instruments {
            power: PowerMeter::new(width),
            life_support: LifeSupportMeter::new(width),
        }
    }

    fn feed(&mut self, lines: Vec<&str>) {
        self.power.feed(&lines);
        self.life_support.feed(&lines);
    }

    fn power(&self) -> u32 {
        self.power.product()
    }

    fn life_support(&self) -> u32 {
        self.life_support.product()
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let first_line_len = input.lines().next().unwrap().chars().count();
    let mut instruments = Instruments::new(first_line_len);

    instruments.feed(input.lines().collect());

    println!("Part A: {}", instruments.power());
    println!("Part B: {}", instruments.life_support());
}
