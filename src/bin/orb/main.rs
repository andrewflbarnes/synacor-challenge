use std::collections::VecDeque;
use std::fmt;

struct Path {
    location: usize,
    steps: u8,
    val: u64,
    route: String,
}

#[derive(Eq, PartialEq)]
enum Op {
    Add,
    Mul,
    Sub,
}

impl Op {
    fn apply(&self, operand1: u64, operand2: u64) -> u64 {
        match self {
            Op::Add => operand1 + operand2,
            Op::Sub => operand1 - operand2,
            Op::Mul => operand1 * operand2,
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
        };
        write!(f, "{}", symbol)
    }
}

struct Tile {
    value: u64,
    neighbours: Vec<Neighbour>,
}

impl Tile {
    fn new(value: u64, neighbours: Vec<(usize, Op)>) -> Self {
        Tile {
            value,
            neighbours: neighbours
                .into_iter()
                .map(|(index, op)| Neighbour { index, op })
                .collect(),
        }
    }
}

struct Neighbour {
    index: usize,
    op: Op,
}

fn main() {
    // reference locations are row (from top) then column (from left) both 1 indexed
    let grid: [Tile; 8] = [
        // (1,2) 8
        Tile::new(
            8,
            vec![
                (1, Op::Sub),
                (2, Op::Mul),
                (3, Op::Sub),
                (3, Op::Mul),
                (4, Op::Mul),
            ],
        ),
        // (1,4) 1 - vault/finish
        Tile::new(
            1,
            vec![(0, Op::Sub), (3, Op::Sub), (3, Op::Mul), (5, Op::Mul)],
        ),
        // (2,1) 4
        Tile::new(
            4,
            vec![(0, Op::Mul), (3, Op::Mul), (4, Op::Mul), (4, Op::Add)],
        ),
        // (2,3) 11
        Tile::new(
            11,
            vec![
                (0, Op::Sub),
                (0, Op::Mul),
                (1, Op::Sub),
                (1, Op::Mul),
                (2, Op::Mul),
                (4, Op::Sub),
                (4, Op::Mul),
                (5, Op::Sub),
                (5, Op::Mul),
                (7, Op::Sub),
            ],
        ),
        // (3,2) 4
        Tile::new(
            4,
            vec![
                (2, Op::Add),
                (2, Op::Mul),
                (3, Op::Sub),
                (3, Op::Mul),
                (5, Op::Sub),
                (7, Op::Sub),
            ],
        ),
        // (3,4) 18
        Tile::new(
            18,
            vec![
                (1, Op::Mul),
                (3, Op::Sub),
                (3, Op::Mul),
                (4, Op::Sub),
                (7, Op::Sub),
                (7, Op::Mul),
            ],
        ),
        // (4,1) 22 - orb/start
        Tile::new(
            22,
            vec![(2, Op::Add), (4, Op::Add), (4, Op::Sub), (7, Op::Sub)],
        ),
        // (4,3) 9
        Tile::new(9, vec![(4, Op::Sub), (5, Op::Sub), (5, Op::Mul)]),
    ];

    let start = Path {
        location: 6,
        steps: 0,
        val: 22,
        route: String::from("22"),
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

        let tile = &grid[location];

        for neighbour in &tile.neighbours {
            let Neighbour {
                op,
                index: neighbour_index,
            } = neighbour;
            let Tile {
                value: neighbour_val,
                ..
            } = grid[*neighbour_index];
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
            if next.location == 1 && next.val == 30 {
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
