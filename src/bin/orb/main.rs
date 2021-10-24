use std::collections::VecDeque;

struct Path {
    location: usize,
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
            (6, " + 22 ".into(), |x| x + 22),
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
            (6, " + 22 ".into(), |x| x + 22),
            (6, " - 22 ".into(), |x| x - 22),
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
            (6, " - 22 ".into(), |x| x - 22),
        ],
    ];

    let start = Path {
        location: 6,
        val: 22,
        route: "".into(),
    };

    let mut paths = VecDeque::new();
    paths.push_back(start);

    loop {
        let path = paths.pop_front().expect("No paths to process!");
        let location = path.location;
        
        for neighbour in &grid[location] {
            let negcheck = neighbour.2(100);
            if negcheck < 100 && (100 - negcheck) >= path.val {
                continue;
            }

            let next: Path = Path {
                location: neighbour.0,
                route: format!("{}{}", path.route, neighbour.1),
                val: neighbour.2(path.val),
            };

            if next.location == 1 {
                println!("Reached vault with orb at value {}: {}", next.val, next.route);
                if next.val == 30 {
                    return;
                }
            }


            paths.push_back(next);
        }
    }
}
