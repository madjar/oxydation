use std;
use std::rand::{task_rng, Rng};
use std::collections::hashmap::HashSet;
use std::cmp;

#[deriving(PartialEq, Eq, Clone)]
pub struct Board {
    pub x: uint,
    pub y: uint,
    pub tab: Vec<uint>,
    highest: uint,
}


impl Board {
    pub fn new(x: uint, y: uint) -> Board {
        Board { x: x, y: y, tab: Vec::from_fn(x*y, |_| 0), highest: 2 }
    }

    #[inline]
    fn check(&self, x: uint, y: uint) {
        if x >= self.x || y >= self.y {
            fail!("({}, {} is out of the board", x, y);
        }
    }

    pub fn get(&self, x: uint, y: uint) -> uint {
        self.check(x, y);
        self.tab[x + y*self.x]
    }

    pub fn set(&mut self, x: uint, y: uint, value: uint) {
        self.check(x, y);
        *self.tab.get_mut(x + y*self.x) = value;
    }

    fn apply_gravity(&mut self) {
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

    fn neighbours(&self, x: uint, y: uint) -> Vec<(uint, uint)> {
        (vec!((x+1, y), (x-1, y), (x, y+1), (x, y-1))).move_iter()
            .filter(|&(x, y)| x < self.x && y < self.y )
            .collect()
    }

    fn find_groups_from_point(&self, x: uint, y: uint, group: &mut Option<HashSet<(uint, uint)>>, seen: &mut HashSet<(uint, uint)>) {
        seen.insert((x, y));

        let value = self.get(x, y);
        if value == 0 {
            return;
        }

        for &(nx, ny) in self.neighbours(x, y).iter() {
            if seen.contains(&(nx, ny)) {
                continue;
            }
            if value == self.get(nx, ny) {
                if group.is_none() {
                    *group = Some(HashSet::new())
                }
                {
                    let group_ref = group.get_mut_ref();
                    group_ref.insert((x, y));
                    group_ref.insert((nx, ny));
                }
                self.find_groups_from_point(nx, ny, group, seen);
            }
        }
    }

    /// Returns a list of all groups of equal neighbours
    fn find_groups(&self) -> Vec<HashSet<(uint, uint)>> {
        let mut seen = HashSet::new();
        let mut groups = Vec::new();

        for x in range(0, self.x) {
            for y in range (0, self.y) {
                if !seen.contains(&(x, y)) {
                    let mut group = None;
                    self.find_groups_from_point(x, y, &mut group, &mut seen);
                    group.map(|g| groups.push(g));
                }
            }
        }

        groups
    }

    /// Transform all matching groups into elements of the next level
    fn transform_matches(&mut self) -> bool {
        let mut transformed_something = false;
        for group in self.find_groups().iter() {
            if group.len() < 3 {
                continue
            }
            let &(x, y) = group.iter().next().unwrap();
            let value = self.get(x, y);

            for &(x, y) in group.iter() {
                self.set(x, y, 0);
            }

            let &(new_x, new_y) = group.iter().min_by(|&&(x, y)| (y, x)).unwrap();
            // TODO : representation problem when reaching 10
            self.set(new_x, new_y, value + 1);
            if value >= self.highest {
                self.highest += 1;
            }

            transformed_something = true;
        }
        transformed_something
    }

    /// Fall and transform matches, rince, repeat
    pub fn evolve(&mut self) {
        let mut one_more_step = true;
        while one_more_step {
            self.apply_gravity();
            one_more_step = self.transform_matches();
        }
    }

    pub fn get_random_value(&self) -> uint {
        task_rng().gen_range(1, self.highest + 1)
    }
}

impl std::from_str::FromStr for Board {
    fn from_str(s: &str) -> Option<Board> {
        let lines: Vec<&str> = s.lines()
            .map(|line| line.trim_chars(' '))
            .rev()
            .collect();
        let y = lines.len();
        if y == 0 {
            // There must be at least one line
            return None;
        }
        let x = lines[0].len();
        for line in lines.tail().iter() {
            if line.len() != x {
                // All lines must have the same len
                return None;
            }
        }
        let mut tab = Vec::with_capacity(x*y);
        for line in lines.iter() {
            for c in line.chars() {
                match c.to_digit(10) {
                    Some(d) => tab.push(d),
                    None => return None
                }
            }
        }
        let max = cmp::max(*tab.iter().max().unwrap(), 2);
        Some(Board { x: x, y: y, tab: tab, highest: max })
    }
}

impl std::fmt::Show for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        for _ in range(0, self.x+2) {
            try!(write!(f, "-"));
        }
        try!(write!(f, "\n"));
        for y in range(0, self.y).rev() {
            try!(write!(f, "|"));
            for x in range(0, self.x) {
                let value = self.get(x, y);
                if value != 0 {
                    try!(write!(f, "{}", value));
                } else {
                    try!(write!(f, " "));
                }
            }
            try!(writeln!(f, "|"));
        }
        for _ in range(0, self.x+2) {
            try!(write!(f, "-"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::iter::FromIterator;
    use test::Bencher;
    use std::rand::random;
    use std::from_str::from_str;
    use super::Board;

    #[test]
    fn get_set() {
        let mut b = Board::new(2, 2);
        assert_eq!(b.get(0, 1), 0);
        b.set(0, 1, 42);
        assert_eq!(b.get(0, 1), 42);
    }

    #[test]
    fn board_from_str() {
        let b: Board = from_str("678
                                345
                                012").unwrap();
        for i in range(0u, 3) {
            for j in range(0u, 3) {
                assert_eq!(b.get(i, j), i + j*3);
            }
        }
    }

    #[test]
    fn gravity() {
        let mut b: Board = from_str("001
                                    230
                                    400").unwrap();
        b.apply_gravity();
        let expected: Board = from_str("000
                                       200
                                       431").unwrap();
        assert_eq!(b, expected);
    }

    #[test]
    fn find_groups() {
        let b: Board = from_str("11
                                01").unwrap();
        let expected = vec!(FromIterator::from_iter(
            [(1u, 0u), (0u, 1u), (1u, 1u)]
                .to_owned().move_iter()));

        assert_eq!(b.find_groups(), expected);
    }

    #[bench]
    fn bench_find_groups(b: &mut Bencher) {
        let mut board = Board::new(10, 10);
        for t in board.tab.mut_iter() {
            *t = random::<uint>() % 10;
        }

        b.iter(|| board.find_groups());
    }

    #[test]
    fn transform_matches() {
        let mut b: Board = from_str("104440
                                    123340
                                    122344").unwrap();
        b.transform_matches();
        let expected: Board = from_str("000000
                                       000000
                                       230450").unwrap();
        assert_eq!(b, expected);
    }
}
