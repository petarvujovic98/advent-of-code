use std::collections::BTreeMap;

/// Read commands from input file.
/// Split the text input by `cd` command calls because we want to know
/// when we change directory one level up or into a child directory.
/// If we are changing one level up, than we pop the last directory
/// from the context (the directory we are currently in), otherwise
/// the entry containing the child directory change will also have
/// the `ls` command call after which we will have the list of files
/// and directories contained inside which we will collect into a vector.
fn read_commands_and_lists(filename: &str) -> BTreeMap<String, Vec<String>> {
    let file = std::fs::read_to_string(filename).unwrap();
    let line_iterator = file.split("$ cd ");

    let mut current_context = vec![];

    BTreeMap::from_iter(line_iterator.skip(1).filter_map(|line| {
        let mut lines = line.trim().lines();
        let dir = lines.next().unwrap();

        match dir {
            ".." => {
                current_context.pop();
                None
            }
            name => {
                current_context.push(name);
                lines.next();
                Some((
                    current_context.join("/"),
                    lines.map(|line| line.to_string()).collect(),
                ))
            }
        }
    }))
}

/// Recursively get the size of a directory by summing the size of all
/// the files it directly containes and the file sizes of any files
/// contained within child directories by calling the same function.
fn get_dir_size(dir: &str, map: &BTreeMap<String, Vec<String>>) -> u32 {
    map.get(dir).unwrap().iter().fold(0, |sum, entry| {
        if entry.starts_with("dir ") {
            let child_dir = format!("{dir}/{}", entry.get(4..).unwrap());
            get_dir_size(&child_dir, map) + sum
        } else {
            entry.split(" ").next().unwrap().parse::<u32>().unwrap() + sum
        }
    })
}

/// Calculate the directory sizes for all directories in the map by calling
/// the `get_dir_size` function for each entry.
fn get_dir_with_entries(map: &BTreeMap<String, Vec<String>>) -> BTreeMap<String, u32> {
    BTreeMap::from_iter(
        map.iter()
            .map(|(dir, _entries)| (dir.to_owned(), get_dir_size(&dir, map))),
    )
}

fn main() {
    // Get the directories and their entries.
    let map = read_commands_and_lists("input.txt");

    // Get the directory sizes.
    let sizes = get_dir_with_entries(&map);

    // Get the sum of all directories which have a size less than 100_000.
    let sum = sizes
        .iter()
        .filter_map(|(_, size)| if size <= &100_000 { Some(size) } else { None })
        .sum::<u32>();

    // Calculate how much needs to be deleted to have room for the update.
    let to_delete = 30_000_000 - (70_000_000 - sizes.get("/").unwrap());

    // Find the smallest of the directories large enough to free up enough space.
    let smallest_large_enough = sizes
        .iter()
        .filter_map(|(_, size)| if size > &&to_delete { Some(size) } else { None })
        .min()
        .unwrap();

    println!("{sum}");
    println!("{smallest_large_enough}");
}
