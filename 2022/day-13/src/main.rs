/// An enum that represents a list of integers or lists.
#[derive(Eq, PartialOrd, Clone, Debug)]
enum Item {
    Integer(i32),
    List(Vec<Item>),
}

/// Get the outmost bracket pair if the string starts with a bracket and whether that bracket pair
/// surrounds the whole string, otherwise return None.
fn get_outmost_bracket_pair(string: &str) -> Option<(usize, bool)> {
    if string.starts_with("[") {
        let mut pairs = 0;

        for (index, char) in string.chars().enumerate() {
            match char {
                '[' => pairs += 1,
                ']' => pairs -= 1,
                _ => (),
            }

            if pairs == 0 {
                return Some((index, index == string.len() - 1));
            }
        }

        return None;
    }

    return None;
}

impl Item {
    /// Check if the item is an integer.
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Integer(_) => true,
            _ => false,
        }
    }

    /// Returns the list of an item if the item is of the List variant.
    pub fn get_list(self) -> Option<Vec<Item>> {
        match self {
            Self::List(list) => Some(list),
            _ => None,
        }
    }

    /// Parse an item from a line of text by recursively parsing lists until we get to either an
    /// empty list or an integer.
    pub fn new(string: &str) -> Self {
        // If string is empty return an empty list.
        if string.is_empty() {
            return Self::List(vec![]);
        }

        // If we don't have any nested lists, return a list of integers.
        if !string.contains("[") {
            return Self::List(
                string
                    .split(",")
                    .map(|int| Self::Integer(int.parse().unwrap()))
                    .collect(),
            );
        }

        // Get the outermost bracket pair is there is one and check if it wraps the current string.
        let split_index = if let Some((closing_index, wrapped)) = get_outmost_bracket_pair(string) {
            // If the current string is wrapped with brackets continue parsing inside.
            if wrapped {
                let inner_items = Self::new(string.get(1..closing_index).unwrap());

                // If it was just an integer inside, wrap it in a list and return it.
                if inner_items.is_integer() {
                    return Self::List(vec![inner_items]);
                }

                // If it was a list we have two scenarios.
                return match get_outmost_bracket_pair(string.get(1..closing_index).unwrap()) {
                    // If the inner list was wrapped in brackets, put the elements back in
                    // a bracket and wrap that with a new list and return it.
                    Some((_, true)) => Self::List(vec![inner_items]),
                    // Otherwise just return the contents wrapped in a list.
                    // This could be a list of integers or lists, or a mix.
                    _ => inner_items,
                };
            }

            // If the string wasn't wrapped but started with a bracket, return the index of the
            // comma after the matching closing bracket
            closing_index + 1
        } else {
            // If the string doesn't start with a bracket, it means there are brackets after the
            // first element, so we find the index of the first comma.
            string.find(",").unwrap()
        };

        // We split the string at the first comma.
        let (first, rest) = string.split_at(split_index);
        // We skip the first character of the rest of the string because it is a comma.
        let rest_of_string = rest.get(1..).unwrap();

        // We parse the first item, it could be a list or integer but we don't need to know.
        let first_item = Self::new(first);

        // We parse the rest of the items, for these we want to know if they were
        let rest_of_items = Self::new(rest_of_string);

        // If the rest of the items is just one integer, put the first an second item into a
        // vector.
        Self::List(if rest_of_items.is_integer() {
            vec![first_item, rest_of_items]
        } else {
            // If the rest of the items are a list we want to know if the list is just multiple
            // siblings, or a sublist.
            match get_outmost_bracket_pair(rest_of_string) {
                // If it is a sublist, we treat it as a single sibling just like in the integer
                // scenario.
                Some((_, true)) => vec![first_item, rest_of_items],
                // If it is multiple siblings, we insert the first item at the start and return the
                // modified list of items.
                _ => {
                    let mut items = rest_of_items.get_list().unwrap();
                    items.insert(0, first_item);

                    items
                }
            }
        })
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(left), Self::Integer(right)) => left == right,
            (Self::List(left), Self::List(right)) => left == right,
            (Self::Integer(left), Self::List(right)) => &vec![Self::Integer(*left)] == right,
            (Self::List(left), Self::Integer(right)) => left == &vec![Self::Integer(*right)],
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Integer(left), Item::Integer(right)) => left.cmp(right),
            (Item::List(left), Item::List(right)) => left.cmp(&right),
            (Item::Integer(left), Item::List(right)) => vec![Item::Integer(*left)].cmp(right),
            (Item::List(left), Item::Integer(right)) => left.cmp(&vec![Item::Integer(*right)]),
        }
    }
}

/// Read the packet pairs from the input file into pairs of items.
fn read_packet_pairs(filename: &str) -> Vec<(Item, Item)> {
    std::fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|packets| {
            let lines = packets.lines().collect::<Vec<_>>();
            let first_packet = Item::new(lines.first().unwrap());
            let second_packet = Item::new(lines.last().unwrap());

            (first_packet, second_packet)
        })
        .collect()
}

/// Filter through the pairs of packets to find the correctly ordered pairs and return their index
/// - the index starts at 1 so we add 1 to the actual iterator's index.
fn find_right_order_pair_indices(pairs: &[(Item, Item)]) -> Vec<usize> {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(index, (first, second))| match first.cmp(second) {
            std::cmp::Ordering::Greater => None,
            std::cmp::Ordering::Equal => panic!("not expected"),
            std::cmp::Ordering::Less => Some(index + 1),
        })
        .collect()
}

/// Read all the packets from the input file while ignoring pairings / empty lines.
fn read_packets(filename: &str) -> Vec<Item> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(Item::new(line))
            }
        })
        .collect()
}

fn main() {
    // Get the packet pairs.
    let pairs = read_packet_pairs("input.txt");
    // Get the indices of the correctly ordered packet pairs.
    let indices = find_right_order_pair_indices(&pairs);
    // Sum the bracket pair indices.
    let sum = indices.iter().sum::<usize>();

    // Get all the packets.
    let mut packets = read_packets("input.txt");
    // Create the divider packets.
    let two_packet = Item::new("[[2]]");
    let six_packet = Item::new("[[6]]");

    // Insert the divider packets into our list.
    packets.extend([two_packet.clone(), six_packet.clone()]);

    // Sort the packets vector.
    packets.sort_unstable_by(|left, right| left.cmp(right));

    // Find the index of the first divider packet.
    let (index_two, _) = packets
        .iter()
        .enumerate()
        .find(|(_, packet)| packet == &&two_packet)
        .unwrap();

    // Find the index of the second divider packet.
    let (index_six, _) = packets
        .iter()
        .enumerate()
        .find(|(_, packet)| packet == &&six_packet)
        .unwrap();

    println!("{sum}");
    println!("{}", (index_six + 1) * (index_two + 1));
}
