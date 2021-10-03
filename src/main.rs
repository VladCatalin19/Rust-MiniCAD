
mod application;
mod shape_factory;
mod shape_visitor;
mod shapes;
mod utils;

use std::env;
use std::vec::Vec;
use application::application::Application;

// http://elf.cs.pub.ro/poo/arhiva/teme/2017/tema2
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <input shapes file> <output image path>", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];
    match Application::run(&input_file, &output_file) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };
}
