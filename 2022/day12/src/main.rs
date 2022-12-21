use std::slice::Iter;

#[derive(Debug, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Coords { x, y }
    }

    fn step(&self, dir: &Dir) -> Self {
        match dir {
            Dir::Up => Coords::new(self.x, self.y - 1),
            Dir::Down => Coords::new(self.x, self.y + 1),
            Dir::Left => Coords::new(self.x - 1, self.y),
            Dir::Right => Coords::new(self.x + 1, self.y),
        }
    }

}

#[derive(Debug)]
struct Grid {
    start: Coords,
    end: Coords,
    grid: Vec<Vec<u32>>,
}

impl From<&str> for Grid {
    fn from(text: &str) -> Self {
        let mut start = Coords::new(0, 0);
        let mut end = Coords::new(0, 0);
        let grid = text
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim().chars().enumerate().map(|(x, c)| {
                    let digit = c as u32;
                    if digit == 'S' as u32 {
                        start = Coords::new(x, y);
                        return 'a' as u32;
                    }
                    if digit == 'E' as u32 {
                        end = Coords::new(x, y);
                        return 'z' as u32;
                    }
                    digit
                }).collect()
            })
            .collect();
        Grid { start, end, grid }
    }
}

#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn iter()-> Iter<'static, Dir> {
        static dirs: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];
        dirs.iter()
    }

    fn next(dir: Option<Dir>) -> Option<Dir> {
        match dir {
            None => Some(Dir::Up),
            Some(Dir::Up) => Some(Dir::Right),
            Some(Dir::Right) => Some(Dir::Down),
            Some(Dir::Down) => Some(Dir::Left),
            Some(Dir::Left) => None,
        }
    }

    fn is_opposite(&self, other: Option<&&Dir>) -> bool {
        if let Some(other) = other {
            return match (self, other) {
                (Dir::Up, Dir::Down) => true,
                (Dir::Down, Dir::Up) => true,
                (Dir::Right, Dir::Left) => true,
                (Dir::Left, Dir::Right) => true,
                _ => false,
            }
        }
        false
    }
}

impl Grid {
    fn contains(&self, coords: &Coords) -> bool {
        coords.x < self.grid[0].len() && coords.y < self.grid.len()
    }

    fn get(&self, coords: &Coords) -> u32 {
        self.grid[coords.y][coords.x]
    }

    fn fewest_steps(&self) -> usize {
        let mut smallest_steps = usize::max_value();
        let mut current = &self.start;
        let mut steps: Vec<(Coords, Dir)> = vec![];

        // Do we have a current value?
        let mut current_tuple = (current, None);

        while current_tuple != (current, Some(Dir::Left)) {
            let next_tuple = (current, Dir::next(current_tuple.1));
            if next_tuple.1.is_none() {
                // TODO BACKTRACK
            }
            let next_coords = current.step(&next_tuple.1.unwrap());
        }

        'outer: loop {
            for dir in Dir::iter() {
                let prev_step = steps.last();
                // Don't go back on ourselves
                if dir.is_opposite(prev_step) {
                    continue;
                }
                let next = current.step(dir);
                if next == self.end {

                }
                if !self.contains(&next) {
                    continue;
                }
                let current_val = self.get(&current);
                let next_val = self.get(&next);
                if current_val.abs_diff(next_val) > 1 {
                    continue;
                }

                // Otherwise a valid step - take it!
                steps.push(dir);
            }

            // uh-oh - no directions were valid

            // let next = current.step(&Dir::Up);

            // steps.push(Dir::Up);

        }
        
    }
}

fn main() {
    let raw = include_str!("../test.txt");
    println!("{:?}", Grid::from(raw));
}
