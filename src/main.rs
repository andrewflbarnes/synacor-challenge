mod vm;
use vm::VirtualMachine;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut vm = VirtualMachine::new();

    // hello world
    // let program: Vec<u16> = vec![
    //     0x1500, 0x1500, 0x1300, 0x6800, 0x1300, 0x6500, 0x1300, 0x6c00, 0x1300, 0x6c00, 0x1300,
    //     0x6f00, 0x1300, 0x2000, 0x1300, 0x7700, 0x1300, 0x6f00, 0x1300, 0x7200, 0x1300, 0x6c00,
    //     0x1300, 0x6400, 0x0000,
    // ];

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("A program file must be provided");
        return;
    }

    let filename = &args[1];

    let mut f =
        File::open(filename).expect(&format!("Unable to open file for reading {}", filename));

    let mut buffer = vec![];

    f.read_to_end(&mut buffer).expect("Buffer overflow");

    let program: Vec<u16> = buffer
        .chunks_exact(2)
        .into_iter()
        .map(|a| (a[0] as u16) << 8 | a[1] as u16)
        .collect();

    vm.load(program);

    vm.run();
}
