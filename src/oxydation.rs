extern crate test;

use game::Game;
use std::io::timer::sleep;

mod board;
mod bot;
mod game;

fn main() {
    demo();
}

fn demo() {
    let mut g = Game::new(10, 10);
    loop {
        println!("{}", g.b);
        let (x, o) = bot::best_move(&g);
        g.play(x, o).ok().expect("Game over");
        sleep(500);
    }
}
