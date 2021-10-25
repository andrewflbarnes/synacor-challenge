use std::collections::VecDeque;
mod structs;
use structs::{Op, Path, Tile};
mod grid;
use grid::GRID;

fn main() {
    let start_location = 6;
    let finish_location = 1;
    let finish_value = 30;

    let Tile { value, .. } = &GRID[start_location];

    let start = Path {
        location: start_location,
        steps: 0,
        val: *value,
        route: value.to_string(),
    };

    let mut paths = VecDeque::new();
    paths.push_back(start);

    let mut max_steps = 100;

    loop {
        if paths.len() == 0 {
            return;
        }

        let path = paths.pop_front().expect("No paths to process!");
        let location = path.location;

        // no need to check any further steps on this path which would be >= max_steps + 1
        // edge case where we have two matches on a row as we go from n steps to n+1 steps
        if path.steps >= max_steps {
            continue;
        }

        let tile = &GRID[location];

        for neighbour in tile.neighbours {
            let (neighbour_index, op) = neighbour;
            let Tile {
                value: neighbour_val,
                ..
            } = GRID[*neighbour_index];
            let Path { val: path_val, .. } = path;

            // If underflow would occur ignore this path
            if *op == Op::Sub && neighbour_val >= path_val {
                continue;
            }

            let next: Path = Path {
                location: *neighbour_index,
                steps: path.steps + 1,
                route: format!("{} {} {}", path.route, op, neighbour_val),
                val: op.apply(path_val, neighbour_val),
            };

            // if we got to the vault with a value of 30, print the
            if next.location == finish_location && next.val == finish_value {
                max_steps = next.steps;
                println!(
                    "Reached vault with orb at value {} in {} steps: {}",
                    next.val, max_steps, next.route
                );
                continue;
            }

            // no need to check any further steps on this path which would be >= max_steps + 1
            if path.steps >= max_steps {
                continue;
            }

            paths.push_back(next);
        }
    }
}
