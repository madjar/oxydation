use std::fmt;


pub struct Board {
    x: uint,
    y: uint,
    tab: Vec<uint>
}


impl Board {
    pub fn new(x: uint, y: uint) -> Board {
        Board { x: x, y: y, tab: Vec::from_fn(x*y, |_| 0) }
    }

    fn check(&self, x: uint, y: uint) {
        if x >= self.x || y >= self.y {
            fail!();
        }    
    }

    pub fn get(&self, x: uint, y: uint) -> uint {
        self.check(x, y);
        *self.tab.get(x + y*self.x)
    }

    pub fn set(&mut self, x: uint, y: uint, value: uint) {
        self.check(x, y);
        *self.tab.get_mut(x + y*self.x) = value;
    }

    pub fn apply_gravity(&mut self) {
        for x in range(0, self.x) {
            let mut last_free = 0;
            for y in range(0, self.y) {
                let value = self.get(x, y);
                if value != 0 {
                    self.set(x, y, 0);
                    self.set(x, last_free, value);
                    last_free += 1;
                }
            }
        }
    }
}

impl fmt::Show for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut result = String::new();
        for _ in range(0, self.x+2) {
            result.push_char('-');
        }
        result.push_char('\n');
        for y in range(0, self.y).rev() {
            result.push_char('|');
            for x in range(0, self.x) {
                let value = self.get(x, y);
                if value != 0 {
                    result.push_str(self.get(x, y).to_str().as_slice());
                } else {
                    result.push_char(' ');
                }
            }
            result.push_str("|\n");
        }
        for _ in range(0, self.x+2) {
            result.push_char('-');
        }
        write!(f, "{}", result)
    }
}


#[test]
fn get_set() {
    let mut b = Board::new(2, 2);
    assert_eq!(b.get(0, 1), 0);
    b.set(0, 1, 42);
    assert_eq!(b.get(0, 1), 42);
}

#[test]
fn gravity() {
    let mut b = Board::new(3, 3);
    b.set(0, 0, 3);
    b.set(0, 1, 4);
    b.set(1, 1, 6);
    b.set(2, 2, 9);
    println!("{}", b);
    b.apply_gravity();
    println!("{}", b);
    assert_eq!(b.get(0, 0), 3);
    assert_eq!(b.get(0, 1), 4);
    assert_eq!(b.get(1, 0), 6);
    assert_eq!(b.get(1, 1), 0);
    assert_eq!(b.get(2, 2), 0);
    assert_eq!(b.get(2, 0), 9);
}
