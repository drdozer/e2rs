//! Eternity 2-family puzzles.

#![allow(incomplete_features)]
#![feature(const_for)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(stmt_expr_attributes)]
#![warn(missing_docs)]

pub mod board;
pub mod e2;
pub mod images;

fn main() {
    let board = e2::tiles();
    let tiles = &board.tiles;

    println!("Loaded tiles.");
    for (i, t) in tiles.into_iter().enumerate() {
        println!("{}:\t{:?}", i, t);
    }

    let clue = tiles[139 - 1];
    println!("{:?}", clue); // todo: tiles are cannonically indexed from 1, not 0

    for r in board::ROTATIONS {
        let rt = clue.rotate(r).apply();
        println!("{:?}: {:?}", r, rt);
    }
}
