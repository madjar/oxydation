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
        b.set(rng.gen_range(0u, 10u),
              9,
              rng.gen_range(1u, 10u));
        println!("{}", b);
        sleep(500);
        b.apply_gravity();
        println!("{}", b);
        sleep(500);
    }
}
