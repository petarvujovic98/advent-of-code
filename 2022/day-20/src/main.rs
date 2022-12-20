/// Mix the values of a given slice by moving each element by adding their value to their inedx.
fn mix(coords: &[i64], rounds: u8) -> Vec<i64> {
    // Create a vector that keeps track of the initial index of a value, the actual value, and the
    // number of times it has moved so far.
    let mut new_coords = coords
        .iter()
        .enumerate()
        .map(|(index, value)| (value, 0, index))
        .collect::<Vec<_>>();

    for round in 0..rounds {
        // Keep mixing in the same order as in the starting slice.
        for (index, value) in coords.iter().enumerate() {
            // Find the new index of the next value we want to move.
            let new_index = new_coords
                .iter()
                .enumerate()
                .find_map(|(new_index, (old_value, moved, old_index))| {
                    if old_value == &value && moved == &round && old_index == &index {
                        Some(new_index)
                    } else {
                        None
                    }
                })
                .unwrap();

            // Remove the value from the vector.
            let new_value = new_coords.remove(new_index);
            // Add the value to the index.
            let unbound_next = new_index as i64 + value;
            // Capture new array length.
            let array_len = new_coords.len() as i64;

            // If the new index is negative we find division remainder and add array length to make it a
            // valid index.
            let next_index = if unbound_next < 0 {
                array_len + unbound_next % array_len
            // If the new index is greater than or equal to the array length we find the
            // division remainder.
            } else if unbound_next >= array_len {
                unbound_next % array_len
            // If the new index is the same as the old index, we decrease it by one because we took
            // the element out.
            } else if unbound_next == index as i64 {
                unbound_next - 1
            // Otherwise just use the same index.
            } else {
                unbound_next
            } as usize;

            // If the new index is zero, that means that the value goes at the end.
            if next_index == 0 {
                new_coords.push((new_value.0, round + 1, new_value.2));
            } else {
                new_coords.insert(next_index, (new_value.0, round + 1, new_value.2));
            }
        }
    }

    new_coords.into_iter().map(|(&value, ..)| value).collect()
}

/// Read the input coordinates from the input file into a vector.
fn read_input(filename: &str) -> Vec<i64> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

/// Find the 1_000-th, 2_000-th and 3_000-th values after a zero value in the slice (iterating
/// the slice circularly) and sum those values.
fn get_coords(coords: &[i64]) -> i64 {
    let zero_index = coords
        .iter()
        .enumerate()
        .find_map(|(i, z)| if z == &0 { Some(i) } else { None })
        .unwrap();

    let index_1_000 = (zero_index + 1_000) % coords.len();
    let index_2_000 = (zero_index + 2_000) % coords.len();
    let index_3_000 = (zero_index + 3_000) % coords.len();

    coords[index_1_000] + coords[index_2_000] + coords[index_3_000]
}

fn main() {
    // Get the coordinate encryption from the input file.
    let coords = read_input("input.txt");

    // Mix the coordinates to decrypt them.
    let mixed = mix(&coords, 1);

    // Sum the 1_000-th, 2_000-th and 3_000-th values.
    let sum_coords = get_coords(&mixed);

    println!("{sum_coords:?}");

    // Mix the coordinates ten times, and before that multiply them with the decryption key.
    let new_mixed = mix(
        &coords
            .iter()
            .map(|coord| coord.checked_mul(811_589_153).unwrap())
            .collect::<Vec<_>>(),
        10,
    );

    // Sum the 1_000-th, 2_000-th and 3_000-th values.
    let sum_coords = get_coords(&new_mixed);

    println!("{sum_coords:?}");
}
