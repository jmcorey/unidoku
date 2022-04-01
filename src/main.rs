pub mod board;
//pub mod solver;
//pub mod gstate;
//pub mod tui;

fn main() {
    let b = crate::board::Board::new(4, 4);
    b.print();
}
