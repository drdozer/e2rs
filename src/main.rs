//! Eternity 2-family puzzles.

#![allow(incomplete_features)]
#![feature(const_for)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(stmt_expr_attributes)]
#![warn(missing_docs)]

pub mod e2;
pub mod board;

fn main() {
    let tiles = e2::tiles();

    println!("Loaded tiles.");
    for (i, t) in tiles.into_iter().enumerate() {
        println!("{}:\t{:?}", i, t);
    }

    println!("{:?}", tiles[139 - 1]); // todo: tiles are cannonically indexed from 1, not 0
}
