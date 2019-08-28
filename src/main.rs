
use log::{error};
use quantum_core::*;

fn main() {
    simple_logger::init().unwrap();

    let mut core = Quantum::new();
    match core.load_plugin("plugin") {
        Ok(name) => println!("{}", name),
        Err(err) => error!("{}", err),
    }
}
