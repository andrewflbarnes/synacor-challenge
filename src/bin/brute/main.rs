use std::collections::HashMap;

fn main() {
    let mut memo: HashMap<(u16, u16), u16> = HashMap::new();

    for r7 in 0x800..0x8000 {
        memo.clear();
        if recur(&mut memo, 4, 1, r7) == 6 {
            println!("MATCH on {}", r7);
            return;
        } else {
            println!("NO MATCH on {}", r7)
        }
    }
}

fn recur(memo: &mut HashMap<(u16, u16), u16>, r0: u16, r1: u16, r7: u16) -> u16 {
    if let Some(val) = memo.get(&(r0, r1)) {
        return *val;
    }

    let result = match (r0, r1) {
        (0, r1) => r1 + 1,
        (r0, 0) => recur(memo, r0 - 1, r7, r7),
        (r0, r1) => {
            let partial = recur(memo, r0, r1-1, r7);
            recur(memo, r0 - 1 , partial, r7)
        },
    };

    memo.insert((r0, r1), result);

    result
}