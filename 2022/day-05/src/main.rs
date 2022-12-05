/// Read the stacks and steps strings from the input file.
fn get_stacks_and_steps(filename: &str) -> (String, String) {
    let stacks_and_steps = std::fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|str| str.to_string())
        .collect::<Vec<_>>();

    (
        stacks_and_steps.get(0).unwrap().to_owned(),
        stacks_and_steps.get(1).unwrap().to_owned(),
    )
}

/// Read an individual step from a line.
/// Separate the line into words and only use the words which
/// parse into numbers.
fn read_step(step: &str) -> (u32, u32, u32) {
    let numbers = step
        .split(" ")
        .filter_map(|word| match word.parse::<u32>() {
            Ok(number) => Some(number),
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    (
        numbers.get(0).unwrap().to_owned(),
        numbers.get(1).unwrap().to_owned() - 1,
        numbers.get(2).unwrap().to_owned() - 1,
    )
}

/// Count the number of stacks based on the numbering line of
/// the stacks text. Count the number of words which can be
/// parsed into numbers.
fn count_stacks(stacks: &str) -> usize {
    stacks
        .split(" ")
        .filter(|char| char.parse::<u32>().is_ok())
        .count()
}

/// Read the stacks from the stacks text into a vector of
/// vectors of characters.
/// Find out the number of stacks in the text.
/// Then, find out the number of levels in the stacks.
/// After this, iterate through each stack level using the
/// following strategy:
///   1. Collect all the characters into a vector of characters;
///   2. Enumerate all the characters of the iterator;
///   3. Iterate using steps of 4 - so skip 3 characters each iteration;
///   4. Enumerate the steps - so the number of times we skip;
///   5. Check if the character we land on is an opening bracket and if it
///      is add the following character to the stack at the index of the
///      current skip.
fn read_stacks(stacks_str: &str) -> Vec<Vec<char>> {
    let number_of_stacks = count_stacks(&stacks_str.lines().last().unwrap());

    let mut stack_levels = stacks_str
        .lines()
        .take(stacks_str.lines().count() - 1)
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    stack_levels.reverse();

    let mut stacks = (0..number_of_stacks)
        .map(|_| Vec::<char>::with_capacity(stack_levels.len()))
        .collect::<Vec<_>>();

    stack_levels.iter().for_each(|stack_level| {
        let chars = stack_level.chars().collect::<Vec<_>>();

        chars.iter().enumerate().step_by(4).enumerate().for_each(
            |(stack_index, (character_index, bracket))| {
                if bracket == &'[' {
                    let stack = stacks.get_mut(stack_index).unwrap();

                    stack.push(chars.get(character_index + 1).unwrap().to_owned());
                }
            },
        );
    });

    stacks
}

/// Perform one step in the crane movement by iterating `crates` number of times
/// to pop one crate from the stack at index `from_stack` and pushing it on
/// top of the stack at index `to_stack`.
fn perform_step(stacks: &mut Vec<Vec<char>>, &(crates, from_stack, to_stack): &(u32, u32, u32)) {
    for _ in 0..crates {
        let from_stack = stacks.get_mut(from_stack as usize).unwrap();
        let crate_to_transfer = from_stack.pop().unwrap();

        let to_stack = stacks.get_mut(to_stack as usize).unwrap();

        to_stack.push(crate_to_transfer);
    }
}

/// Perform one step in the crane movement of the 9001 crane model by
/// collecting `crates` number of crates from the stack at index
/// `from_stack` and extending the stack at the index `to_stack` with
/// those crates.
fn perform_step_v2(stacks: &mut Vec<Vec<char>>, &(crates, from_stack, to_stack): &(u32, u32, u32)) {
    let from_stack = stacks.get_mut(from_stack as usize).unwrap();
    let mut crates_to_transfer = Vec::with_capacity(crates as usize);

    for _ in 0..crates {
        crates_to_transfer.insert(0, from_stack.pop().unwrap());
    }

    let to_stack = stacks.get_mut(to_stack as usize).unwrap();

    to_stack.extend_from_slice(&crates_to_transfer);
}

/// Collect the top crates from each stack into a String.
fn get_top_crates(stacks: &[Vec<char>]) -> String {
    String::from_iter(stacks.iter().map(|stack| stack.last().unwrap()))
}

fn main() {
    // Get stack and steps strings.
    let (stacks_str, steps_str) = get_stacks_and_steps("input.txt");

    // Get the stacks from the stacks string.
    let mut stacks = read_stacks(&stacks_str);
    // Clone the stacks to use in part 2.
    let mut stacks_v2 = stacks.clone();

    // Get the steps from the steps string.
    let steps = steps_str.lines().map(read_step).collect::<Vec<_>>();

    // Perform the steps for part 1.
    steps
        .iter()
        .for_each(|step| perform_step(&mut stacks, step));

    // Collect the top crates.
    let top_crates = get_top_crates(&stacks);

    // Perform the steps for part 2.
    steps
        .iter()
        .for_each(|step| perform_step_v2(&mut stacks_v2, step));

    // Collect the top crates.
    let top_crates_v2 = get_top_crates(&stacks_v2);

    println!("{top_crates}");
    println!("{top_crates_v2}");
}
