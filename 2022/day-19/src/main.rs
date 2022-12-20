use std::{collections::HashMap, hash::Hash};

use once_cell::sync::OnceCell;

/// An enum that represents a robot worker which can collect/crack a type of resource.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    /// List out all the robot/resource types.
    pub fn all_types() -> Vec<Self> {
        use Robot::*;

        vec![Ore, Clay, Obsidian, Geode]
    }
}

/// A struct that keeps track of how many resources we have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Storage {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Storage {
    /// Create a storage instance with 0 of each resource.
    pub fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    /// Increase the count of resources gathered by the count of robots for each resource and the
    /// given number of iterations/minutes for gathering.
    pub fn gather(&mut self, robots: &HashMap<Robot, i32>, iterations: i32) {
        robots.iter().for_each(|(robot, count)| match robot {
            Robot::Ore => self.ore += *count * iterations,
            Robot::Clay => self.clay += *count * iterations,
            Robot::Obsidian => self.obsidian += *count * iterations,
            Robot::Geode => self.geode += *count * iterations,
        });
    }
}

/// A struct that represents a blueprint for robot building costs.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Blueprint {
    ore: i32,
    clay: i32,
    obsidian: (i32, i32),
    geode: (i32, i32),
    max_spend: HashMap<Robot, i32>,
}

/// The start of the ore cost for obsidian robots.
const OBSIDIAN_START_ORE: usize = " Each obsidian robot costs ".len();
/// The start of the clay cost for obsidian robots.
const OBSIDIAN_START_CLAY: usize = OBSIDIAN_START_ORE + 1 + " ore and ".len();
/// The start of the ore cost for geode robots.
const GEODE_START_ORE: usize = " Each geode robot costs ".len();
/// The start of the obsidian cost for geode robots.
const GEODE_START_OBSIDIAN: usize = GEODE_START_ORE + 1 + " ore and ".len();

/// A global cache for recursive calls.
static mut CACHE: OnceCell<HashMap<String, i32>> = OnceCell::new();

/// Helper function to query the cache.
fn check_cache(key: &str) -> Option<i32> {
    unsafe { CACHE.get().unwrap().get(key).copied() }
}

/// Helper function to update the cache.
fn update_cache(key: String, value: i32) {
    unsafe {
        CACHE.get_mut().unwrap().insert(key, value);
    }
}

/// Helper function to extract an integer from a string.
fn get_int(string: &str) -> i32 {
    string
        .chars()
        .filter(|char| char.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap()
}

impl Blueprint {
    /// Parse a new blueprint from a blueprint line.
    pub fn new(line: &str) -> Self {
        let mut costs = line.split(":").skip(1).next().unwrap().split(".");

        let ore = get_int(costs.next().unwrap());

        let clay = get_int(costs.next().unwrap());

        let obsidian_str = costs.next().unwrap();
        let obsidian_ore = get_int(obsidian_str.get(..OBSIDIAN_START_ORE + 2).unwrap());
        let obsidian_clay = get_int(obsidian_str.get(OBSIDIAN_START_CLAY..).unwrap());

        let geode_str = costs.next().unwrap();
        let geode_ore = get_int(geode_str.get(..GEODE_START_ORE + 2).unwrap());
        let geode_obsidian = get_int(geode_str.get(GEODE_START_OBSIDIAN..).unwrap());

        Self {
            ore,
            clay,
            obsidian: (obsidian_ore, obsidian_clay),
            geode: (geode_ore, geode_obsidian),
            // Find the max spend for each resource type.
            max_spend: HashMap::from_iter([
                (Robot::Ore, ore.max(clay).max(obsidian_ore).max(geode_ore)),
                (Robot::Clay, obsidian_clay),
                (Robot::Obsidian, geode_obsidian),
            ]),
        }
    }

    /// Get the ore cost for a robot type.
    fn get_ore_cost(&self, robot: &Robot) -> i32 {
        match robot {
            Robot::Ore => self.ore,
            Robot::Clay => self.clay,
            Robot::Obsidian => self.obsidian.0,
            Robot::Geode => self.geode.0,
        }
    }

    /// Calculate the time needed to wait to build a given robot type. If no robots that build the
    /// resources required for this robots creation exist return None. Otherwise return the number
    /// of minutes before we are able to create a robot of the given type.
    fn time_to_next_robot(
        &self,
        robot: &Robot,
        robots: &HashMap<Robot, i32>,
        storage: &Storage,
    ) -> Option<i32> {
        let ore_cost = self.get_ore_cost(&robot);
        let Some(count) = robots.get(&Robot::Ore) else {
            return None;
        };

        let ore_time = 0.max((ore_cost - storage.ore + *count - 1) / *count);

        match robot {
            Robot::Ore | Robot::Clay => Some(ore_time),
            Robot::Obsidian => {
                let Some(count) = robots.get(&Robot::Clay) else {
                  return None;
                };

                Some(ore_time.max((self.obsidian.1 - storage.clay + *count - 1) / *count))
            }
            Robot::Geode => {
                let Some(count) = robots.get(&Robot::Obsidian) else {
                    return None;
                };

                Some(ore_time.max((self.geode.1 - storage.obsidian + *count - 1) / *count))
            }
        }
    }

    /// Remove any extra robots. We consider robots that build more resources than we can use
    /// in a single turn to be extra robots.
    fn remove_extra_robots(&self, robots: &mut HashMap<Robot, i32>) {
        for (robot, count) in robots
            .iter_mut()
            .filter(|(robot, _)| robot != &&Robot::Geode)
        {
            *count = (*count).min(*self.max_spend.get(robot).unwrap_or(&0));
        }
    }

    /// Remove any extra resources. We consider resources that have more units than we can spend in
    /// the remaining turns to be extra resources.
    fn remove_extra_resources(
        &self,
        robots: &HashMap<Robot, i32>,
        storage: &mut Storage,
        iterations: i32,
    ) {
        self.max_spend
            .iter()
            .for_each(|(ore_type, spend)| match ore_type {
                Robot::Ore => {
                    storage.ore = storage.ore.min(
                        (*spend * iterations)
                            - (iterations - 1) * robots.get(ore_type).unwrap_or(&0),
                    )
                }
                Robot::Clay => {
                    storage.clay = storage.clay.min(
                        (*spend * iterations)
                            - (iterations - 1) * robots.get(ore_type).unwrap_or(&0),
                    )
                }
                Robot::Obsidian => {
                    storage.obsidian = storage.obsidian.min(
                        (*spend * iterations)
                            - (iterations - 1) * robots.get(ore_type).unwrap_or(&0),
                    )
                }
                Robot::Geode => panic!("Shouldn't happen"),
            });
    }

    /// Pay for a robot creation. We decrease the amount of resources in storage based on the robot
    /// type and it's cost according to the blueprint.
    fn pay_for_robot(&self, storage: &mut Storage, robot: &Robot) {
        match robot {
            Robot::Ore => storage.ore -= self.ore,
            Robot::Clay => storage.ore -= self.clay,
            Robot::Obsidian => {
                storage.ore -= self.obsidian.0;
                storage.clay -= self.obsidian.1;
            }
            Robot::Geode => {
                storage.ore -= self.geode.0;
                storage.obsidian -= self.geode.1;
            }
        }
    }

    /// Recursively search for the decision chain which would bring us the largest amount of
    /// geodes.
    fn max_geodes(
        &self,
        minutes_left: i32,
        robots: &HashMap<Robot, i32>,
        storage: &Storage,
    ) -> i32 {
        // If there is no time left we return the number of geodes we have in storage.
        if minutes_left == 0 {
            return storage.geode;
        }

        // Create a key for the cache based on current parameters.
        let key = format!("{minutes_left}:{self:?}+{robots:?}+{storage:?}");

        // If there is a cache hit we return the value from the cache.
        if let Some(result) = check_cache(&key) {
            return result;
        }

        let mut max_geodes = storage.geode;

        // Increase the assumed number of max geodes by the amount of geodes the current geode
        // robots would produce in the remaining time.
        if let Some(geode_robots) = robots.get(&Robot::Geode) {
            max_geodes += geode_robots * minutes_left;
        }

        // Iterate through all robot types.
        for robot_type in Robot::all_types() {
            // If the robot type count is larger than the max amount we could spend we just ignore
            // this path.
            if let (Some(spend), Some(count)) =
                (self.max_spend.get(&robot_type), robots.get(&robot_type))
            {
                if count >= spend {
                    continue;
                }
            }

            // If there is not time we could wait to build a robot of this type we skip this path,
            // otherwise we record the time we would wait.
            let Some(wait_time) = self.time_to_next_robot(&robot_type, robots, storage) else {
                continue;
            };

            let remaining_time = minutes_left - wait_time - 1;

            // If time leftover after the robot creation is zero or less, we ignore this path.
            if remaining_time <= 0 {
                continue;
            }

            let mut storage_clone = storage.clone();

            // Gather the resources with the current robots.
            storage_clone.gather(robots, wait_time + 1);

            // Pay for the robot creation.
            self.pay_for_robot(&mut storage_clone, &robot_type);

            let mut robots_clone = robots.clone();

            // Add the robot to our robots map.
            robots_clone
                .entry(robot_type)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            // Remove any extra robots.
            self.remove_extra_robots(&mut robots_clone);

            // Remove any extra resources.
            self.remove_extra_resources(&robots_clone, &mut storage_clone, remaining_time);

            // Find the max geodes we could build in the remaining time.
            max_geodes =
                max_geodes.max(self.max_geodes(remaining_time, &robots_clone, &storage_clone));
        }

        // Update the cache with the new result.
        update_cache(key, max_geodes);

        max_geodes
    }
}

/// Read the blueprints from a given input file into a vector.
fn get_blueprints(filename: &str) -> Vec<Blueprint> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| Blueprint::new(line))
        .collect()
}

fn main() {
    // Get the blueprints.
    let blueprints = get_blueprints("input.txt");

    // Initialize the starting values.
    let starting_robots = HashMap::from_iter([(Robot::Ore, 1)]);
    let storage = Storage::new();
    unsafe {
        CACHE.set(HashMap::new()).unwrap();
    }

    // Sum the quality levels of each blueprint.
    let quality_levels = blueprints
        .iter()
        .enumerate()
        .map(|(index, blueprint)| {
            blueprint.max_geodes(24, &starting_robots.clone(), &storage.clone())
                * (index + 1) as i32
        })
        .sum::<i32>();

    println!("{quality_levels}");

    // Calculate the product of the first three blueprints' maximum geodes cracked.
    let first_three_product = blueprints
        .iter()
        .take(3)
        .map(|blueprint| blueprint.max_geodes(32, &starting_robots.clone(), &storage.clone()))
        .product::<i32>();

    println!("{first_three_product}");
}
