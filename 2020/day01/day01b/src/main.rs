#[allow(dead_code)]
fn find_answer(nums: &[u32]) -> Option<u32> {
    for a in nums[..nums.len() - 2].iter() {
        for b in nums[1..nums.len() - 1].iter() {
            for c in nums[2..].iter() {
                if a + b + c == 2020 {
                    return Some(a*b*c);
                }
            }
        }
    }
    None
}

fn main() {
    let input = include_str!("../input.txt");
    let nums : Vec<u32> = input.lines()
        .map(|x| x.trim().parse().unwrap())
        .collect();
    println!("{:?}", find_answer(&nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_list() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_answer(&input).unwrap(), 241861950);
    }
}
