use std::{
    rand,
    num,
};
use std::iter::AdditiveIterator;
use std::rand::Rng;

use board::Board;

use game::{
    Game,
    Orientation,
    Horizontal,
    Vertical,
    RevHorizontal,
    RevVertical,
};

fn available_moves(x: uint) -> Vec<(uint, Orientation)> {
    range(0u, x - 1).map(|i| (i, Horizontal)).chain(
        range(0u, x - 1).map(|i| (i, RevHorizontal))).chain(
        range(0u, x).map(|i| (i, Vertical))).chain(
        range(0u, x).map(|i| (i, RevVertical)))
        .collect()
}

fn score(b: &Board) -> uint {
    b.tab.iter().map(|&i| num::pow(10u, i)).sum()
}

pub fn best_move(g: &Game) -> (uint, Orientation) {
    let mut moves = available_moves(g.b.x);
    let mut rng = rand::task_rng();
    rng.shuffle(moves.as_mut_slice());

    *moves.iter().max_by(|&&(x, o)| {
        let mut game: Game = g.clone();
        match game.play(x, o) {
            Ok(()) => score(&game.b),
            Err(_) => 0,
        }
    }).unwrap()
}
