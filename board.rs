use std::fmt;
use std::strbuf::StrBuf;


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
        for y in range(0, self.y) {
            let mut last_free = 0;
            for x in range(0, self.x) {
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
        let mut result = StrBuf::new();
        for _ in range(0, self.x+2) {
            result.push_char('-');
        }
        result.push_char('\n');
        for y in range(0, self.y).rev() {
            result.push_char('|');
            for x in range(0, self.x) {
                let value = self.get(x, y);
                if value != 0 {
                    result.push_str(self.get(x, y).to_str());
                } else {
                    result.push_char(' ');
                }
            }
            result.push_str("|\n");
        }
        for _ in range(0, self.x+2) {
            result.push_char('-');
        }
        write!(f.buf, "{}", result)
    }
}
