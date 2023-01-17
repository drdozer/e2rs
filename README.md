# e2rs

Code for working with edge-matching puzzles in the Eternity 2 family.

This is a WIP sketch of some APIs and algs for representing and manipulating edge-matching puzzles.
We attempt to keep the representations light-weight and performant where possible, but stress readability over performance where they conflict.

## CLI

### render_solution

    Render a clues file to an image

    Usage: render_solution <CLUES> <IMAGE> [CLOCKWISE]

    Arguments:
    <SOLUTION>   file to read the solution from
    <IMAGE>      file to write the board image to
    [CLOCKWISE]  set the rotation direction to clockwise (default anti-clockwise)

    Options:
    -h, --help     Print help
    -V, --version  Print version

If you are running this from cargo, then this is an example command that should generate an image of the standard clues for the standard puzzle.

    cargo run -r --bin render_solution -- data/e2clues.txt clues.png

If you are using the pre-built cli binaries directly then instead use:

    render_solution data/e2clues.txt clues.jpg

Select the image format by using the appropriate file extension.
