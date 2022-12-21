use std::collections::HashMap;

/// An enum that represents a arithmetic operation that a monkey could yell out.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    /// Create a new operation from a string containing one of the '+','-','*','/' characters.
    pub fn new(input: &str) -> Self {
        match input {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Invalid input"),
        }
    }

    /// Perform a operation.
    fn perform(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }

    /// Get the value of a left operand for the given right operand and result of operation.
    fn reverse_right(&self, right: i64, result: i64) -> i64 {
        match self {
            Self::Add => result - right,
            Self::Sub => result + right,
            Self::Mul => result / right,
            Self::Div => result * right,
        }
    }

    /// Get the value of a right operand for the given left operand and result of operation.
    fn reverse_left(&self, left: i64, result: i64) -> i64 {
        match self {
            Self::Add => result - left,
            Self::Sub => left - result,
            Self::Mul => result / left,
            Self::Div => left / result,
        }
    }
}

/// An enum that represents a value or arithmetic a monkey could yell. It also stores a human
/// variant for the case of a value placeholder.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Monkey {
    Number(i64),
    Math(String, Operation, String),
    Human,
}

impl Monkey {
    /// Create a monkey from a monkey input line.
    pub fn new(line: &str) -> Self {
        let trimmed = line.trim();

        // If the line contains a space that means it is an arithmetic operation.
        if trimmed.contains(" ") {
            let mut split = trimmed.split(" ");
            let left = split.next().unwrap().to_string();
            let operation = Operation::new(split.next().unwrap());
            let right = split.next().unwrap().to_string();

            Self::Math(left, operation, right)
        // Otherwise it is a regular number.
        } else {
            Self::Number(trimmed.parse().unwrap())
        }
    }

    /// calculate the value the monkey is yelling given what all the other monkeys yell. Update the
    /// value for each monkey if their arithmetic can be calculated. If we encounter a human
    /// placeholder we return None.
    fn get_value(&self, monkeys: &mut HashMap<String, Self>) -> Option<i64> {
        if let Self::Number(value) = self {
            return Some(*value);
        }

        if self == &Self::Human {
            return None;
        }

        // We already checked for the cases where the monkey could be a human or number so we know
        // that we can only get a math monkey here.
        let Self::Math(left, operation, right) = self else {
            unreachable!();
        };

        // We clone the monkeys to avoid borrow collisions.
        let clone = monkeys.clone();

        // If at any point we don't find a monkey, we return None - this shouldn't happen.
        let left_monkey = clone.get(left)?;
        let right_monkey = clone.get(right)?;

        // Calculate the values for each monkey recursively.
        let left_value = left_monkey.get_value(monkeys);
        let right_value = right_monkey.get_value(monkeys);

        // We try to update the monkey in our monkey map in case we were able to calculate the
        // value and the monkey was a math monkey, otherwise we continue.
        if let Self::Math(..) = left_monkey {
            if left_value.is_some() {
                monkeys.insert(left.to_string(), Self::Number(left_value.unwrap()));
            }
        }

        // We do the same for the right monkey.
        if let Self::Math(..) = right_monkey {
            if right_value.is_some() {
                monkeys.insert(right.to_string(), Self::Number(right_value.unwrap()));
            }
        }

        // If we have a left and right value we return the result of the operation between the two
        // values.
        Some(operation.perform(left_value?, right_value?))
    }

    /// Calculate the value we need to set to a placeholder monkey in order to have the `value` be
    /// the result of this monkey's arithmetics.
    fn adjust(&self, monkeys: &HashMap<String, Self>, value: i64) -> i64 {
        match self {
            // If the current monkey is a placeholder we just return the value.
            Self::Human => value,
            // If the monkey is a number than something went wrong.
            Self::Number(_) => {
                panic!("Shouldn't be here")
            }
            // Now we recursively find the next monkey to adjust.
            Self::Math(left, operation, right) => {
                let right_monkey = monkeys.get(right).unwrap();
                let left_monkey = monkeys.get(left).unwrap();

                match left_monkey {
                    // If the left monkey is a number we adjust the right monkey.
                    Self::Number(left_value) => {
                        right_monkey.adjust(monkeys, operation.reverse_left(*left_value, value))
                    }
                    // Otherwise we adjust the left monkey.
                    _ => match right_monkey {
                        Self::Number(right_value) => left_monkey
                            .adjust(monkeys, operation.reverse_right(*right_value, value)),
                        // The right monkey should be a number and if it isn't then something went
                        // wrong.
                        _ => panic!("Shouldn't happen"),
                    },
                }
            }
        }
    }
}

/// Get the monkeys and what they are yelling from the input file into a hash map.
fn read_monkeys(filename: &str) -> HashMap<String, Monkey> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(": ");

            (
                split.next().unwrap().to_string(),
                Monkey::new(split.next().unwrap()),
            )
        })
        .collect()
}

fn main() {
    // Get the monkeys.
    let mut monkeys = read_monkeys("input.txt");

    // Clone the monkeys to save the original equations.
    let mut clone = monkeys.clone();

    // Get the root monkey.
    let root_monkey = monkeys.get("root").unwrap().clone();

    // Get the value of the root monkey.
    let number = root_monkey.get_value(&mut clone).unwrap();

    println!("{number}");

    // Get the left and right operands of what the root monkey is yelling.
    if let Monkey::Math(left, _, right) = root_monkey {
        // Insert a placeholder at the `humn` monkey position.
        monkeys.insert("humn".to_string(), Monkey::Human);

        // Get the values of the left and right operands we calculated earlier.
        let left_value = clone.get(&left).unwrap().clone();
        let right_value = clone.get(&right).unwrap().clone();

        // Get the monkeys from the original input.
        let left_tree = monkeys.get(&left).unwrap().clone();
        let right_tree = monkeys.get(&right).unwrap().clone();

        // If the left monkey uses a placeholder somewhere in their calculations, we find the
        // adjusted value inside that subtree that would make the subtree's value equal the right
        // subtree's value.
        let adjusted = if left_tree.get_value(&mut monkeys).is_none() {
            left_tree.adjust(&monkeys, right_value.get_value(&mut clone).unwrap())
        // We do the same process for the right subtree if the left didn't use the placeholder.
        } else {
            right_tree.adjust(&monkeys, left_value.get_value(&mut clone).unwrap())
        };

        println!("{adjusted}");
    }
}
