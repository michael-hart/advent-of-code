mod map_cell;

use std::convert::TryFrom;
use grid::{Grid, Point};
use crate::map_cell::MapCell;

fn count_trees(grid: &Grid<MapCell>, delta: Point) -> u64 {
    let mut count = 0;
    let mut current = Point::new(0, 0);
    while current.y < grid.height() {
        if grid.cell_at(&current) == Some(&MapCell::Tree) {
            count += 1;
        }

        current += delta;
        if current.x >= grid.width() {
            current = Point::new(current.x % grid.width(), current.y);
        }
    }

    count
}

fn main() {
    let grid: Grid<MapCell> = Grid::try_from(include_str!("../input.txt")).unwrap();
    println!("{} trees", count_trees(&grid, Point::new(3, 1)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid: Grid<MapCell> = Grid::try_from(include_str!("../test.txt")).unwrap();
        println!("{}", &grid);
        assert_eq!(count_trees(&grid, Point::new(3, 1)), 7);
    }
}
