use std::mem;

use board::Board;

#[deriving(Rand)]
enum Orientation {
    Horizontal,
    Vertical,
    RevHorizontal,
    RevVertical,
}

pub struct Game {
    pub b: Board,
    current: (uint, uint),
}

impl Game {
    pub fn new(x: uint, y: uint) -> Game {
        let b = Board::new(x, y);
        let current = (b.get_random_value(),
                       b.get_random_value());
        Game { b: b, current: current }
    }

    pub fn get_current(&self) -> (uint, uint) {
        self.current
    }

    fn apply_play(&mut self, x: uint, o: Orientation) {
        // TODO: handle defeat
        let try_set = |b: &mut Board, x: uint, y: uint, value: uint| {
            if b.get(x, y) != 0 {
                fail!("Game lost");
            }
            b.set(x, y, value);
        };
        let last_line = self.b.y - 1;

        let (mut v1, mut v2) = self.current;
        match o {
            RevHorizontal | RevVertical => mem::swap(&mut v1, &mut v2),
            _ => {}
        }
        match o {
            Horizontal | RevHorizontal => {
                try_set(&mut self.b, x, last_line, v1);
                try_set(&mut self.b, x + 1, last_line, v2);
            }
            Vertical | RevVertical => {
                try_set(&mut self.b, x, last_line, v1);
                try_set(&mut self.b, x, last_line - 1, v2);
            }
        }

    }

    pub fn play(&mut self, x: uint, o: Orientation) {
        self.apply_play(x, o);
        self.current = (self.b.get_random_value(),
                        self.b.get_random_value());
        self.b.evolve()
    }
}
