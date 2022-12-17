use std::collections::{HashMap, HashSet};

/// An enum that represents the direction of air coming from a jet.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    /// Create a direction from the input character.
    pub fn new(character: &char) -> Self {
        match character {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid input. Got: {character}"),
        }
    }
}

/// A enum that represents all of the possible rock shapes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rock {
    HorizontalLine,
    Plus,
    LShape,
    VerticalLine,
    Square,
}

/// A enum that represents whether the rock moved or stopped/got jammed.
enum MoveNext {
    Stopped(HashSet<(u64, u64)>),
    Moved(u64, u64),
}

impl Rock {
    /// Get the coordinates that the rock takes up.
    fn get_coords(&self) -> HashSet<(u64, u64)> {
        match self {
            Rock::HorizontalLine => HashSet::from_iter([(0, 0), (1, 0), (2, 0), (3, 0)]),
            Rock::Plus => HashSet::from_iter([(0, 1), (1, 1), (2, 1), (1, 2), (1, 0)]),
            Rock::LShape => HashSet::from_iter([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
            Rock::VerticalLine => HashSet::from_iter([(0, 0), (0, 1), (0, 2), (0, 3)]),
            Rock::Square => HashSet::from_iter([(0, 0), (1, 0), (0, 1), (1, 1)]),
        }
    }

    /// Get the width of the rock.
    fn width(&self) -> u64 {
        match self {
            Rock::HorizontalLine => 4,
            Rock::Plus => 3,
            Rock::LShape => 3,
            Rock::VerticalLine => 1,
            Rock::Square => 2,
        }
    }

    /// Move the rock with the given chamber, direction and current coordinates.
    fn move_rock(
        &self,
        chamber: &HashSet<(u64, u64)>,
        direction: &Direction,
        coords: &(u64, u64),
    ) -> MoveNext {
        // If on the edge of the chamber stay in place, otherwise move in the given direction.
        let next_x = match direction {
            Direction::Left => {
                if coords.0 == 0 {
                    coords.0
                } else {
                    coords.0 - 1
                }
            }
            Direction::Right => {
                if coords.0 + self.width() == 7 {
                    coords.0
                } else {
                    coords.0 + 1
                }
            }
        };

        // If at the bottom of the chamber stay in place, otherwise move down.
        let next_y = if coords.1 == 0 {
            coords.1
        } else {
            coords.1 - 1
        };

        // Get the coordinates to check for collisions.
        let coords_to_test = self.get_coords();

        // If there is a collision because of the move sideways, don't perform the move.
        let new_x = if next_x != coords.0
            && !coords_to_test
                .iter()
                .any(|(x, y)| chamber.contains(&(next_x + x, coords.1 + y)))
        {
            next_x
        } else {
            coords.0
        };

        // If there is a collision because of the move down, don't move/get jammed/settled.
        let new_y = if next_y != coords.1
            && !coords_to_test
                .iter()
                .any(|(x, y)| chamber.contains(&(new_x + x, next_y + y)))
        {
            next_y
        } else {
            coords.1
        };

        // If the rock didn't move down, return the coordinates of where the rock got jammed.
        // Otherwise return the new location.
        if coords.1 > new_y {
            MoveNext::Moved(new_x, new_y)
        } else {
            MoveNext::Stopped(
                coords_to_test
                    .iter()
                    .map(|(x, y)| (new_x + x, new_y + y))
                    .collect(),
            )
        }
    }
}

/// Read the jet directions from the input file.
fn get_jets(filename: &str) -> Vec<Direction> {
    std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .chars()
        .map(|char| Direction::new(&char))
        .collect()
}

/// Get the height of the rock formation after `number_of_rocks` rocks have settled.
fn get_height(number_of_rocks: u64, directions: &[Direction]) -> u64 {
    // Create a cyclical iterator of directions.
    let mut direction_iter = directions.iter().enumerate().cycle();

    // Create a cyclical iterator of rocks.
    let all_rocks = vec![
        Rock::HorizontalLine,
        Rock::Plus,
        Rock::LShape,
        Rock::VerticalLine,
        Rock::Square,
    ];
    let rocks = all_rocks.iter().cycle().take(number_of_rocks as usize);

    // Create the chamber.
    let mut chamber = HashSet::<(u64, u64)>::new();

    // Keep track of the height of the rock formation.
    let mut height: u64 = 0;

    // Keep track of fully closed chamber compartments.
    let mut floors = HashMap::new();

    // Iterate through all of the rocks.
    for (round, rock) in rocks.enumerate() {
        // Mark the starting possition of the current rock.
        let mut coords = (2, height + 3);

        // Capture the last jet index while moving the rock until it settles.
        let current_jet = loop {
            // Get the next jet direction.
            let (jet, direction) = direction_iter.next().unwrap();

            // Move the rock.
            match rock.move_rock(&chamber, direction, &coords) {
                MoveNext::Moved(x, y) => {
                    coords = (x, y);
                    continue;
                }
                MoveNext::Stopped(blocked) => {
                    height = height.max(*blocked.iter().map(|(_, y)| y).max().unwrap() + 1);
                    chamber.extend(blocked);
                    break jet;
                }
            }
        };

        // Check if there is a closed chamber compartement.
        if (0..7).all(|x| chamber.contains(&(x, height - 2))) {
            // If we already had a compartment at the current rock and jet index combination,
            // Calculate the remainder of the height by using the cycles of the formation.
            if let Some((first_iteration, first_height)) = floors.get(&(rock, current_jet)) {
                let rocks_in_cycle = round - first_iteration;
                let cycle_height = height - 1 - first_height;
                let leftover_rounds = number_of_rocks - *first_iteration as u64;
                let cycles_left = leftover_rounds / rocks_in_cycle as u64;
                let leftover_rocks = leftover_rounds % rocks_in_cycle as u64;

                return cycles_left * cycle_height
                    // There will be rocks leftover from the cycle division as it might not be a
                    // whole number. There will also be rocks from before we entered a cycle so we
                    // also calculate the height for those.
                    + get_height(*first_iteration as u64 + leftover_rocks, directions);
            } else {
                // Otherwise remember the current chambers location.
                floors.insert((rock, current_jet), (round, height - 1));
            }
        }
    }

    height
}

fn main() {
    // Get the jet directions from the input file.
    let jets = get_jets("input.txt");

    // Calculate the height of the rock formation.
    let height = get_height(2022, &jets);

    // Calculate the height of the rock formation.
    let new_height = get_height(1_000_000_000_000, &jets);

    println!("{height}");
    println!("{new_height}");
}
