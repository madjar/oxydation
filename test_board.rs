mod board;

#[test]
fn get_set() {
    let mut b = board::Board::new(2, 2);
    assert_eq!(b.get(0, 1), 0);
    b.set(0, 1, 42);
    assert_eq!(b.get(0, 1), 42);
}

#[test]
fn gravity() {
    let mut b = board::Board::new(3, 3);
    b.set(0, 0, 3);
    b.set(1, 1, 6);
    b.set(2, 2, 9);
    println!("{}", b);
    b.apply_gravity();
    println!("{}", b);
    assert_eq!(b.get(0, 0), 3);
    assert_eq!(b.get(1, 0), 6);
    assert_eq!(b.get(1, 1), 0);
    assert_eq!(b.get(2, 2), 0);
    assert_eq!(b.get(2, 0), 9);
}
