use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
enum Cell {
    Rock,
    Sand,
    Air,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coords {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Coords {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {

        let x = value.split(",").next().ok_or("Input string is not in correct format!")?.parse::<usize>().map_err(|_| "Input string is not in correct format!")?;
        let y = value.split(",").skip(1).next().ok_or("Input string is not in correct format!")?.parse::<usize>().map_err(|_| "Input string is not in correct format!")?;

        Ok(Self { x, y })
    }
}

impl Coords {
    fn rock_path(line: &str) -> HashMap<Coords, Cell> {
        // Split by space, filter_map to Coords, then build path
        let raw_cells: Vec<Coords> = line.split_whitespace().filter_map(|x| Coords::try_from(x).ok()).collect();

        let mut map = HashMap::new();
        for (left, right) in raw_cells.iter().zip(raw_cells[1..].iter()) {
            for x_inner in left.x.min(right.x)..=left.x.max(right.x) {
                for y_inner in left.y.min(right.y)..=left.y.max(right.y) {
                    let current = Coords { x: x_inner, y: y_inner };
                    map.insert(current, Cell::Rock);
                }
            }
        }

        map
    }
}

struct CaveSlice {
    cells: HashMap<Coords, Cell>,
}

impl From<&str> for CaveSlice {
    fn from(text: &str) -> Self {
        let mut cells = HashMap::new();
        text.lines().for_each(|line| cells.extend(Coords::rock_path(line)));
        Self { cells }
    }
}

fn main() {
    let raw = include_str!("../test.txt");
    let cave_slice = CaveSlice::from(raw);
    println!("Got path: {:?}", cave_slice.cells);
}
