mod utils;
mod system;

use system::{run_system};

fn main() {
    run_system();
    println!("System Closed!");
}
