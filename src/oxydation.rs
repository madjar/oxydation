extern crate test;

use board::Board;
use std::rand::{task_rng, Rng};
use std::io::timer::sleep;

mod board;

fn main() {
    demo();
}

fn demo() {
    let mut b = Board::new(10, 10);
    let mut rng = task_rng();
    loop {
        let value = b.get_random_value();
        b.set(rng.gen_range(0u, 10u),
              9,
              value);
        println!("{}", b);
        sleep(500);
        b.evolve();
        println!("{}", b);
        sleep(500);
    }
}
