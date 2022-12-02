
#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn from(x: u32, y: u32) -> Point {
        Point { x, y }
    }
    fn dir(&self, dir: &Direction) -> Point {
        match dir {
            Direction::North => Point::from(self.x, self.y + 1),
            Direction::East => Point::from(self.x + 1, self.y),
            Direction::South => Point::from(self.x, self.y - 1),
            Direction::West => Point::from(self.x - 1, self.y),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellVal {
    Start,
    End,
    Empty,
    Wall,
}

#[derive(Debug)]
struct Cell {
    pos: Point,
    val: CellVal,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<CellVal>>,
}

impl Grid {
    fn from_lines(lines: &str) -> Self {
        let grid = lines
            .lines()
            .map(|l| {
                l
                    .chars()
                    .filter_map(|c| {
                        match c {
                            '#' => Some(CellVal::Wall),
                            '-' => Some(CellVal::Empty),
                            'S' => Some(CellVal::Start),
                            'E' => Some(CellVal::End),
                            _ => None,
                        }
                    })
                    .collect::<Vec<CellVal>>()
            })
            .collect::<Vec<Vec<CellVal>>>();
        // TODO validate: grid is square, grid has start, grid has end.
        Grid { grid }
    }

    fn try_find_start(&self) -> Option<Point> {
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell == CellVal::Start {
                    return Some(Point::from(col_idx as u32, row_idx as u32));
                }
            }
        }
        None
    }

    fn in_bounds(&self, p: &Point) -> bool {
        // println!("{} <= {}?", p.y, self.grid.len());
        if p.y as usize >= self.grid.len() {
            return false;
        }
        // println!("{} <= {}?", p.x, self.grid[0].len());
        if p.x as usize >= self.grid[0].len() {
            return false;
        }
        true
    }

    fn try_get_cell(&self, p: &Point) -> Option<CellVal> {
        if !self.in_bounds(p) {
            return None;
        }
        Some(self.grid[p.y as usize][p.x as usize])
    }
}

fn try_find_path(grid: &Grid) -> Option<Vec<Point>> {
    let start = match grid.try_find_start() {
        Some(s) => s,
        None => return None,
    };
    let mut pos_stack = vec![start];
    let mut dir_stack = vec![];
    loop {
        let mut current = pos_stack.last().expect("Position stack somehow empty!").clone();
        let next_dir = match grid.try_get_cell(&current).expect("Somehow got outside grid!") {
            CellVal::End => return Some(pos_stack),
            CellVal::Start | CellVal::Empty => Direction::North,
            CellVal::Wall => {
                pos_stack.pop();
                current = pos_stack.last().expect("Position stack somehow empty!").clone();
                match dir_stack.pop() {
                    None => return None,
                    Some(Direction::North) => Direction::East,
                    Some(Direction::East) => Direction::South,
                    Some(Direction::South) => Direction::West,
                    Some(Direction::West) => {
                        pos_stack.pop();
                        continue
                    }
                }
            },
        };
        println!("Trying dir {:?} from point {:?}", next_dir, current);
        let current = current.dir(&next_dir);
        // We're doing a back/forth loop from 9,10 to 10,10
        // We have to say we can't go back on ourselves
        // We check if the new position is in the tracked positions
        // If it is, we need to keep trying - but not sure how
        // UGH
        if pos_stack.contains(current) {
            // Can't use direction
        }
        dir_stack.push(next_dir);
        pos_stack.push(current);
    }
}


fn main() {
    let raw = include_str!("../data/input.txt");
    let grid = Grid::from_lines(raw);

    let path = try_find_path(&grid);
    println!("{:?}", path);

    // println!("Start is at point {:?}", start);

    // println!("{:?}", grid);
}
