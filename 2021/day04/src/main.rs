mod bingo_board;
use bingo_board::BingoBoard;

fn part_a(boards: &Vec<BingoBoard>, to_call: &Vec<u32>) -> u32 {
    let mut boards = boards.to_vec();
    for call in to_call.iter() {
        for board in boards.iter_mut() {
            board.call(*call);
            if board.is_bingo() {
                return board.calc_result();
            }
        }
    }

    0
}

fn part_b(boards: &Vec<BingoBoard>, to_call: &Vec<u32>) -> u32 {
    let mut boards = boards.to_vec();
    let mut board_ref: Option<BingoBoard> = None;
    for call in to_call.iter() {
        if board_ref.is_some() {
            let board = board_ref.as_mut().unwrap();
            board.call(*call);
            if board.is_bingo() {
                return board.calc_result();
            }
        } else {
            boards = boards
                .iter_mut()
                .map(|board| { board.call(*call); board })
                .filter(|board| !board.is_bingo())
                .map(|b| b.clone())
                .collect();
            if boards.len() == 1 {
                board_ref = Some(boards[0].clone());
            }
        }
    }

    0
}

fn main() {
    let input = include_str!("../data/input.txt");
    let to_call: Vec<u32> = input
        .lines()
        .next().expect("No lines in file!")
        .split(",")
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut boards = vec![];
    let mut board_raw = vec![];
    for line in input.lines().skip(1) {
        if line.len() == 0 {
            continue;
        }
        board_raw.push(line);
        if board_raw.len() == 5 {
            boards.push(BingoBoard::from_raw(&board_raw));
            board_raw.clear();
        }
    }

    println!("Part A: {}", part_a(&boards, &to_call));
    println!("Part B: {}", part_b(&boards, &to_call));
}
