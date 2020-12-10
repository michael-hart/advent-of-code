use itertools::Itertools;

fn solve(raw: &str, preamble_len: usize) -> Option<u64> {
    let nums: Vec<u64> = raw.lines().map(|l| l.parse().unwrap()).collect();
    
    for (idx, val) in nums.iter().skip(preamble_len).enumerate() {
        let possibilities : Vec<u64> = nums
            .iter()
            .skip(idx)
            .take(preamble_len)
            .combinations(2)
            .map(|v| v.into_iter().sum())
            .collect();
        let contained = possibilities.contains(val);
        if !contained { return Some(*val); }
    }

    None
} 

fn main() {
    println!("Soln is {:?}", solve(include_str!("../input.txt"), 25));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_solve() {
        assert_eq!(solve(include_str!("../test.txt"), 5), Some(127));
    }
}
