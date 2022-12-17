use once_cell::sync::OnceCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// A struct which holds the data of a valve location.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    flow_rate: u32,
    tunnels: BTreeSet<String>,
}

/// Read the input scan into a graph of valves.
fn read_scan(filename: &str) -> BTreeMap<String, Valve> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let name = line.get(6..8).unwrap().to_string();
            let equal_index = line.find("=").unwrap() + 1;
            let semi_index = line.find(";").unwrap();
            let flow_rate = line.get(equal_index..semi_index).unwrap().parse().unwrap();
            let tunnels_index = line.find("valve").unwrap();
            let tunnels_string = line.get(tunnels_index + 5..).unwrap();
            let tunnels_string = if tunnels_string.starts_with("s") {
                tunnels_string.get(2..).unwrap()
            } else {
                tunnels_string.get(1..).unwrap()
            };

            let tunnels = tunnels_string
                .split(", ")
                .map(|string| string.to_string())
                .collect();

            (name, Valve { flow_rate, tunnels })
        })
        .collect()
}

/// Map the graph of valves into vectors of flow rates and tunnels.
fn map_tunnels_to_ints(tunnels: BTreeMap<String, Valve>) -> (Vec<u32>, Vec<Vec<u32>>) {
    let mut name_map = BTreeMap::new();

    // Map the names of valves to the indexes of the valves.
    tunnels.iter().enumerate().for_each(|(index, (name, _))| {
        if !name_map.contains_key(name) {
            name_map.insert(name.clone(), index as u32);
        }
    });

    // Create a vector of flow rates. The index of the flow rate is the index of the valve.
    let flow_map = tunnels.iter().map(|(_, valve)| valve.flow_rate).collect();

    // Createt a vector of vectors of tunnels. The index of the vector of tunnels is the index of
    // the valve which can lead to the valves in the vector. We need to map each tunnel to the
    // index of that valve.
    let tunnel_map = tunnels
        .iter()
        .map(|(_, valves)| {
            valves
                .tunnels
                .iter()
                .map(|tunnel| *name_map.get(tunnel).unwrap())
                .collect()
        })
        .collect();

    (flow_map, tunnel_map)
}

/// A hash map cache for our recursive calls. We want to reduce computation so we skip each
/// invocation of a already seen set of inputs and return the outcome of that invocation.
static mut CACHE: OnceCell<HashMap<String, u32>> = OnceCell::new();

/// We recursively compute the maximum flow rate starting from the valve `valve` given the opened
/// valves `opened_valves`, minutes available `minutes_available` and number of other players
/// `other_players`.
fn max_flow_rate(
    valve: u32,
    valves: &Vec<u32>,
    tunnels: &Vec<Vec<u32>>,
    opened_valves: u64,
    minutes_available: u32,
    other_players: u32,
) -> u32 {
    // If there are no minutes left we check if there are more players to compute for.
    if minutes_available == 0 {
        // If there are more players to compute for, we start at the start valve and reset the
        // minutes available to 26, but we keep the same valves open.
        return if other_players > 0 {
            max_flow_rate(0, valves, tunnels, opened_valves, 26, other_players - 1)
        // Otherwise we just return 0.
        } else {
            0
        };
    }

    // We create a key to check for cached invocations.
    let key = format!("{valve}-{opened_valves}-{minutes_available}-{other_players}");

    // If there exists a invocation under the computed key, we return that value.
    unsafe {
        if let Some(value) = CACHE.get().unwrap().get(&key) {
            return *value;
        }
    }

    // Our assumed max flow rate is initially 0.
    let mut max_flow = 0;

    // We create a bit mask for opening the current valve.
    let mask = 1 << valve;
    // We get the flow rate of the current valve.
    let flow = valves.get(valve as usize).unwrap();

    // If the valve is not already opened, and the flow rate is more than 0 we call recursively
    // with the valve open and minutes available decreased.
    if opened_valves & mask == 0 && flow > &0 {
        // Add the flow rate increase.
        let flow_rate = flow * (minutes_available - 1);

        // Create the new opened valves value.
        let new_opened = opened_valves | mask;

        // We find the max between the current max flow rate and the flow rate of the next
        // recursive call with this valve open.
        max_flow = max_flow.max(
            flow_rate
                + max_flow_rate(
                    valve,
                    valves,
                    tunnels,
                    new_opened,
                    minutes_available - 1,
                    other_players,
                ),
        );
    }

    // For all the tunnels this valve location is connected to, we recurse and find the max flow
    // rate.
    for &tunnel in tunnels.get(valve as usize).unwrap() {
        max_flow = max_flow.max(max_flow_rate(
            tunnel,
            valves,
            tunnels,
            opened_valves,
            minutes_available - 1,
            other_players,
        ));
    }

    // We update the cache for this call with the max flow we calculated.
    unsafe {
        CACHE.get_mut().unwrap().insert(key, max_flow);
    }

    max_flow
}

fn main() {
    // Get the valves graph from the input scan.
    let valves = read_scan("input.txt");
    // We map the valves to vectors.
    let (flow, tunnels) = map_tunnels_to_ints(valves);

    // We initialize the cache.
    unsafe {
        CACHE.set(HashMap::new()).unwrap();
    }

    // Calculate the max flow rate for one player and 30 minutes available.
    let max_flow = max_flow_rate(0, &flow, &tunnels, 0, 30, 0);

    println!("{max_flow}");

    // Calculate the max flow rate for two players and 26 minutes available.
    let max_flow_two_people = max_flow_rate(0, &flow, &tunnels, 0, 26, 1);

    println!("{max_flow_two_people}");
}
