use std::{io::{BufReader, Read}, fs::File};

use clap::Parser;
use e2rs::{e2::E2_BOARD_SPEC, images::board_image};

/// Render a clues file to an image.
#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    /// file to read clues from
    clues: std::path::PathBuf,
    /// file to write the board image to
    image: std::path::PathBuf,
    /// set the rotation direction to clockwise (default anti-clockwise)
    #[arg(default_value_t = false)]
    clockwise: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let mut clue_file = BufReader::new(File::open(args.clues)?);
    let mut clue_txt = String::new();
    clue_file.read_to_string(&mut clue_txt)?;

    let clues = E2_BOARD_SPEC.tiles.parse_clues(clue_txt.as_str(), args.clockwise);
    let mut board = E2_BOARD_SPEC.dimensions.unwrap().new_board();

    for clue in clues {
        clue.apply(&mut board);
    }

    let img = board_image(&board);
    img.save(args.image)?;

    Ok(())
}