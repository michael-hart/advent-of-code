fn inner_solve(progress: &Vec<u64>, remaining: &Vec<u64>) -> u64 {

    if remaining.len() == 0 {
        return 1;
    }

    let current = *progress.last().unwrap_or(&0);

    remaining
        .iter()
        .take_while(|r| **r - current <= 3)
        .enumerate()
        .map(|(idx, r)| {
            let mut next_progress = progress.clone();
            next_progress.push(*r);
            inner_solve(&next_progress, &remaining.iter().skip(idx + 1).map(|c| *c).collect())
        })
        .sum::<u64>()
}

fn solve_chunk(chunk: &Vec<u64>) -> u64 {
    inner_solve(&vec![*chunk.first().unwrap()], &chunk.iter().skip(1).map(|x| *x).collect())
}

fn solve(raw: &str) -> u64 {
    let mut nums: Vec<u64> = raw.lines().map(|l| l.parse().unwrap()).collect();
    nums.sort();
    nums.insert(0, 0);
    nums.push(nums.last().unwrap() + 3);

    let mut chunks = vec![];
    let mut chunk = vec![];
    for (idx, hi_val) in nums.iter().skip(1).enumerate() {
        let lo_val = nums.get(idx).unwrap();
        chunk.push(*lo_val);
        if hi_val - lo_val == 3 {
            if chunk.len() > 2 {
                chunks.push(chunk);
            }
            chunk = vec![];
        }
    }
    println!("{:?}", chunks);

    chunks
        .iter()
        .map(|chunk| solve_chunk(chunk))
        .fold(1, |acc, x| acc * x)
}

fn main() {
    println!("Soln is {}", solve(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(include_str!("../test_small.txt")), 8);
        assert_eq!(solve(include_str!("../test_large.txt")), 19208);
    }
}
