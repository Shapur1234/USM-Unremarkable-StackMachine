mod vm;

use std::fs;

use crate::vm::VirtualMachine;

fn main() {
    let contents = fs::read_to_string("./programms/2 - Series of squares.txt")
        .expect("Should have been able to read the file");
    println!("Raw text: {contents}");

    let mut vm = VirtualMachine::new(contents).unwrap();

    while {
        // println!("{vm:}");
        vm.step();
        !vm.endstate()
    } {}
}
