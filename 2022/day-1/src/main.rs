use std::fs;

/// Get the sum of calories for each of the elfs in the input file.
/// First split into strings by the empty line which separates elf entries.
/// Then do some string cleanup to remove accidental double empty lines or whitespace characters.
/// Then fold those lines into sums of calories by parsing each line as an unsigned 32 bit integer.
/// Finally sort and reverse the vector.
fn get_elf_calories(filename: &str) -> Vec<u32> {
    let mut calories_per_elf = fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .filter_map(|lines| {
            let lines = lines.trim().to_string();

            if lines.is_empty() {
                None
            } else {
                Some(lines)
            }
        })
        .map(|lines_of_elf| {
            lines_of_elf
                .lines()
                .fold(0u32, |sum, line| sum + line.parse::<u32>().unwrap())
        })
        .collect::<Vec<u32>>();

    calories_per_elf.sort();
    calories_per_elf.reverse();

    calories_per_elf
}

fn main() {
    // Get the calories for each elf.
    let elf_calories = get_elf_calories("./input.txt");

    // Get the max calories of any elf.
    let max_calories = elf_calories.get(0).unwrap();

    // Get the sum of the top three elfs.
    let sum_of_top_three: u32 = elf_calories.get(0..3).unwrap().iter().sum();

    println!("{max_calories}");
    println!("{sum_of_top_three}");
}
