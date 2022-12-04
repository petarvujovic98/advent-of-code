use std::collections::HashSet;

/// Create a sum of sets.
fn add_sets<'a, T>(first: &'a HashSet<T>, second: &'a HashSet<T>) -> HashSet<&'a T>
where
    T: Eq,
    T: core::hash::Hash,
{
    HashSet::<&T>::from_iter(first.iter().chain(second.iter()))
}

/// Read input file into string and iterate through the lines of input.
/// Map each line into two hash sets made from halfs of the string at
/// that line.
fn get_rucksack_compartments(filename: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let (first_compartment, second_compartment) = line.split_at(line.len() / 2);

            (
                HashSet::from_iter(first_compartment.chars()),
                HashSet::from_iter(second_compartment.chars()),
            )
        })
        .collect()
}

/// Find the common item between two hash sets.
fn find_common_item(
    (first_compartment, second_compartment): &(HashSet<char>, HashSet<char>),
) -> char {
    let intersection = first_compartment
        .intersection(&second_compartment)
        .next()
        .unwrap();

    intersection.to_owned()
}

/// Iterate through rucksacks 3 rucksacks at a time. Create hash sets
/// from each rucksack and find the intersection between the 3 different
/// sets and return it into a vector of characters.
fn get_elf_groups(rucksacks: &[(HashSet<char>, HashSet<char>)]) -> Vec<char> {
    rucksacks
        .iter()
        .enumerate()
        .step_by(3)
        .map(|(index, (left, right))| {
            let first = add_sets(left, right);
            let second = rucksacks.get(index + 1).unwrap();
            let second = add_sets(&second.0, &second.1);
            let third = rucksacks.get(index + 2).unwrap();
            let third = add_sets(&third.0, &third.1);

            let intersection = first
                .intersection(&second)
                .map(|char| char.to_owned())
                .collect::<HashSet<&char>>();

            let intersection = intersection.intersection(&third).next().unwrap();

            intersection.to_owned().to_owned()
        })
        .collect()
}

/// Calculate priority based on the character passed to the function.
fn get_priority(item: &char) -> usize {
    let mut range = ('a'..='z').collect::<String>();
    range.extend('A'..='Z');

    range.find(|char| &char == item).unwrap() + 1
}

fn main() {
    // Get the rucksacks from the input file.
    let rucksacks = get_rucksack_compartments("input.txt");

    // Calculate the sum of priorities of the missplaced items in each rucksack.
    let sum_of_priorites: usize = rucksacks
        .iter()
        .map(|rucksack| get_priority(&find_common_item(rucksack)))
        .sum();

    // Calculate the sum of priorities of the group badges for each 3-elf group.
    let sum_of_groups: usize = get_elf_groups(&rucksacks).iter().map(get_priority).sum();

    println!("{sum_of_priorites}");
    println!("{sum_of_groups}");
}
