/// Read tree height grid from input file.
fn read_grid(filename: &str) -> Vec<Vec<u8>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}

/// Check if tree at position [`x`, `y`] is visible in
/// grid `grid`.
/// Iterate through trees left of the position,
/// then iterate through trees right of the position,
/// after that through trees above the position and
/// finally through trees below the position.
/// If at any direction we get to the end of the grid
/// without seeing a tree as tall or taller than the
/// tree at position [`x`, `y`] we return `true`.
fn is_visible(x: usize, y: usize, grid: &[Vec<u8>]) -> bool {
    let row = grid.get(y).unwrap();
    let height = row.get(x).unwrap();

    for index in 0..x {
        let current_height = row.get(index).unwrap();

        if index == x - 1 && current_height < height {
            return true;
        } else if current_height >= height {
            break;
        }
    }

    for index in (x + 1..row.len()).rev() {
        let current_height = row.get(index).unwrap();

        if index == x + 1 && current_height < height {
            return true;
        } else if current_height >= height {
            break;
        }
    }

    for index in 0..y {
        let current_height = grid.get(index).unwrap().get(x).unwrap();

        if index == y - 1 && current_height < height {
            return true;
        } else if current_height >= height {
            break;
        }
    }

    for index in (y + 1..grid.len()).rev() {
        let current_height = grid.get(index).unwrap().get(x).unwrap();

        if index == y + 1 && current_height < height {
            return true;
        } else if current_height >= height {
            break;
        }
    }

    false
}

/// Calculate the scenic score for tree at position [`x`, `y`]
/// by iterating through trees from the curren tree towards an
/// edge. If at any time we encounter a tree as tall or taller
/// than the tree at position [`x`, `y`] we multiply the scenic
/// score by the distance between the trees. If we get to an
/// edge we multiply the scenic score by the distance from the
/// edge to the tree we are calculating the score for.
fn scenic_score(x: usize, y: usize, grid: &[Vec<u8>]) -> usize {
    let row = grid.get(y).unwrap();
    let height = row.get(x).unwrap();
    let mut scenic_score = 1;

    for index in (0..x).rev() {
        let current_height = row.get(index).unwrap();

        if index == 0 {
            scenic_score *= x;
        } else if current_height >= height {
            scenic_score *= x - index;
            break;
        }
    }

    for index in x + 1..row.len() {
        let current_height = row.get(index).unwrap();

        if index == row.len() - 1 {
            scenic_score *= row.len() - 1 - x;
        } else if current_height >= height {
            scenic_score *= index - x;
            break;
        }
    }

    for index in (0..y).rev() {
        let current_height = grid.get(index).unwrap().get(x).unwrap();

        if index == 0 {
            scenic_score *= y;
        } else if current_height >= height {
            scenic_score *= y - index;
            break;
        }
    }

    for index in y + 1..grid.len() {
        let current_height = grid.get(index).unwrap().get(x).unwrap();

        if index == grid.len() - 1 {
            scenic_score *= grid.len() - 1 - y;
        } else if current_height >= height {
            scenic_score *= index - y;
            break;
        }
    }

    scenic_score
}

fn main() {
    // Get the grid from the input file.
    let grid = read_grid("input.txt");

    // Count the visible trees inside the grid.
    let visible_count_inside = grid
        .iter()
        .enumerate()
        // We skip the first row as it is an edge and all trees are visible.
        .skip(1)
        // We ignore the last row for the same reason.
        .take(grid.len() - 2)
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                // We skip the first column as it is an edge and all trees are visible.
                .skip(1)
                // We ignore the last column for the same reason.
                .take(grid.last().unwrap().len() - 2)
                .filter(|(x, _)| is_visible(*x, y, &grid))
                .count()
        })
        .sum::<usize>();

    // We count the number of trees on the edges.
    let visible_count_outside = grid.len() * 2 + grid.last().unwrap().len() * 2 - 4;
    let visible_count = visible_count_inside + visible_count_outside;

    // Find the max scenic score within the grid.
    let max_scenic_score = grid
        .iter()
        .enumerate()
        // We skip the first row as it is an edge and scenic scores will be 0.
        .skip(1)
        // We ignore the last row for the same reason.
        .take(grid.len() - 2)
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                // We skip the first column as it is an edge and scenic scores will be 0.
                .skip(1)
                // We ignore the last column for the same reason.
                .take(grid.len() - 2)
                .map(|(x, _)| scenic_score(x, y, &grid))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{visible_count}");
    println!("{max_scenic_score}");
}
