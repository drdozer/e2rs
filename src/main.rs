//! Eternity 2-family puzzles.

#![allow(incomplete_features)]
#![feature(const_for)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(stmt_expr_attributes)]
#![warn(missing_docs)]

use rand::{distributions::{Slice, Uniform}, prelude::Distribution};

use crate::{board::{Clue, ROTATIONS, Indx}, e2::E2_CLUES};

pub mod board;
pub mod e2;
pub mod images;

fn main() {
    let spec = e2::board_spec();
    let tiles = &spec.tiles;
    let dims = spec.dimensions.unwrap();

    println!("Loaded tiles.");
    for (i, t) in tiles.into_iter().enumerate() {
        println!("{}:\t{:?}", i, t);
    }

    let clue = tiles[139];
    println!("{:?}", clue); // todo: tiles are cannonically indexed from 1, not 0

    for r in board::ROTATIONS {
        let rt = clue.rotate(r).apply();
        println!("{:?}: {:?}", r, rt);
    }

    let mut rng = rand::thread_rng();
    let r_tile = Slice::new(&tiles[..]).unwrap().map(Clone::clone);
    let r_col= Uniform::new(0, dims.columns);
    let r_row = Uniform::new(0, dims.rows);
    let r_rot = Slice::new(&ROTATIONS).unwrap().map(Clone::clone);

    println!("Creating a blank board");
    let mut rand_board = dims.new_board();

    for _ in 1..20 {
        let clue = Clue {
            tile: r_tile.sample(&mut rng),
            rotation: r_rot.sample(&mut rng),
            at: Indx { col: r_col.sample(&mut rng), row: r_row.sample(&mut rng) }
        };
        println!("Applying clue: {:?}", clue);

        clue.apply(&mut rand_board);
    }
    println!("Built randomised board.");
    let rand_img = images::board_image(&rand_board);
    println!("Constructed board image.");
    rand_img.save("randomised_board.png").unwrap();
    println!("Saved image to file");

    println!("Creating clue board");
    let mut clue_board = dims.new_board();
    for clue in E2_CLUES.iter() {
        clue.apply(&mut clue_board);
    }
    let clue_img = images::board_image(&clue_board);
    clue_img.save("clues.png").unwrap();
}
