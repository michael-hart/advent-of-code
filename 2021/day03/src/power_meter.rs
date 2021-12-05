#[derive(Debug)]
struct Counter {
    ones: u32,
    zeros: u32,
}

impl Default for Counter {
    fn default() -> Self {
        Self { ones: 0, zeros: 0 }
    }
}

impl Counter {
    fn most_freq(&self) -> u32 {
        if self.ones > self.zeros {
            1
        } else {
            0
        }
    }

    fn least_freq(&self) -> u32 {
        if self.ones < self.zeros {
            1
        } else {
            0
        }
    }

    fn update(&mut self, c: char) {
        match c {
            '1' => { self.ones += 1; () },
            '0' => { self.zeros += 1; () },
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct PowerMeter {
    set: Vec<Counter>,
}

impl PowerMeter {
    pub fn new(width: usize) -> Self {
        let mut set = vec![];
        for _ in 0..width {
            set.push(Counter::default());
        }
        Self { set }
    }

    pub fn feed(&mut self, lines: &Vec<&str>) {
        lines
            .iter()
            .for_each(|l| 
                l
                    .chars()
                    .zip(self.set.iter_mut())
                    .for_each(|(c, counter)| counter.update(c))
            );
    }

    pub fn product(&self) -> u32 {
        let mut gamma = 0;
        let mut epsilon = 0;
        for counter in &self.set {
            gamma = 2*gamma + counter.most_freq();
            epsilon = 2*epsilon + counter.least_freq();
        }

        gamma * epsilon
    }
}
