use petgraph::visit::EdgeRef;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Grid {
    fn width(&self) -> usize {
        self.grid[0].len()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn contains(&self, coords: &Coords) -> bool {
        (coords.x >= 0) &&
        (coords.x < self.width() as i32) &&
        (coords.y >= 0) &&
        (coords.y < self.height() as i32)
    }

    fn get(&self, coords: &Coords) -> u32 {
        self.grid[coords.y as usize][coords.x as usize]
    }

    fn as_graph(&self) -> petgraph::graphmap::GraphMap<Coords, u32, petgraph::Directed> {
        let mut graph = petgraph::graphmap::GraphMap::default();
        for row_idx in 0..self.height() {
            for col_idx in 0..self.width() {
                let coords = Coords::new(col_idx as i32, row_idx as i32);
                let current_val = self.get(&coords);
                for dir in [Dir::Right, Dir::Down] {
                    let check_coords = coords.step(&dir);
                    if self.contains(&check_coords) {
                        let check_val = self.get(&check_coords);
                        // If stepping level or up/down one, add both direction
                        if check_val.abs_diff(current_val) <= 1 {
                            graph.add_edge(coords, check_coords, 1);
                            graph.add_edge(check_coords, coords, 1);
                        // If stepping down a lot, that's allowed, but only in this direction
                        } else if current_val > check_val {
                            graph.add_edge(coords, check_coords, 1);
                        } else {
                            graph.add_edge(check_coords, coords, 1);
                        }
                    }
                }
            }
        }
        graph
    }

    fn as_rev_graph(&self) -> petgraph::graphmap::GraphMap<Coords, u32, petgraph::Directed> {
        // This graph is to allow reverse searching from 'z' to all 'a' tiles. As such, we're
        // not allowed to just step down as much as we want - we're travelling backwards, so we
        // flip the rules so we can only step down one, but step up as many as we want.
        let mut graph = petgraph::graphmap::GraphMap::default();
        for row_idx in 0..self.height() {
            for col_idx in 0..self.width() {
                let coords = Coords::new(col_idx as i32, row_idx as i32);
                let current_val = self.get(&coords);
                for dir in [Dir::Right, Dir::Down] {
                    let check_coords = coords.step(&dir);
                    if self.contains(&check_coords) {
                        let check_val = self.get(&check_coords);
                        if check_val.abs_diff(current_val) <= 1 {
                            graph.add_edge(coords, check_coords, 1);
                            graph.add_edge(check_coords, coords, 1);
                        } else if current_val > check_val {
                            graph.add_edge(check_coords, coords, 1);
                        } else {
                            graph.add_edge(coords, check_coords, 1);
                        }
                    }
                }
            }
        }
        graph
    }

    fn fewest_steps(&self) -> Option<u32> {
        let graph = self.as_graph();
        let manhattan = |test: Coords| test.x.abs_diff(self.start.x) + test.y.abs_diff(self.start.y);
        petgraph::algo::astar(&graph, self.start, |x| x == self.end, |e| *e.weight(), manhattan).map(|(steps, _)| steps)
    }

    fn best_trail_length(&self) -> Option<u32> {
        let graph = self.as_rev_graph();

        // Do Dijkstra to all tiles, then pick the fewest steps that lead to 'a'
        let trails = petgraph::algo::dijkstra(&graph, self.end, None, |_| 1);
        trails.iter()
            .filter_map(|(coords, steps)| if self.get(coords) == 'a' as u32 { Some(*steps) } else { None })
            .min()
    }
    
}

fn main() {
    let raw = include_str!("../input.txt");
    let grid = Grid::from(raw);
    println!("A: {:?}", grid.fewest_steps());
    println!("B: {:?}", grid.best_trail_length());
}
