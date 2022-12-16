use std::{collections::BTreeMap, ops::RangeInclusive};

/// A struct that represents a point on the map. It could be a sensor, a beacon or a point which a
/// sensor covers, in this case named Nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Sensor,
    Beacon,
    Nothing,
}

/// Read the coordinates from a string in the one of the following formats:
/// Sensor at x=20, y=1
/// closest beacon is at x=15, y=3
fn read_coords(instruction: &str) -> (i32, i32) {
    let from_x = instruction
        .get(instruction.find("x=").unwrap() + 2..)
        .unwrap();
    let split = from_x.split(",").collect::<Vec<_>>();
    let x = split.first().unwrap().parse().unwrap();
    let y = split.last().unwrap().get(3..).unwrap().parse().unwrap();

    (x, y)
}

/// Get the range of coordinates a sensor would cover at a target y based on the beacon closest to
/// the sensor.
fn get_empty_coords(sensor: &(i32, i32), beacon: &(i32, i32), target_y: i32) -> Option<(i32, i32)> {
    let distance = (sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)) as i32;

    let y_distance = sensor.1.abs_diff(target_y) as i32;

    if y_distance > distance {
        None
    } else {
        Some((
            sensor.0 - (distance - y_distance),
            sensor.0 + (distance - y_distance),
        ))
    }
}

/// Read the instructions from the input file for a target y into a BTreeMap.
fn read_map(filename: &str, target_y: i32) -> BTreeMap<i32, Item> {
    let mut map = BTreeMap::<i32, Item>::new();

    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .for_each(|line| {
            let split = line.split(":").collect::<Vec<_>>();
            let sensor = read_coords(split.first().unwrap());
            let beacon = read_coords(split.last().unwrap());

            if let Some((min, max)) = get_empty_coords(&sensor, &beacon, target_y) {
                map.extend((min..=max).map(|x| (x, Item::Nothing)));
            }

            if sensor.1 == target_y {
                map.insert(sensor.0, Item::Sensor);
            }

            if beacon.1 == target_y {
                map.insert(beacon.0, Item::Beacon);
            }
        });

    map
}

/// Read all of the sensor coverages from the input file into a vector of vector ranges.
/// Sort the vector ranges based on the y they cover and the start of the range for x.
/// Fold the ranges into a vector of vectors disregarding the y coordinate and filtering any which
/// would belong to the y coordinates outside the range 0 to and including T.
fn read_full_map<const T: i32>(filename: &str) -> Vec<Vec<RangeInclusive<i32>>> {
    // Get all the ranges for each sensor.
    let mut ranges = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .flat_map(|line| {
            let split = line.split(":").collect::<Vec<_>>();
            let sensor = read_coords(split.first().unwrap());
            let (x, y) = read_coords(split.last().unwrap());

            // Calculate the sensor area distance based on closes beacon.
            let distance = (sensor.0 - x).abs() + (sensor.1 - y).abs();

            // Calcluate the sensors areas for each y.
            ((sensor.1 - distance)..=(sensor.1 + distance))
                .map(|y| {
                    (
                        y,
                        (sensor.0 - (distance - (sensor.1 - y).abs()))
                            ..=(sensor.0 + (distance - (sensor.1 - y).abs())),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Sort the ranges.
    ranges.sort_by(|left, right| {
        let cmp = left.0.cmp(&right.0);

        if cmp == std::cmp::Ordering::Equal {
            left.1.start().cmp(right.1.start())
        } else {
            cmp
        }
    });

    // Merge all ranges that are on the same y.
    ranges
        .iter()
        .fold(
            Vec::<(i32, Vec<RangeInclusive<i32>>)>::new(),
            |mut acc, curr| {
                if acc.is_empty() {
                    acc.push((curr.0, vec![curr.1.clone()]));
                    acc
                } else {
                    match acc.last().unwrap().0.cmp(&curr.0) {
                        std::cmp::Ordering::Less => {
                            acc.push((curr.0, vec![curr.1.clone()]));
                            acc
                        }
                        std::cmp::Ordering::Equal => {
                            let last = acc.last_mut().unwrap();
                            last.1.push(curr.1.clone());
                            acc
                        }
                        std::cmp::Ordering::Greater => acc,
                    }
                }
            },
        )
        .into_iter()
        .filter(|(y, _)| (0..=T).contains(y))
        .map(|(_, ranges)| ranges)
        .collect()
}

fn main() {
    // Specify the target y to check for.
    let target_y = 2_000_000;
    // Read the map from the input file.
    let map = read_map("input.txt", target_y);
    // Count how many sensor fields are covered.
    let count_empty = map
        .iter()
        .filter(|(_, &item)| item == Item::Nothing)
        .count();

    println!("{count_empty}");

    // Specify the end of the range.
    const END: i32 = 4_000_000;

    // Read the map of ranges.
    let full_map = read_full_map::<END>("input.txt");

    // Get the x and y not covered by any sensor.
    let (x, y) = full_map
        .iter()
        .enumerate()
        .find_map(|(y, ranges)| {
            let mut start_range = 0..=0;

            for current_range in ranges {
                if start_range.end() + 1 >= *current_range.start() {
                    start_range =
                        *start_range.start()..=(*current_range.end().max(start_range.end()));
                } else if start_range.end() > &END {
                    return None;
                } else {
                    return Some(((start_range.end() + 1) as usize, y));
                }
            }

            return None;
        })
        .unwrap();

    println!("{x},{y}");
    println!("{}", x * (END as usize) + y);
}
