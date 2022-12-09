use std::collections::HashSet;

/// Read moves from the input file into direction and step count pairs.
fn read_moves(filename: &str) -> Vec<(char, u32)> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let direction = split.next().unwrap();
            let steps = split.next().unwrap();

            (direction.parse().unwrap(), steps.parse().unwrap())
        })
        .collect()
}

/// Return next position of head based on the direction and
/// current position.
fn move_head(direction: &char, (y, x): &(i32, i32)) -> (i32, i32) {
    match direction {
        'U' => (y + 1, *x),
        'D' => (y - 1, *x),
        'L' => (*y, x - 1),
        'R' => (*y, x + 1),
        _ => {
            panic!("Invalid direction!");
        }
    }
}

/// Count the tail steps for a given set of moves and a given tail
/// length.
/// Create a set of visited positions and insert the position of the
/// tail at each step increment.
/// Keep a vector of tail knot positions for each knot in the tail.
/// Go through the knots and update the position based on the knot
/// that preceeded.
fn count_tail_steps(moves: &[(char, u32)], tail_length: usize) -> usize {
    let mut set = HashSet::new();
    set.insert((0, 0));
    let mut tail = Vec::from_iter((0..tail_length).map(|_| (0, 0)));

    moves.iter().for_each(|(direction, steps)| {
        for _ in 0..*steps {
            // Get the head.
            let head = tail.get_mut(0).unwrap();

            // Move head.
            *head = move_head(direction, &head);

            // Save current knot.
            let mut current_knot = head.clone();

            // Iterate through the tail to update positions. Skip the head.
            for knot in tail.iter_mut().skip(1) {
                // Calculate the distance between two knots.
                let diff_x = knot.1.abs_diff(current_knot.1);
                let diff_y = knot.0.abs_diff(current_knot.0);

                // Skip position update if the knots are still touching.
                if diff_x + diff_y < 2 || (diff_x == 1 && diff_y == 1) {
                    break;
                }

                // Update both coordinates if the knots are diagonally separated.
                // Otherwise update only one coordinate.
                if diff_x + diff_y > 2 {
                    knot.1 += if knot.1 < current_knot.1 { 1 } else { -1 };
                    knot.0 += if knot.0 < current_knot.0 { 1 } else { -1 };
                } else if diff_x > 1 {
                    knot.1 += if knot.1 < current_knot.1 { 1 } else { -1 };
                } else {
                    knot.0 += if knot.0 < current_knot.0 { 1 } else { -1 };
                }

                current_knot = knot.clone();
            }

            set.insert(*tail.last().unwrap());
        }
    });

    set.len()
}

fn main() {
    // Get the moves list from the input file.
    let moves = read_moves("input.txt");

    // Count the steps for a two knot rope.
    let steps_count = count_tail_steps(&moves, 2);

    // Count the steps for a ten knot rope.
    let steps_tail_count = count_tail_steps(&moves, 10);

    println!("{steps_count}");
    println!("{steps_tail_count}");
}
