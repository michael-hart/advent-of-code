#[derive(Clone, Debug)]
struct BingoCell {
    value: u32,
    called: bool,
}

#[derive(Clone, Debug)]
pub struct BingoBoard {
    board: Vec<Vec<BingoCell>>,
    last_called: u32,
}

impl BingoBoard {
    pub fn from_raw(input: &Vec<&str>) -> Self {
        let mut board = vec![];
        for row_raw in input {
            let row = row_raw
                .split(" ")
                .filter_map(|x| x.parse().ok())
                .map(|x| BingoCell { value: x, called: false })
                .collect();
            board.push(row);
        }
        BingoBoard { board, last_called: 0 }
    }

    pub fn call(&mut self, num: u32) {
        for row in self.board.iter_mut() {
            for cell in row.iter_mut() {
                if cell.value == num {
                    cell.called = true;
                    self.last_called = num;
                    break;
                }
            }
        }
    }

    pub fn is_bingo(&self) -> bool {
        // Check rows
        for row in &self.board {
            let mut full = true;
            for cell in row {
                if !cell.called {
                    full = false;
                    break;
                }
            }
            if full {
                return true;
            }
        }

        // Check columns
        for i in 0..self.board.first().unwrap().len() {
            let mut full = true;
            for row in &self.board {
                let cell = row.get(i).unwrap();
                if !cell.called {
                    full = false;
                    break;
                }
            }
            if full {
                return true;
            }
        }

        false
    }

    pub fn calc_result(&self) -> u32 {
        let mut result = 0;
        for row in &self.board {
            for cell in row {
                if !cell.called {
                    result += cell.value;
                }
            }
        }

        println!("Result: {}; last_called: {}", result, self.last_called);

        result * self.last_called
    }
}
