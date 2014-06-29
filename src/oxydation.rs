use board::Board;
mod board;

fn main() {
    let mut b = Board::new(10, 10);
    println!("{}", b);
}
