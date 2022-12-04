/// Read the input file into a string and iterate through the lines.
/// Map each line to a tuple of two characters representing the round shapes.
fn get_rounds(filename: &str) -> Vec<(char, char)> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|round_line| {
            let chars = round_line.split(" ").collect::<Vec<_>>();
            let get_char = |char: &str| char.chars().collect::<Vec<_>>().get(0).unwrap().to_owned();

            (
                get_char(chars.get(0).unwrap()),
                get_char(chars.get(1).unwrap()),
            )
        })
        .collect()
}

/// Get the round score by comparing the combinations of symbols.
/// For using Rock - 1 point, Paper - 2 points, Scissors - 3 points.
/// For losing - 0 points, drawing - 3 points, winning - 6 points.
/// A - Rock, B - Paper, C - Scissors.
/// X - Rock, Y - Paper, Z - Scissors.
fn calculate_round_score((opponent, you): &(char, char)) -> u32 {
    match opponent {
        'A' => match you {
            'X' => 4,
            'Y' => 8,
            'Z' => 3,
            _ => panic!("Unexpected symbol!"),
        },
        'B' => match you {
            'X' => 1,
            'Y' => 5,
            'Z' => 9,
            _ => panic!("Unexpected symbol!"),
        },
        'C' => match you {
            'X' => 7,
            'Y' => 2,
            'Z' => 6,
            _ => panic!("Unexpected symbol!"),
        },
        _ => panic!("Unexpected symbol!"),
    }
}

/// Get the round score by comparing the combinations of symbols.
/// For using Rock - 1 point, Paper - 2 points, Scissors - 3 points.
/// For losing - 0 points, drawing - 3 points, winning - 6 points.
/// A - Rock, B - Paper, C - Scissors.
/// X - loss, Y - draw , Z - win.
fn calculate_round_score_v2((opponent, you): &(char, char)) -> u32 {
    match opponent {
        'A' => match you {
            'X' => 3,
            'Y' => 4,
            'Z' => 8,
            _ => panic!("Unexpected symbol!"),
        },
        'B' => match you {
            'X' => 1,
            'Y' => 5,
            'Z' => 9,
            _ => panic!("Unexpected symbol!"),
        },
        'C' => match you {
            'X' => 2,
            'Y' => 6,
            'Z' => 7,
            _ => panic!("Unexpected symbol!"),
        },
        _ => panic!("Unexpected symbol!"),
    }
}

fn main() {
    // Get the rounds in a vector.
    let rounds = get_rounds("input.txt");

    // Calculate the total score by mapping over all the rounds and summing the results.
    let total_score = rounds.iter().map(calculate_round_score).sum::<u32>();

    // Calculate the total score by mapping over all the rounds and summing the results.
    let total_score_v2 = rounds.iter().map(calculate_round_score_v2).sum::<u32>();

    println!("{total_score}");
    println!("{total_score_v2}");
}
