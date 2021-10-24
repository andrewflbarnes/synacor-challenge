use std::collections::VecDeque;

struct Path {
    location: usize,
    steps: u8,
    val: u64,
    route: String,
}

fn main() {
    // reference locations are row (from top) then column (from left) both 1 indexed
    let grid: [Vec<(usize, String, fn(u64) -> u64)>; 8] = [
        // (1,2) 8
        vec![
            (1, " - 1 ".into(), |x| x - 1),
            (2, " * 4 ".into(), |x| x * 4),
            (3, " - 11 ".into(), |x| x - 11),
            (3, " * 11 ".into(), |x| x * 11),
            (4, " * 4 ".into(), |x| x * 4),
        ],
        // (1,4) 1 - vault/finish
        vec![
            (0, " - 8 ".into(), |x| x - 8),
            (3, " - 11 ".into(), |x| x - 11),
            (3, " * 11 ".into(), |x| x * 11),
            (5, " * 18 ".into(), |x| x * 18),
        ],
        // (2,1) 4
        vec![
            (0, " * 8 ".into(), |x| x * 8),
            (3, " * 11 ".into(), |x| x * 11),
            (4, " * 4 ".into(), |x| x * 4),
            (4, " + 4 ".into(), |x| x + 4),
        ],
        // (2,3) 11
        vec![
            (0, " - 8 ".into(), |x| x - 8),
            (0, " * 8 ".into(), |x| x * 8),
            (1, " - 1 ".into(), |x| x - 1),
            (1, " * 1 ".into(), |x| x * 1),
            (2, " * 4 ".into(), |x| x * 4),
            (4, " * 4 ".into(), |x| x * 4),
            (4, " - 4 ".into(), |x| x - 4),
            (5, " * 18 ".into(), |x| x * 18),
            (5, " - 18 ".into(), |x| x - 18),
            (7, " - 9 ".into(), |x| x - 9),
        ],
        // (3,2) 4
        vec![
            (2, " + 4 ".into(), |x| x + 4),
            (2, " * 4 ".into(), |x| x * 4),
            (3, " - 11 ".into(), |x| x - 11),
            (3, " * 11 ".into(), |x| x * 11),
            (5, " - 18 ".into(), |x| x - 18),
            (7, " - 9 ".into(), |x| x - 9),
        ],
        // (3,4) 18
        vec![
            (1, " * 1 ".into(), |x| x * 1),
            (3, " - 11 ".into(), |x| x - 11),
            (3, " * 11 ".into(), |x| x * 11),
            (4, " - 4 ".into(), |x| x - 4),
            (7, " - 9 ".into(), |x| x - 9),
            (7, " * 9 ".into(), |x| x * 9),
        ],
        // (4,1) 22 - orb/start
        vec![
            (2, " + 4 ".into(), |x| x + 4),
            (4, " + 4 ".into(), |x| x + 4),
            (4, " - 4 ".into(), |x| x - 4),
            (7, " - 9 ".into(), |x| x - 9),
        ],
        // (4,3) 9
        vec![
            (4, " - 4 ".into(), |x| x - 4),
            (5, " - 18 ".into(), |x| x - 18),
            (5, " * 18 ".into(), |x| x * 18),
        ],
    ];

    let start = Path {
        location: 6,
        steps: 0,
        val: 22,
        route: "".into(),
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

        for neighbour in &grid[location] {
            // If underflow would occur ignore this path
            let negcheck = neighbour.2(100);
            if negcheck < 100 && (100 - negcheck) >= path.val {
                continue;
            }

            let next: Path = Path {
                location: neighbour.0,
                steps: path.steps + 1,
                route: format!("{}{}", path.route, neighbour.1),
                val: neighbour.2(path.val),
            };

            // if we got to the vault with a value of 30, print the
            if next.location == 1 && next.val == 30 {
                max_steps = next.steps;
                println!(
                    "Reached vault with orb at value {} in {} steps:{}",
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
