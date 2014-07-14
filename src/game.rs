use std::{
    mem,
    fmt,
};
use board::Board;

#[deriving(Rand)]
enum Orientation {
    Horizontal,
    Vertical,
    RevHorizontal,
    RevVertical,
}

pub type GameResult<T> = Result<T, GameError>;

pub struct GameError {
    pub kind: GameErrorKind,
    pub desc: &'static str,
}

impl fmt::Show for GameError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.desc)
    }
}

pub enum GameErrorKind {
    GameOver,
}

fn error(kind: GameErrorKind, desc: &'static str) -> GameError {
    GameError { kind: kind, desc: desc }
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

    fn apply_play(&mut self, x: uint, o: Orientation) -> GameResult<()>{
        let try_set = |b: &mut Board, x: uint, y: uint, value: uint| -> GameResult<()>{
            if b.get(x, y) != 0 {
                return Err(error(GameOver, "This move ends the game"));
            }
            b.set(x, y, value);
            Ok(())
        };
        let last_line = self.b.y - 1;

        let (mut v1, mut v2) = self.current;
        match o {
            RevHorizontal | RevVertical => mem::swap(&mut v1, &mut v2),
            _ => {}
        }
        match o {
            Horizontal | RevHorizontal => {
                try!(try_set(&mut self.b, x, last_line, v1));
                try!(try_set(&mut self.b, x + 1, last_line, v2));
            }
            Vertical | RevVertical => {
                try!(try_set(&mut self.b, x, last_line, v1));
                try!(try_set(&mut self.b, x, last_line - 1, v2));
            }
        }
        Ok(())
    }

    pub fn play(&mut self, x: uint, o: Orientation) -> GameResult<()> {
        try!(self.apply_play(x, o));
        self.current = (self.b.get_random_value(),
                        self.b.get_random_value());
        Ok(self.b.evolve())
    }
}
