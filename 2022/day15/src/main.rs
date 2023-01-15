use std::{collections::HashSet, hash::Hash};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

const X_MAX: i32 = 4000000;
const Y_MAX: i32 = 4000000;

const REPORT_STR: &str =
    "Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)";
lazy_static! {
    static ref RE: Regex = Regex::new(REPORT_STR).expect("Could not compile regex!");
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Self {
        Coords { x, y }
    }

    fn manhattan_to(&self, other: &Coords) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SensorSet {
    sensor: Coords,
    beacon: Coords,
    distance: u32,
}

impl From<&str> for SensorSet {
    fn from(text: &str) -> Self {
        let caps = RE
            .captures(text)
            .expect("Input text does not match expected format!");

        let parse_int = |cap: &Captures, idx| {
            cap.get(idx)
                .map_or("", |m| m.as_str())
                .parse::<i32>()
                .unwrap()
        };

        let s_x = parse_int(&caps, 1);
        let s_y = parse_int(&caps, 2);
        let b_x = parse_int(&caps, 3);
        let b_y = parse_int(&caps, 4);
        let sensor = Coords::new(s_x, s_y);
        let beacon = Coords::new(b_x, b_y);
        let distance = sensor.manhattan_to(&beacon);
        Self {
            sensor,
            beacon,
            distance,
        }
    }
}

struct SensorSpace {
    sets: HashSet<SensorSet>,
}

impl From<&str> for SensorSpace {
    fn from(text: &str) -> Self {
        let sets = text.lines().map(SensorSet::from).collect();
        SensorSpace { sets }
    }
}

impl SensorSpace {
    fn unreachable_on_given_line(&self, y_set: i32) -> usize {
        // Find min/max x from beacons and sensors
        let mut x_vals: Vec<i32> = self.sets.iter().map(|set| set.beacon.x).collect();
        x_vals.extend(self.sets.iter().map(|set| set.sensor.x - set.distance as i32));
        x_vals.extend(self.sets.iter().map(|set| set.sensor.x + set.distance as i32));
        let x_vals = x_vals;

        let x_min = *x_vals.iter().min().unwrap();
        let x_max = *x_vals.iter().max().unwrap();

        let mut count = 0;
        'outer: for x in x_min..=x_max {
            // Iterate through all sensor sets, and if there's a closer beacon in manhattan distance, this is unavailable
            let coords = Coords::new(x, y_set);
            for set in self.sets.iter() {
                if coords.manhattan_to(&set.sensor) <= set.distance && coords != set.beacon && coords != set.sensor {
                    count += 1;
                    continue 'outer;
                }
            }
        }
        count
    }

    fn tuning_frequency(&self) -> Option<usize> {
        for set in self.sets.iter() {
            for x in 0..=(set.distance as i32 + 1) {
                let y = set.distance as i32 - x + 1;
                let candidates: HashSet<Coords> = {
                    let mut cands = HashSet::new();
                    cands.insert(Coords::new(set.sensor.x + x, set.sensor.y + y));
                    cands.insert(Coords::new(set.sensor.x + x, set.sensor.y - y));
                    cands.insert(Coords::new(set.sensor.x - x, set.sensor.y + y));
                    cands.insert(Coords::new(set.sensor.x - x, set.sensor.y - y));
                    cands
                };
                'outer: for candidate in candidates {
                    if candidate.x < 0 || candidate.x > X_MAX || candidate.y < 0 || candidate.y > Y_MAX {
                        continue;
                    }
                    for test_set in self.sets.iter() {
                        // Is this too close to another sensor to be a real choice?
                        if candidate.manhattan_to(&test_set.sensor) <= test_set.distance && candidate != test_set.beacon {
                            continue 'outer;
                        }
                    }

                    let freq = (candidate.x as usize * 4000000) + candidate.y as usize;
                    return Some(freq);
                }
            }
        };

        None
    }
}

fn main() {
    // This is NOT very fast. I'm sure there's a faster way. Still, if you run as release build,
    // it only takes a few seconds.
    let raw = include_str!("../input.txt");
    let sensor_space = SensorSpace::from(raw);
    println!("A: {}", sensor_space.unreachable_on_given_line(2e6 as i32));
    println!("B: {:?}", sensor_space.tuning_frequency());
}
