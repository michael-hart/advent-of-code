use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coords {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Coords {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let x = value
            .split(",")
            .next()
            .ok_or("Input string is not in correct format!")?
            .parse::<usize>()
            .map_err(|_| "Input string is not in correct format!")?;
        let y = value
            .split(",")
            .skip(1)
            .next()
            .ok_or("Input string is not in correct format!")?
            .parse::<usize>()
            .map_err(|_| "Input string is not in correct format!")?;

        Ok(Self { x, y })
    }
}

impl Coords {
    fn rock_path(line: &str) -> HashSet<Coords> {
        let raw_cells: Vec<Coords> = line
            .split_whitespace()
            .filter_map(|x| Coords::try_from(x).ok())
            .collect();

        let mut coord_set = HashSet::new();
        for (left, right) in raw_cells.iter().zip(raw_cells[1..].iter()) {
            for x_inner in left.x.min(right.x)..=left.x.max(right.x) {
                for y_inner in left.y.min(right.y)..=left.y.max(right.y) {
                    let current = Coords {
                        x: x_inner,
                        y: y_inner,
                    };
                    coord_set.insert(current);
                }
            }
        }

        coord_set
    }
}

struct CaveSlice {
    cells: HashSet<Coords>,
}

impl From<&str> for CaveSlice {
    fn from(text: &str) -> Self {
        let mut cells = HashSet::new();
        text.lines()
            .for_each(|line| cells.extend(Coords::rock_path(line)));
        Self { cells }
    }
}

impl CaveSlice {
    fn sand_count_until_free_flowing(&mut self) -> usize {
        // As soon as any grain of sand meets y_max, it is considered free-flowing
        let y_max = self
            .cells
            .iter()
            .map(|c| c.y)
            .max()
            .expect("Must have a y_max available!");
        let mut count = 0;

        'outer: loop {
            let mut sand_coords = Coords { x: 500, y: 0 };
            loop {
                if sand_coords.y >= y_max {
                    return count;
                }

                // Try downwards
                let candidate = Coords {
                    x: sand_coords.x,
                    y: sand_coords.y + 1,
                };
                if !self.cells.contains(&candidate) {
                    (sand_coords.x, sand_coords.y) = (candidate.x, candidate.y);
                    continue;
                }

                // Try down/left
                let candidate = Coords {
                    x: sand_coords.x - 1,
                    y: sand_coords.y + 1,
                };
                if !self.cells.contains(&candidate) {
                    (sand_coords.x, sand_coords.y) = (candidate.x, candidate.y);
                    continue;
                }

                // Try down/right
                let candidate = Coords {
                    x: sand_coords.x + 1,
                    y: sand_coords.y + 1,
                };
                if !self.cells.contains(&candidate) {
                    (sand_coords.x, sand_coords.y) = (candidate.x, candidate.y);
                    continue;
                }

                count += 1;
                self.cells.insert(sand_coords);
                continue 'outer;
            }
        }
    }

    fn sand_count_until_full(&mut self) -> usize {
        // As soon as any grain of sand meets y_max, it is considered free-flowing
        let y_floor = self
            .cells
            .iter()
            .map(|c| c.y)
            .max()
            .expect("Must have a y_max available!") + 2;
        let mut count = 0;

        
        let is_filled = |c: &Coords, cells: &HashSet<Coords>| cells.contains(c) || c.y >= y_floor;
        
        'outer: loop {
            let mut sand_coords = Coords { x: 500, y: 0 };
            loop {
                // Try downwards
                let candidate = Coords {
                    x: sand_coords.x,
                    y: sand_coords.y + 1,
                };
                if !is_filled(&candidate, &self.cells) {
                    (sand_coords.x, sand_coords.y) = (candidate.x, candidate.y);
                    continue;
                }

                // Try down/left
                let candidate = Coords {
                    x: sand_coords.x - 1,
                    y: sand_coords.y + 1,
                };
                if !is_filled(&candidate, &self.cells) {
                    (sand_coords.x, sand_coords.y) = (candidate.x, candidate.y);
                    continue;
                }

                // Try down/right
                let candidate = Coords {
                    x: sand_coords.x + 1,
                    y: sand_coords.y + 1,
                };
                if !is_filled(&candidate, &self.cells) {
                    (sand_coords.x, sand_coords.y) = (candidate.x, candidate.y);
                    continue;
                }

                if sand_coords.y == 0 {
                    return count + 1;
                }

                count += 1;
                self.cells.insert(sand_coords);
                continue 'outer;
            }
        }
    }
}

fn main() {
    let raw = include_str!("../input.txt");
    let mut cave_slice = CaveSlice::from(raw);
    let part_a = cave_slice.sand_count_until_free_flowing();
    let part_b = cave_slice.sand_count_until_full() + part_a;
    println!("A: {}", part_a);
    // Reuse old cave slice; the sand coming to rest is the same, so we save a few grains calculation
    println!("B: {}", part_b);
}
