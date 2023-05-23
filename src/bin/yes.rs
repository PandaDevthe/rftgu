use std::env::args; // fn args()
use std::env::Args; // struct Args

fn main() {
    let argi = args();
    let y = Args::nth(&mut argi, 1);
    loop {
        println!("{}", y)
    }
}
