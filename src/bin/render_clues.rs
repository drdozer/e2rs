use clap::Parser;

/// Render a clues file to an image.
#[derive(Parser, Debug)]
#[command(author, version)]
struct Cli {
    /// file to read clues from
    clues: std::path::PathBuf,
    /// file to write the board image to
    image: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    

    Ok(())
}