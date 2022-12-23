#[derive(Clone, Debug, PartialEq)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Self {
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
                        start = Coords::new(x as i32, y as i32);
                        return 'a' as u32;
                    }
                    if digit == 'E' as u32 {
                        end = Coords::new(x as i32, y as i32);
                        return 'z' as u32;
                    }
                    digit
                }).collect()
            })
            .collect();
        Grid { start, end, grid }
    }
}

#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn next(dir: Option<Dir>) -> Option<Dir> {
        match dir {
            None => Some(Dir::Up),
            Some(Dir::Up) => Some(Dir::Right),
            Some(Dir::Right) => Some(Dir::Down),
            Some(Dir::Down) => Some(Dir::Left),
            Some(Dir::Left) => None,
        }
    }
}

impl Grid {
    fn contains(&self, coords: &Coords) -> bool {
        (coords.x >= 0) &&
        (coords.x < self.grid[0].len() as i32) &&
        (coords.y >= 0) &&
        (coords.y < self.grid.len() as i32)
    }

    fn get(&self, coords: &Coords) -> u32 {
        self.grid[coords.y as usize][coords.x as usize]
    }

    fn fewest_steps(&self) -> usize {
        let mut smallest_steps = usize::max_value();
        let mut steps: Vec<(Coords, Dir)> = vec![(self.start.clone(), Dir::Up)];

        let push_next_dir = |test_tuple: (Coords, Dir), steps: &mut Vec<(Coords, Dir)>| {
            if let Some(next_dir) = Dir::next(Some(test_tuple.1)) {
                steps.push((test_tuple.0, next_dir));
            } else {
                while let Some(prev_tuple) = steps.pop() {
                    if let Some(next_dir) = Dir::next(Some(prev_tuple.1)) {
                        steps.push((prev_tuple.0, next_dir));
                        break;
                    }
                }
            }
        };

        while let Some(test_tuple) = steps.pop() {
            // println!("Just popped: {:?}", &test_tuple);
            if test_tuple.0 == self.end {
                // Reached the end! Backtrack to exhaust all routes.
                smallest_steps = smallest_steps.min(steps.len());
                let prev_tuple = steps.pop().unwrap();
                push_next_dir(prev_tuple, &mut steps);
                continue;
            }

            let next_coords = test_tuple.0.step(&test_tuple.1);

            // Are the coords possible?
            if !self.contains(&next_coords) {
                push_next_dir(test_tuple, &mut steps);
                continue;
            }

            // Is it possible to step this way?
            let current_val = self.get(&test_tuple.0);
            let next_val = self.get(&next_coords);
            if current_val.abs_diff(next_val) > 1 {
                push_next_dir(test_tuple, &mut steps);
                continue;
            }

            // Has this step already been taken?
            let seen = steps.iter().filter(|(x, _)| *x == next_coords).count() > 0;
            if seen {
                push_next_dir(test_tuple, &mut steps);
                continue;
            }

            // Coords are in range; step is possible; we've not been to this square. Push the next coords with Up!
            steps.push(test_tuple);
            steps.push((next_coords, Dir::Up));
        }

        smallest_steps

    }
    
}

fn main() {
    let raw = include_str!("../input.txt");
    let grid = Grid::from(raw);
    println!("{}", grid.fewest_steps());
}
