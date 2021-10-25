use crate::structs::{Op, Tile};

// reference locations are row (from top) then column (from left) both 1 indexed
pub const GRID: [Tile; 8] = [
    // (1,2) 8
    Tile::new(
        8,
        &[
            (1, Op::Sub),
            (2, Op::Mul),
            (3, Op::Sub),
            (3, Op::Mul),
            (4, Op::Mul),
        ],
    ),
    // (1,4) 1 - vault/finish
    Tile::new(1, &[(0, Op::Sub), (3, Op::Sub), (3, Op::Mul), (5, Op::Mul)]),
    // (2,1) 4
    Tile::new(4, &[(0, Op::Mul), (3, Op::Mul), (4, Op::Mul), (4, Op::Add)]),
    // (2,3) 11
    Tile::new(
        11,
        &[
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
        &[
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
        &[
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
        &[(2, Op::Add), (4, Op::Add), (4, Op::Sub), (7, Op::Sub)],
    ),
    // (4,3) 9
    Tile::new(9, &[(4, Op::Sub), (5, Op::Sub), (5, Op::Mul)]),
];
