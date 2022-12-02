fn count_flashes(energies: &mut Vec<Vec<u32>>) -> usize {
    let height = energies.len();
    let width = energies[0].len();
    let mut flashed_coords = vec![];
    loop {
        let mut over9s = vec![];
        for (row_idx, row) in energies.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell > 9 {
                    over9s.push((col_idx, row_idx));
                }
            }
        }

        if over9s.len() == 0 {
            break;
        }

        for (col_idx, row_idx) in over9s.iter() {
            
            if *row_idx > 0 {
                if *col_idx > 0 {
                    energies[(*row_idx) - 1][*col_idx - 1] += 1;
                }
                energies[(*row_idx) - 1][*col_idx] += 1;
                if *col_idx < width - 1 {
                    energies[(*row_idx) - 1][*col_idx + 1] += 1;
                }
            }

            if *col_idx > 0 {
                energies[(*row_idx)][*col_idx - 1] += 1;
            }
            energies[(*row_idx)][*col_idx] += 1;
            if *col_idx < width - 1 {
                energies[(*row_idx)][*col_idx + 1] += 1;
            }

            if *row_idx < height - 1 {
                if *col_idx > 0 {
                    energies[(*row_idx) + 1][*col_idx - 1] += 1;
                }
                energies[(*row_idx) + 1][*col_idx] += 1;
                if *col_idx < width - 1 {
                    energies[(*row_idx) + 1][*col_idx + 1] += 1;
                }
            }

            energies[*row_idx][*col_idx] = 0;
        }

        flashed_coords.extend(over9s);
    }

    // Reset energies
    for (col_idx, row_idx) in flashed_coords.iter() {
        energies[*row_idx][*col_idx] = 0;
    }

    flashed_coords.len()
}

fn step(energies: &mut Vec<Vec<u32>>) -> usize {
    for row in energies.iter_mut() {
        for cell in row.iter_mut() {
            *cell += 1;
        }
    }
    count_flashes(energies)
}

fn part_a(energies: &Vec<Vec<u32>>) -> usize {
    let mut energies = energies.clone();

    (0..100)
        .map(|_| step(&mut energies))
        .fold(0, |acc, x| acc + x)
}

fn part_b(energies: &Vec<Vec<u32>>) -> usize {
    let area = energies.len() * energies[0].len();
    let mut energies = energies.clone();

    (1..)
        .map(|x| (x, step(&mut energies)))
        .take_while(|x| x.1 < area)
        .last()
        .expect("No last item!")
        .0 + 1
}

fn main() {
    let energies: Vec<Vec<u32>> = include_str!("../data/input.txt")
        .lines()
        .map(|l| l
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        )
        .collect();
    println!("Part A: {}", part_a(&energies));
    println!("Part B: {}", part_b(&energies));
}
