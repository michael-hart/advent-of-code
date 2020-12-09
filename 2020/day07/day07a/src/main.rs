use std::collections::{HashMap, HashSet};
use regex::Regex;

type BagMap = HashMap<String, Vec<BagQuantity>>;

#[derive(Debug)]
struct BagQuantity {
    name: String,
    quant: u32,
}

impl BagQuantity {
    fn new(name: &str, quant: u32) -> BagQuantity {
        BagQuantity { name: String::from(name), quant }
    }
}

fn solve(map: &BagMap, desired: String) -> usize {
    let mut definites = HashSet::new();
    let mut candidates = vec![desired];

    while candidates.len() > 0 {
        let mut next_candidates = vec![];
        for candidate in candidates {
            // Find all entries in BagMap that contains candidate
            'outer: for (bag, bag_list) in map.iter() {

                for inner_bag in bag_list {
                    if candidate == inner_bag.name {
                        next_candidates.push(bag.clone());
                        continue 'outer;
                    }
                }
            }
        }
        definites.extend(next_candidates.clone());
        candidates = next_candidates;
    }

    definites.len()
}

fn map_of(raw: &str) -> BagMap {
    let mut map = HashMap::new();
    let total_re = Regex::new(r"(.*) bags contain (.*) bags*.").unwrap();
    let bag_re = Regex::new(r"(\d+) (\w*\s\w*)").unwrap();
    for line in raw.lines() {
        let groups = match total_re.captures(line) {
            Some(caps) => caps,
            None => continue,
        };
        let key = groups.get(1).unwrap();
        let val = groups.get(2).unwrap();
        if val.as_str() == "no other" {
            map.insert(String::from(key.as_str()), vec![]);
        } else {
            let mut bags = vec![];
            for sub in bag_re.find_iter(val.as_str()) {
                let bag_groups = match bag_re.captures(sub.as_str()) {
                    Some(caps) => caps,
                    None => continue,
                };
                let n = bag_groups.get(1).unwrap().as_str().parse().unwrap();
                let desc = bag_groups.get(2).unwrap().as_str();
                bags.push(BagQuantity::new(desc, n));
            }

            map.insert(String::from(key.as_str()), bags);
        }
    }

    map
}

fn main() {
    println!("Number of ways is {}", solve(&map_of(include_str!("../input.txt")), String::from("shiny gold")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn test_build_small_map() {
        let result = map_of(include_str!("../test_small.txt"));
        let mut expected = HashMap::new();
        expected.insert(
            String::from("vibrant plum"),
            vec![
                BagQuantity::new("faded blue", 5),
                BagQuantity::new("dotted black", 6)]);
        expected.insert(String::from("faded blue"), vec![]);
        expected.insert(String::from("dotted black"), vec![]);

        println!("{:?}", result);

        assert!(result.contains_key(&String::from("vibrant plum")));
        assert_eq!(result.get(&String::from("vibrant plum")).unwrap().len(), 2);
        assert_eq!(result.get(&String::from("vibrant plum")).unwrap()[0].name, "faded blue");
        assert_eq!(result.get(&String::from("vibrant plum")).unwrap()[0].quant, 5);

        assert_eq!(result.get(&String::from("vibrant plum")).unwrap()[1].name, "dotted black");
        assert_eq!(result.get(&String::from("vibrant plum")).unwrap()[1].quant, 6);
        
        assert!(result.contains_key(&String::from("faded blue")));
        assert_eq!(result.get(&String::from("faded blue")).unwrap().len(), 0);
        assert!(result.contains_key(&String::from("dotted black")));
        assert_eq!(result.get(&String::from("dotted black")).unwrap().len(), 0);
    }

    #[test]
    fn test_solve_large() {
        let map = map_of(include_str!("../test.txt"));
        assert_eq!(solve(&map, String::from("shiny gold")), 4);
    }
}
