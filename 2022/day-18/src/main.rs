use std::collections::{HashSet, VecDeque};

/// A struct that represents a 1x1x1 cube by its coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: i16,
    y: i16,
    z: i16,
}

impl Cube {
    /// Create a new cube from a x,y,z string.
    pub fn new(line: &str) -> Self {
        let mut coords = line.split(",").map(|number| number.parse().unwrap());

        Self {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        }
    }

    /// Get a set of neighbours of the cube.
    fn get_neighbours(&self) -> HashSet<Self> {
        [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        .map(|(x, y, z)| Cube {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        })
        .collect()
    }

    /// Count the visible sides of a cube in a given cube set.
    fn visible_sides(&self, others: &HashSet<Self>) -> usize {
        let neighbours = self.get_neighbours();
        let count_neighbours = others.intersection(&neighbours).count();

        6 - count_neighbours
    }

    /// Check if cube is within the given bounds.
    fn in_bounds(&self, lower: &Self, upper: &Self) -> bool {
        lower.x <= self.x
            && self.x <= upper.x
            && lower.y <= self.y
            && self.y <= upper.y
            && lower.z <= self.z
            && self.z <= upper.z
    }
}

/// Read the cube coordinates from the input file into a set.
fn read_cubes(filename: &str) -> HashSet<Cube> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| Cube::new(line))
        .collect()
}

/// Visit all the cubes outside the given set to check how many sides are reachable from outside.
/// Use BFS to visit all cubes.
fn count_outside(cubes: &HashSet<Cube>) -> usize {
    // Find the max coordinates in the set of cubes.
    let (max_x, max_y, max_z) = cubes
        .iter()
        .fold((0, 0, 0), |(max_x, max_y, max_z), Cube { x, y, z }| {
            (max_x.max(*x), max_y.max(*y), max_z.max(*z))
        });
    // Create a start/lower bound cube.
    let start = Cube {
        x: -1,
        y: -1,
        z: -1,
    };
    // Create a end/upper bound cube.
    let end = Cube {
        x: max_x + 1,
        y: max_y + 1,
        z: max_z + 1,
    };
    // Keep track of visited cubes.
    let mut visited = HashSet::new();
    // Create a queue of cubes to visit.
    let mut queue = VecDeque::from_iter([start]);
    // Keep running count of visible sides.
    let mut count = 0;

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();

        // If the cube is in the given set, increase the count.
        if cubes.contains(&next) {
            count += 1;
            continue;
        }

        // If cube is not already visited and is in bounds, add it to the visited set.
        // Also add its neighbours to the queue.
        if !visited.contains(&next) && next.in_bounds(&start, &end) {
            visited.insert(next);

            queue.extend(next.get_neighbours());
        }
    }

    count
}

fn main() {
    // Get the cubes from the input file.
    let cubes = read_cubes("input.txt");

    // Count all the visible sides.
    let visible_sides = cubes
        .iter()
        .map(|cube| cube.visible_sides(&cubes))
        .sum::<usize>();

    // Count the sides visible from the outside.
    let count_outside = count_outside(&cubes);

    println!("{visible_sides}");
    println!("{count_outside}");
}
