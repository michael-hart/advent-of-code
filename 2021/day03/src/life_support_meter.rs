#[derive(Debug)]
pub struct LifeSupportMeter {
    width: usize,
    oxygen: u32,
    co2: u32,
}

fn filter_preferred(set: &Vec<&str>, width: usize, preferred: char) -> u32 {
    let mut my_set = set.clone();
    for i in 0..width {
        let mut ones = 0;
        let mut zeros = 0;
        // Loop once to find the character we want
        for line in &my_set {
            let c = line.chars().nth(i).unwrap();
            match c {
                '1' => { ones += 1; () },
                '0' => { zeros += 1; () },
                _ => (),
            }
        }

        let is_preferred = if preferred == '0' { zeros <= ones } else { ones >= zeros };
        let character_needed = if is_preferred { preferred } else {
            if preferred == '1' { '0' } else { '1' }
        };

        my_set = my_set
            .iter()
            .filter(|&l| l.chars().nth(i).unwrap() == character_needed)
            .map(|l| *l)
            .collect();

        if my_set.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(my_set.first().unwrap(), 2).unwrap()
}

impl LifeSupportMeter {
    pub fn new(width: usize) -> Self {
        Self { width, oxygen: 0, co2: 0 }
    }

    pub fn feed(&mut self, lines: &Vec<&str>) {
        self.oxygen = filter_preferred(lines, self.width, '1');
        self.co2 = filter_preferred(lines, self.width, '0');
    }

    pub fn product(&self) -> u32 {
        self.oxygen * self.co2
    }
}
