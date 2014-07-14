use std::rand;

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

pub fn best_move(g: &Game) -> (uint, Orientation) {
    let moves = available_moves(g.b.x);

    let mut rng = rand::task_rng();
    rand::sample(&mut rng, moves.move_iter(), 1).pop().unwrap()
}
