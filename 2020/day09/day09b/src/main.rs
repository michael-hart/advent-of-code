use itertools::Itertools;

fn invalid_number(nums: &Vec<u64>, preamble_len: usize) -> Option<u64> {
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

fn solve(raw: &str, preamble_len: usize) -> Option<u64> {
    let nums: Vec<u64> = raw.lines().map(|l| l.parse().unwrap()).collect();
    let invalid = match invalid_number(&nums, preamble_len) {
        Some(x) => x,
        None => return None,
    };

    'outer: for (idx, val) in nums.iter().take(nums.len() - 1).enumerate() {
        let mut sum_nums = vec![val, nums.get(idx + 1).unwrap()];
        let mut acc: u64 = val + nums.get(idx + 1).unwrap();
        let mut idx_tmp = idx + 2;
        while acc < invalid {
            if (idx_tmp >= nums.len()) {
                continue 'outer;
            }
            let tmp_val = nums.get(idx_tmp).unwrap();
            acc += tmp_val;
            sum_nums.push(tmp_val);
            idx_tmp += 1;
        }
        if acc == invalid {
            return Some(**sum_nums.iter().min().unwrap() + **sum_nums.iter().max().unwrap());
        }
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
        assert_eq!(solve(include_str!("../test.txt"), 5), Some(62));
    }
}
