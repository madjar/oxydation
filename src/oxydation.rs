extern crate test;

use game::Game;
use std::rand::{task_rng, Rng, random};
use std::io::timer::sleep;

mod board;
mod game;

fn main() {
    demo();
}

fn demo() {
    let mut g = Game::new(10, 10);
    let mut rng = task_rng();
    loop {
        println!("{}", g.b);
        g.play(rng.gen_range(0, 9), random()).ok().expect("Game over");
        sleep(500);
    }
}
