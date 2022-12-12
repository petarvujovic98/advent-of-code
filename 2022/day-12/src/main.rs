use std::collections::HashMap;

/// A struct which represents a point in the heightmap.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Node {
    coords: (usize, usize),
    distance: usize,
    height: u8,
    start: bool,
    end: bool,
}

impl Node {
    /// Create a new node given the coordinates and the height of the point as a character.
    pub fn new(x: usize, y: usize, character: char) -> Self {
        let height = match character {
            'S' => 'a' as u8,
            'E' => 'z' as u8,
            other => other as u8,
        };

        Self {
            coords: (x, y),
            distance: 0,
            height: height - 'a' as u8,
            start: character == 'S',
            end: character == 'E',
        }
    }
}

/// Create heightmap from the input file and collect width and height of input.
fn read_map(filename: &str) -> (HashMap<(usize, usize), Node>, (usize, usize)) {
    let mut x = 0;

    let map = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            x = line.len();

            line.chars()
                .enumerate()
                .map(|(x, character)| ((x, y), Node::new(x, y, character)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<(usize, usize), Node>>();

    let y = map.len() / x;

    (map, (x, y))
}

/// Find the distance from the `Start` node to the `End` node using BFS (breadth first search).
fn calculate_distance(
    map: &HashMap<(usize, usize), Node>,
    (max_x, max_y): (usize, usize),
) -> usize {
    // Create a visited set.
    let mut visited = HashMap::<(usize, usize), Node>::new();

    // Find the `Start` node.
    let node = map.iter().find(|(_, node)| node.start).unwrap().1.clone();

    // Create a visitation queue with the start node as the first element.
    let mut next_to_visit = vec![node];

    // Loop while the visitation queue is not empty.
    while !next_to_visit.is_empty() {
        let next_node = next_to_visit.remove(0);
        let coords = next_node.coords;

        // If this node is already visited just skip adding it's neighbors to the queue.
        if visited.contains_key(&coords) {
            continue;
        }

        // Check if we are at the left border of the map.
        if coords.0 > 0 {
            let mut neighbor = map.get(&(coords.0 - 1, coords.1)).unwrap().clone();

            // Add the neigbor if we can move to it - e.g. if the neighbor is not more than one
            // point heigher.
            if next_node.height + 1 >= neighbor.height {
                neighbor.distance = next_node.distance + 1;
                next_to_visit.push(neighbor);
            }
        }

        // Check if we are at the right border of the map.
        if coords.0 + 1 < max_x {
            let mut neighbor = map.get(&(coords.0 + 1, coords.1)).unwrap().clone();

            // Add the neigbor if we can move to it - e.g. if the neighbor is not more than one
            // point heigher.
            if next_node.height + 1 >= neighbor.height {
                neighbor.distance = next_node.distance + 1;
                next_to_visit.push(neighbor);
            }
        }

        // Check if we are at the bottom border of the map.
        if coords.1 > 0 {
            let mut neighbor = map.get(&(coords.0, coords.1 - 1)).unwrap().clone();

            // Add the neigbor if we can move to it - e.g. if the neighbor is not more than one
            // point heigher.
            if next_node.height + 1 >= neighbor.height {
                neighbor.distance = next_node.distance + 1;
                next_to_visit.push(neighbor);
            }
        }

        // Check if we are at the top border of the map.
        if coords.1 + 1 < max_y {
            let mut neighbor = map.get(&(coords.0, coords.1 + 1)).unwrap().clone();

            // Add the neigbor if we can move to it - e.g. if the neighbor is not more than one
            // point heigher.
            if next_node.height + 1 >= neighbor.height {
                neighbor.distance = next_node.distance + 1;
                next_to_visit.push(neighbor.clone());
            }
        }

        visited.insert(coords, next_node);
    }

    // Find the distance from the start node to the end node - return the max distance if the end
    // node is not reachable from the start.
    if let Some((_, node)) = visited.iter().find(|(_, node)| node.end) {
        node.distance
    } else {
        std::usize::MAX
    }
}

/// Go through all low points in the map to find the best start point.
fn find_best_starting_point(map: &HashMap<(usize, usize), Node>, ranges: (usize, usize)) -> usize {
    // Make a clone of our map.
    let mut clone = map.clone();

    // Turn the given start point to a regular low point.
    let start = clone.iter_mut().find(|(_, node)| node.start).unwrap();
    start.1.start = false;

    // Iterate over the low points of the map and find the minimum distance from end node.
    map.iter()
        .filter_map(|(coords, node)| match node.height {
            0 => {
                // Make the current node a start node in the clone map.
                let clone_node = clone.get_mut(&coords).unwrap();
                clone_node.start = true;

                // Get the distance from this start node to the end node.
                let result = calculate_distance(&clone, ranges);

                // Revert the current node into a low point node in the clone map.
                let clone_node = clone.get_mut(&coords).unwrap();
                clone_node.start = false;

                Some(result)
            }
            _ => None,
        })
        .min()
        .unwrap()
}

fn main() {
    // Read the heightmap from the input file.
    let (map, ranges) = read_map("input.txt");

    // Get the distance of the starting node to the end node.
    let distance = calculate_distance(&map, ranges);

    // Get the closest low point node's distance to the end node.
    let min_distance = find_best_starting_point(&map, ranges);

    println!("{distance}");
    println!("{min_distance}");
}
