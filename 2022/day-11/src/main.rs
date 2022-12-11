/// A struct representing the worry operation from an item inspection of a monkey.
#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Option<u128>),
    Multiply(Option<u128>),
}

impl Operation {
    /// Creates a new operation from the operation string and the right hand operand.
    pub fn new(op: &str, value: &str) -> Self {
        let value = match value.parse::<u128>() {
            Ok(int) => Some(int),
            Err(_) => None,
        };

        match op {
            "+" => Operation::Add(value),
            "*" => Operation::Multiply(value),
            _ => panic!("Invalid operation!"),
        }
    }

    /// Performs the worry operation on an item.
    pub fn run_operation(&self, item: &u128) -> u128 {
        match *self {
            Operation::Add(value) => match value {
                Some(value) => value + item,
                None => item + item,
            },
            Operation::Multiply(value) => match value {
                Some(value) => value * item,
                None => item * item,
            },
        }
    }
}

/// A struct that represents the items, worry operation, divisor, next monkey index as well as the
/// number of items the monkey has inspected.
#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    divisor: u128,
    true_index: usize,
    false_index: usize,
    items_inspected: u128,
}

impl Monkey {
    /// Creates a new monkey from the slice of monkey information - usually five consecutive lines.
    pub fn new(monkey: &[&str]) -> Self {
        // Collect the item worry values from the monkey.
        let items = monkey
            .get(0)
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .split(",")
            .map(|item| item.trim().parse().unwrap())
            .collect();

        // Collect the monkey operation.
        let second_line = monkey.get(1).unwrap().split(" ").collect::<Vec<_>>();
        let operation = Operation::new(
            second_line.get(second_line.len() - 2).unwrap(),
            second_line.last().unwrap(),
        );

        // Get the divisor for decision making.
        let divisor = monkey
            .get(2)
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        // Get the index of the monkey to go to if the division is with modulo 0.
        let true_index = monkey
            .get(3)
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        // Get the index of the monkey to go to if the division is with modulo other than 0.
        let false_index = monkey
            .get(4)
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Self {
            items,
            operation,
            divisor,
            true_index,
            false_index,
            // Set the items inspected to start at 0.
            items_inspected: 0,
        }
    }
}

/// Collect the monkeys from the input file.
fn read_monkeys(filename: &str) -> Vec<Monkey> {
    std::fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|monkey_string| Monkey::new(&monkey_string.lines().skip(1).collect::<Vec<_>>()))
        .collect()
}

/// Run a monkey turn by iterating through all the items of the monkey.
fn run_monkey_turn(index: usize, monkeys: &mut [Monkey]) {
    let mut monkey = monkeys.get(index).unwrap().clone();

    monkey.items.iter().for_each(|item| {
        let new_item_value = monkey.operation.run_operation(&item) / 3;

        let next_monkey_index = if new_item_value % monkey.divisor == 0 {
            monkey.true_index
        } else {
            monkey.false_index
        };

        let destination_monkey = monkeys.get_mut(next_monkey_index).unwrap();

        destination_monkey.items.push(new_item_value);
        monkey.items_inspected += 1;
    });

    monkey.items.clear();

    let new_monkey = monkeys.get_mut(index).unwrap();

    *new_monkey = monkey;
}

/// Run the monkey turn according to the new rules. Instead of dividing the worry level by three,
/// get the modulo of the worry level with base divisor - this is the product of all divisors in
/// the monkey slice.
fn run_new_rules_turn(index: usize, monkeys: &mut [Monkey], divisor: &u128) {
    let mut monkey = monkeys.get(index).unwrap().clone();

    monkey.items.iter().for_each(|item| {
        let new_item_value = monkey.operation.run_operation(&item) % divisor;

        let next_monkey_index = if new_item_value % monkey.divisor == 0 {
            monkey.true_index
        } else {
            monkey.false_index
        };

        let destination_monkey = monkeys.get_mut(next_monkey_index).unwrap();

        destination_monkey.items.push(new_item_value);
        monkey.items_inspected += 1;
    });

    monkey.items.clear();

    let new_monkey = monkeys.get_mut(index).unwrap();

    *new_monkey = monkey;
}

/// Run a monkey turn for each monkey in the slice.
fn run_round(monkeys: &mut [Monkey]) {
    for index in 0..monkeys.len() {
        run_monkey_turn(index, monkeys);
    }
}

/// Run a monkey turn for each monkey in the slice according to the new rules.
fn run_new_rules_round(monkeys: &mut [Monkey], divisor: &u128) {
    for index in 0..monkeys.len() {
        run_new_rules_turn(index, monkeys, divisor);
    }
}

fn main() {
    // Get the monkeys into a vector.
    let mut monkeys = read_monkeys("input.txt");
    // Clone the monkeys for part two.
    let mut monkeys_clone = monkeys.clone();

    // Run twenty rounds.
    for _ in 0..20 {
        run_round(&mut monkeys);
    }

    // Sort the monkeys by number of items inspected in descending order.
    monkeys.sort_by(|first, second| second.items_inspected.cmp(&first.items_inspected));

    // Calculate the monkey business value by taking the two most active monkeys and multiplying
    // the number of items inspected.
    let monkey_business = monkeys
        .iter()
        .take(2)
        .fold(1, |product, monkey| product * monkey.items_inspected);

    // Calculate the divisor - the product of divisors for each monkey.
    let divisor = monkeys_clone
        .iter()
        .fold(1, |product, monkey| product * monkey.divisor);

    // Run ten thousand rounds.
    for _ in 0..10_000 {
        run_new_rules_round(&mut monkeys_clone, &divisor);
    }

    // Sort the monkeys by number of items inspected in descending order.
    monkeys_clone.sort_by(|first, second| second.items_inspected.cmp(&first.items_inspected));

    // Calculate the monkey business value by taking the two most active monkeys and multiplying
    // the number of items inspected.
    let monkey_business_new = monkeys_clone
        .iter()
        .take(2)
        .fold(1, |product, monkey| product * monkey.items_inspected);

    println!("{monkey_business}");
    println!("{monkey_business_new}");
}
