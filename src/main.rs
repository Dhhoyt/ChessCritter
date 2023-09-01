#![allow(dead_code)]

mod bot;

use bot::Board;

fn main() {
    let mut board = Board::default();
    println!("{}", board.perft(6));
}

