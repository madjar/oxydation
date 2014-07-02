use std::fmt;
use std::collections::hashmap::{
    HashSet,
};
use std::iter::FromIterator;
use test::Bencher;
use std::rand::random;

pub struct Board {
    x: uint,
    y: uint,
    tab: Vec<uint>
}


impl Board {
    pub fn new(x: uint, y: uint) -> Board {
        Board { x: x, y: y, tab: Vec::from_fn(x*y, |_| 0) }
    }

    #[inline]
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

#[test]
fn find_groups() {
    let mut b = Board::new(3,3);
    b.set(1, 0, 1);
    b.set(0, 1, 1);
    b.set(1, 1, 1);
    let expected = vec!(FromIterator::from_iter([(1u, 0u), (0u, 1u), (1u, 1u)].to_owned().move_iter()));

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
