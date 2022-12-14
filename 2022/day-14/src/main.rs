use std::collections::HashMap;

/// An enum representing an item that could block sand from falling further.
#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Rock,
    Sand,
}

/// Read the rock locations from the input file line by line and record the locations of the rocks
/// in a sparse matrix, or in our case a HashMap. We also find the height of the cave.
fn get_rock_locations(filename: &str) -> (HashMap<(u16, u16), Item>, u16) {
    let mut map = HashMap::<(u16, u16), Item>::new();
    let mut height = 0;

    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .for_each(|line| {
            // We collect the coordinate pairs into a vector.
            let coords = line
                .split(" -> ")
                .map(|coords| {
                    let coords = coords.split(",").collect::<Vec<_>>();
                    let x = coords.first().unwrap().parse::<u16>().unwrap();
                    let y = coords.last().unwrap().parse::<u16>().unwrap();

                    (x, y)
                })
                .collect::<Vec<_>>();

            // We take the starting coordinates for the firs rock formation.
            let (mut first_x, mut first_y) = coords.first().unwrap().clone();

            // Then we iterate through the rest of the coordinates.
            for (x, y) in coords.iter().skip(1) {
                // If there is a change in the `x` coordinate, we iterate over the range of changes
                // and insert a rock item into our map.
                match first_x.cmp(x) {
                    std::cmp::Ordering::Less => {
                        for current_x in first_x..*x + 1 {
                            map.insert((current_x, *y), Item::Rock);
                        }
                    }
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => {
                        for current_x in *x..first_x + 1 {
                            map.insert((current_x, *y), Item::Rock);
                        }
                    }
                }

                // Similarly to a change in `x`, we also record any changes in the `y` coordinate.
                match first_y.cmp(y) {
                    std::cmp::Ordering::Less => {
                        for current_y in first_y..*y + 1 {
                            map.insert((*x, current_y), Item::Rock);
                        }
                    }
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => {
                        for current_y in *y..first_y + 1 {
                            map.insert((*x, current_y), Item::Rock);
                        }
                    }
                }

                // We change the current x and y coordinates to be the next iterations starting
                // coordinates.
                first_x = *x;
                first_y = *y;

                // If this is the heighest point we have seen yet, we record it into our height
                // variable. Otherwise we move on.
                if first_y > height {
                    height = first_y;
                }
            }
        });

    (map, height)
}

/// Drop a single drop of sand until it stops moving because of a rock or a peice of sand.
/// If we can't move the drop of sand any further we return false, but if we found a place for
/// this drop of sand we record it in the map and return true.
fn drop_sand(map: &mut HashMap<(u16, u16), Item>, height: &u16) -> bool {
    let (mut start_x, mut start_y) = (500, 0);

    loop {
        // If we are exceeding the height of the map we cannot move the sand any more.
        if start_y >= *height {
            return false;
        }

        // If there is no item below, we move down.
        if !map.contains_key(&(start_x, start_y + 1)) {
            start_y += 1;
            continue;
        }

        // If there was an item below, but not down and left, we go there.
        if !map.contains_key(&(start_x - 1, start_y + 1)) {
            start_x -= 1;
            start_y += 1;
            continue;
        }

        // If both down and down left were taken, but not down right we go down right.
        if !map.contains_key(&(start_x + 1, start_y + 1)) {
            start_x += 1;
            start_y += 1;
            continue;
        }

        // The sand drop cannot go anywhere, but the current location is open, so we put it here
        // and return true.
        if !map.contains_key(&(start_x, start_y)) {
            map.insert((start_x, start_y), Item::Sand);
            return true;
        }

        // We cannot go anywhere and the location is taken, so no more sand can go here.
        return false;
    }
}

/// Drop sand until no more sand can be.
fn drop_all_sand(map: &mut HashMap<(u16, u16), Item>, height: &u16) {
    while drop_sand(map, height) {}
}

fn main() {
    // Get the cave layout and height.
    let (mut map, height) = get_rock_locations("input.txt");

    // Drop sand into the cave.
    drop_all_sand(&mut map, &height);

    // Count the number of sand drops in the cave.
    let sand_units = map.iter().filter(|(_, item)| item == &&Item::Sand).count();

    // Add a floor to the cave.
    (0..std::u16::MAX).for_each(|x| {
        map.insert((x, height + 2), Item::Rock);
    });

    // Drop more sand into the cave.
    drop_all_sand(&mut map, &(height + 2));

    // Count the number of sand drops in the cave again.
    let second_sand_units = map.iter().filter(|(_, item)| item == &&Item::Sand).count();

    println!("{sand_units}");
    println!("{second_sand_units}");
}
