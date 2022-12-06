use std::collections::HashSet;

/// Read the buffer from the input string.
fn get_buffer(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

/// Check to see if sequence of characters is unique -
/// meaning all the characters are different.
fn check_if_unique(marker: &str) -> bool {
    HashSet::<char>::from_iter(marker.chars()).len() == marker.len()
}

/// Find the start of the packet by searching for the first
/// unique 4 character sequence.
fn find_start_of_packet(buffer: &str) -> usize {
    for index in 4..=buffer.len() {
        let marker = buffer.get(index - 4..index).unwrap();

        if check_if_unique(marker) {
            return index;
        }
    }

    buffer.len() - 1
}

/// Find the start of the message by searching for the first
/// unique 14 character sequence.
fn find_start_of_message(buffer: &str) -> usize {
    for index in 14..=buffer.len() {
        let marker = buffer.get(index - 14..index).unwrap();

        if check_if_unique(marker) {
            return index;
        }
    }

    buffer.len() - 1
}

fn main() {
    // Get the buffer from the input file.
    let buffer = get_buffer("input.txt");

    // Find the start of the packet.
    let start_of_packet = find_start_of_packet(&buffer);

    // Find the start of the message.
    let start_of_message = find_start_of_message(&buffer);

    println!("{start_of_packet}");
    println!("{start_of_message}");
}
