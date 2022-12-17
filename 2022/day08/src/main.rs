use std::ops::Range;

#[derive(Debug)]
struct Forest {
    grid: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl Forest {
    fn get_visible_count(&self) -> usize {
        let count_row =
            |(row_idx, row): (usize, &Vec<u32>)|
            row.iter().enumerate().filter(|(col_idx, _)| self.is_visible(row_idx, *col_idx)).count();
        self.grid.iter().enumerate().map(count_row).sum()
    }

    fn get_max_scenic_score(&self) -> u64 {
        let max_of_row =
            |(row_idx, row): (usize, &Vec<u32>)|
            row.iter().enumerate().map(|(col_idx, _)| self.scenic_score(row_idx, col_idx)).max().unwrap();
        self.grid.iter().enumerate().map(max_of_row).max().unwrap()
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 || row == self.height - 1 || col == self.width - 1 {
            return true;
        }
        let check_val = self.grid[row][col];

        let check_rng_row = |rng: Range<usize>| {
            for cur_row in rng {
                let current_val = self.grid[cur_row][col];
                if current_val >= check_val {
                    return false;
                }
            }
            true
        };
        if check_rng_row(0..row) {
            return true;
        }
        if check_rng_row(row + 1 .. self.height) {
            return true;
        }

        let check_rng_col = |rng: Range<usize>| {
            for cur_col in rng {
                let current_val = self.grid[row][cur_col];
                if current_val >= check_val {
                    return false;
                }
            }
            true
        };

        if check_rng_col(0..col) {
            return true;
        }
        if check_rng_col(col+1 .. self.width) {
            return true;
        }

        false
    }

    fn scenic_score(&self, row: usize, col: usize) -> u64 {
        if row == 0 || col == 0 || row == self.height - 1 || col == self.width - 1 {
            return 0;
        }
        let check_val = self.grid[row][col];

        let mut score_up = row as u64;
        for cur_row in (0..row).rev() {
            if self.grid[cur_row][col] >= check_val {
                score_up = row as u64 - cur_row as u64;
                break;
            }
        };

        let mut score_down = (self.height - row - 1) as u64;
        for cur_row in row + 1 .. self.height {
            if self.grid[cur_row][col] >= check_val {
                score_down = cur_row as u64 - row as u64;
                break;
            }
        };

        let mut score_left = col as u64;
        for cur_col in (0..col).rev() {
            if self.grid[row][cur_col] >= check_val {
                score_left = col as u64 - cur_col as u64;
                break;
            }
        };

        let mut score_right = (self.width - col - 1) as u64;
        for cur_col in col + 1 .. self.width {
            if self.grid[row][cur_col] >= check_val {
                score_right = cur_col as u64 - col as u64;
                break;
            }
        };

        score_up * score_down * score_left * score_right
    }
}

impl From<&str> for Forest {
    fn from(text: &str) -> Self {
        let line_to_row = |line: &str| line.chars().filter_map(|x| x.to_digit(10)).collect::<Vec<u32>>();
        let grid = text
            .lines()
            .map(line_to_row)
            .collect::<Vec<Vec<u32>>>();
        let height = grid.len();
        let width = grid.first().unwrap().len();
        Forest { grid, height, width }
    }
}

fn main() {
    let raw = include_str!("../input.txt");
    let forest = Forest::from(raw);
    // println!("{:?}", forest);
    println!("A: {}", forest.get_visible_count());
    println!("B: {}", forest.get_max_scenic_score());
}
