fn main() {
    let input = include_str!("../input.txt");
    let blocks: Vec<&str> = input.split("\r\n\r\n").collect();
    let groups: Vec<Vec<u32>> = blocks.iter().map(|x| x.split_whitespace().map(|y| y.parse::<u32>().expect("Could not parse int!")).collect::<Vec<u32>>()).collect();
    let mut sums: Vec<u32> = groups.iter().map(|x| x.iter().sum::<u32>()).collect();
    sums.sort();

    let max = sums.iter().skip(sums.len() - 1).next();
    let max_of_3: u32 = sums.iter().skip(sums.len() - 3).sum();

    println!("A: {:?}", max);
    println!("B: {:?}", max_of_3);
}
