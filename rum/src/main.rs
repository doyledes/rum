use rum::machine;
use std::env;

fn main() {
    let filename = env::args().nth(1).expect("Usage: rum progname");
    let instructions = machine::boot(&filename);
    machine::run(instructions);
}
