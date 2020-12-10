fn solve(raw: &str) -> u64 {
    let mut nums: Vec<u32> = raw.lines().map(|l| l.parse().unwrap()).collect();
    nums.sort();
    nums.insert(0, 0);
    let diffs: Vec<u32> = nums.iter().zip(nums.iter().skip(1)).map(|(lower, higher)| higher - lower).collect();
    let ones = diffs.iter().filter(|d| **d == 1).count() as u64;
    let threes = diffs.iter().filter(|d| **d == 3).count() as u64 + 1;
    ones * threes
}

fn main() {
    println!("Soln is {}", solve(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(include_str!("../test_small.txt")), 5*7);
        assert_eq!(solve(include_str!("../test_large.txt")), 22*10);
    }
}
