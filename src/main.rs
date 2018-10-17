//#![feature(associated_consts)]
#[macro_use]
extern crate imgui;
mod motorino;
use motorino::Motorino;

fn main() {
    println!("Starting up!");
    let mut motorino = Motorino::new();
    motorino.run();
}
