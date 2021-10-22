type Memo = [[Option<u16>; 0x10000]; 5];

fn main() {
    for r7 in 0x0..0x8000 {
        let mut memo: Memo = [[None; 0x10000]; 5];

        if recur(&mut memo, 4, 1, r7) == 6 {
            println!("MATCH on {}", r7);
            return;
        } else if r7 % 100 == 0 {
            println!("Tried {}...", r7)
        }
    }
}

fn recur(memo: &mut Memo, r0: u16, r1: u16, r7: u16) -> u16 {
    let (r0_usize, r1_usize) = (r0 as usize, r1 as usize);

    if let Some(val) = memo[r0_usize][r1_usize] {
        return val;
    }

    let result = match (r0, r1) {
        (0, r1) => r1 + 1,
        (r0, 0) => recur(memo, r0 - 1, r7, r7),
        (r0, r1) => {
            let partial = recur(memo, r0, r1-1, r7);
            recur(memo, r0 - 1 , partial, r7)
        },
    };

    memo[r0_usize][r1_usize] = Some(result);

    result
}