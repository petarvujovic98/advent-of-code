/// Read the instructions from the input file and
/// calculate the value of the `X` register for each cycle.
fn get_cycles(filename: &str) -> Vec<i32> {
    let mut cycles = vec![1];

    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .for_each(|line| {
            let x = cycles.last().unwrap().clone();
            cycles.push(x);

            if line.starts_with("addx") {
                let v = line.get(5..).unwrap().parse::<i32>().unwrap();
                cycles.push(x + v);
            }
        });

    cycles
}

fn main() {
    // Get the cycles from the input file.
    let cycles = get_cycles("input.txt");

    // Calculate the sum of the products of the cycle number and `X` register
    // value at each 40 cycles starting from the 20th cycle.
    let sum = cycles
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(cycle, x)| (cycle + 1) as i32 * x)
        .sum::<i32>();

    // Print the CRT screen into a String by iterating over each cycle.
    let crt_screen =
        cycles
            .iter()
            .enumerate()
            .skip(1)
            .fold("".to_string(), |screen, (cycle, x)| {
                // Check to see if the middle pixel of the sprite is visible at current position.
                let pixel = if (cycle as i32 % 40).abs_diff(*x) < 2 {
                    "#"
                } else {
                    "."
                };

                // Add new line if the current cycle is the last cycle in the row.
                let new_line = if cycle % 40 == 0 { "\n" } else { "" };

                format!("{screen}{pixel}{new_line}")
            });

    println!("{sum}");
    println!("{crt_screen}");
}
