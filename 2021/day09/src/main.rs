use std::collections::HashSet;

fn part_a(grid: &Vec<Vec<u32>>) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut sum = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            let mut all_equals = true;
            if row_idx > 0 {
                let cand = grid[row_idx - 1][col_idx];
                if cand < *cell {
                    continue;
                } else if cand > *cell {
                    all_equals = false;
                }
            }
            if row_idx < height - 1 {
                let cand = grid[row_idx + 1][col_idx];
                if cand < *cell {
                    continue;
                } else if cand > *cell {
                    all_equals = false;
                }
            }
            if col_idx > 0 {
                let cand = grid[row_idx][col_idx - 1];
                if cand < *cell {
                    continue;
                } else if cand > *cell {
                    all_equals = false;
                }
            }
            if col_idx < width - 1 {
                let cand = grid[row_idx][col_idx + 1];
                if cand < *cell {
                    continue;
                } else if cand > *cell {
                    all_equals = false;
                }
            }
            // Minimum must be LOWER than at least one surrounding point
            if all_equals {
                continue;
            }
            sum += cell + 1;
        }
    }

    sum
}

fn basin_num(coords: &(usize, usize), basins: &Vec<HashSet<(usize, usize)>>) -> Option<usize> {
    for (idx, basin) in basins.iter().enumerate() {
        if basin.contains(coords) {
            return Some(idx);
        }
    }
    None
}

fn part_b(grid: &Vec<Vec<u32>>) -> u32 {
    let mut basins = vec![];
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell == 9 {
                continue;
            }

            let mut basin_candidates = vec![];
            // Check if above entry is in a basin
            if row_idx > 0 {
                if let Some(up_basin) = basin_num(&(col_idx, row_idx - 1), &basins) {
                    basin_candidates.push(up_basin);
                }
            }
            // Check if left entry is in a basin
            if col_idx > 0 {
                if let Some(left_basin) = basin_num(&(col_idx - 1, row_idx), &basins) {
                    if !basin_candidates.contains(&left_basin) {
                        basin_candidates.push(left_basin)
                    }
                }
            }

            let n_basins = basin_candidates.len();
            if n_basins == 0 {
                // If no entries, create a new basin and add it
                let mut next_basin = HashSet::new();
                next_basin.insert((col_idx, row_idx));
                basins.push(next_basin);
            } else if n_basins == 1 {
                // If one entry, add to that basin
                let basin = basins.get_mut(basin_candidates[0]).expect("No basins available!");
                basin.insert((col_idx, row_idx));
            } else {
                // If two entries, add to basin, merge those two basins, and remove one of them
                let left_idx = basin_candidates.iter().min_by(|a, b| a.cmp(b)).expect("Could not get min!");
                let right_idx = basin_candidates.iter().max_by(|a, b| a.cmp(b)).expect("Could not get max!");

                let (left_basins, right_basins) = basins.split_at_mut(*right_idx);
                let left_basin = &mut left_basins[*left_idx];
                let right_basin = &right_basins[0];

                left_basin.insert((col_idx, row_idx));
                left_basin.extend(right_basin);

                basins.remove(*right_idx);
            }

        }
    }

    // Find result by sorting lengths and multiplying 3 largest together
    let mut lengths: Vec<usize> = basins
        .iter()
        .map(|x| x.len())
        .collect();

    lengths.sort();
    let result = lengths
        .iter()
        .skip(lengths.len() - 3)
        .take(3)
        .fold(1, |acc, x| acc * x);

    result as u32
}

fn main() {
    let raw = include_str!("../data/input.txt");
    let grid = raw
        .lines()
        .map(|l| {
            l
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    println!("Part A: {}", part_a(&grid));
    println!("Part B: {}", part_b(&grid));
}
