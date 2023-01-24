use std;

use std::ops::Index;

use super::Side;
use super::Tile;

/// A complete tileset for an eternity-style puzzle.
///
/// Tile sets are indexed from 1 in the puzzle numbering scheme.
/// To make this work well, element 0 is a blank tile, not to be used.
#[derive(Debug)]
pub struct TileSet<E>(Vec<Tile<E>>);

impl<E: Default> TileSet<E> {
    /// Create a new, empty tileset.
    pub fn new() -> Self {
        let mut tiles = Vec::new();
        tiles.push(Default::default());
        TileSet(tiles)
    }

    /// Get the length of this tileset.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}


impl<E, Idx> Index<Idx> for TileSet<E>
where
    Idx: std::slice::SliceIndex<[Tile<E>]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, E> IntoIterator for &'a TileSet<E> {
    type Item = &'a Tile<E>;
    type IntoIter = <&'a Vec<Tile<E>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}


/// Parse a tiles file.
///
/// Each row is expected to contain exactly 4 numbers separated by whitespace.
/// The S1..S4 parameters specify which sides the 4 digits correspond to.
/// So if S1 is North, the first edge in a row will be an edge assigned to the north side of a tile.
pub fn parse_tiles<E, const S1: Side, const S2: Side, const S3: Side, const S4: Side>(
    txt: &str,
) -> TileSet<E>
where
    E: From<u8> + Copy + Default,
{
    let blank: Tile<E> = Default::default();

    let mut tiles = TileSet::new();
    for line in txt.lines() {
        // println!("`{}'", line);
        let digits: Vec<_> = line
            .trim()
            .split(" ")
            // .map(|d| { println!("`{}', ", d); d})
            .collect();

        match digits.len() {
            4 => {
                let mut tile = blank.clone();
                let digits: Vec<_> = digits
                    .iter()
                    .map(|d| d.parse::<u8>().unwrap())
                    .map(From::from)
                    .collect();
                tile[S1] = digits[0];
                tile[S2] = digits[1];
                tile[S3] = digits[2];
                tile[S4] = digits[3];
                tiles.0.push(tile);
            }
            l => panic!("Bad number of digits in board file: {}", l),
        }
    }

    tiles
}
