mod map_cell;

use std::convert::TryFrom;
use grid::{Grid, Point};
use crate::map_cell::MapCell;

fn multiply_slopes(grid: &Grid<MapCell>, slopes: &[Point]) -> u64 {
    slopes
        .iter()
        .map(|s| count_trees(&grid, &s))
        .fold(1, |a, x| a * x)
}

fn count_trees(grid: &Grid<MapCell>, delta: &Point) -> u64 {
    let mut count = 0;
    let mut current = Point::new(0, 0);
    while current.y < grid.height() {
        if grid.cell_at(&current) == Some(&MapCell::Tree) {
            count += 1;
        }

        current += *delta;
        if current.x >= grid.width() {
            current = Point::new(current.x % grid.width(), current.y);
        }
    }

    count
}

fn main() {
    let grid: Grid<MapCell> = Grid::try_from(include_str!("../input.txt")).unwrap();
    let slopes = vec![
            Point::new(1, 1),
            Point::new(3, 1),
            Point::new(5, 1),
            Point::new(7, 1),
            Point::new(1, 2),
        ];
    println!("{} trees", multiply_slopes(&grid, &slopes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid: Grid<MapCell> = Grid::try_from(include_str!("../test.txt")).unwrap();
        let slopes = vec![
            Point::new(1, 1),
            Point::new(3, 1),
            Point::new(5, 1),
            Point::new(7, 1),
            Point::new(1, 2),
        ];
        assert_eq!(multiply_slopes(&grid, &slopes), 336);
    }
}
