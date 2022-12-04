/// Parse the range from a string in the following format X-Y.
fn get_range(range_str: &str) -> (u32, u32) {
    let ends = range_str.split("-").collect::<Vec<_>>();
    let lower = ends.get(0).unwrap().parse().unwrap();
    let upper = ends.get(1).unwrap().parse().unwrap();

    (lower, upper)
}

/// Parse the ranges from a string in the following format A-B,X-Y.
fn get_range_pairs(range_pair_str: &str) -> ((u32, u32), (u32, u32)) {
    let ranges = range_pair_str.split(",").collect::<Vec<_>>();
    let first = get_range(ranges.get(0).unwrap());
    let second = get_range(ranges.get(1).unwrap());

    (first, second)
}

/// Check if one range fully containes another.
fn some_fully_contained((first, second): &((u32, u32), (u32, u32))) -> bool {
    if first.0 <= second.0 && first.1 >= second.1 {
        true
    } else if second.0 <= first.0 && second.1 >= first.1 {
        true
    } else {
        false
    }
}

/// Check if two ranges have an intersection.
fn some_overlap((first, second): &((u32, u32), (u32, u32))) -> bool {
    if first.0 <= second.0 && first.1 >= second.0 {
        true
    } else if first.0 <= second.1 && first.1 >= second.1 {
        true
    } else if second.0 <= first.0 && second.1 >= first.0 {
        true
    } else if second.0 <= first.1 && second.1 >= first.1 {
        true
    } else {
        false
    }
}

/// Read lines from input file.
fn read_range_pairs(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn main() {
    // Read range pairs from input file.
    let range_pairs = read_range_pairs("input.txt");

    // Get the count of pairs where one range fully containes another.
    let count_containing = range_pairs
        .iter()
        .map(|range_pair_str| get_range_pairs(&range_pair_str))
        .filter(|range_pair| some_fully_contained(&range_pair))
        .count();

    // Get the count of pairs with an intersection.
    let count_overlapping = range_pairs
        .iter()
        .map(|range_pair_str| get_range_pairs(&range_pair_str))
        .filter(|range_pair| some_overlap(&range_pair))
        .count();

    println!("{count_containing}");
    println!("{count_overlapping}");
}
