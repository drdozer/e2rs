#![feature(is_some_and)]

use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;
use e2rs::{e2::E2_BOARD_SPEC, images::board_image};

/// Render a clues file to an image.
#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    /// file to read the solution from
    clues: std::path::PathBuf,
    /// file to write the board image to
    image: std::path::PathBuf,
    /// set the rotation direction to clockwise (default anti-clockwise)
    clockwise: Option<bool>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let mut solution = BufReader::new(File::open(args.clues)?);
    let mut solution_txt = String::new();
    solution.read_to_string(&mut solution_txt)?;

    let clues = E2_BOARD_SPEC
        .parse_clues(solution_txt.as_str(), args.clockwise.is_some_and(|b| b));
    let mut board = E2_BOARD_SPEC.dimensions.new_board();

    for clue in clues {
        clue.apply(&mut board);
    }

    let img = board_image(&board);
    img.save(args.image)?;

    Ok(())
}
