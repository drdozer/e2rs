use std;

use std::marker::PhantomData;
use std::ops::Index;
use std::ops::RangeFull;

use super::Side;
use super::Tile;

/// The ID of a puzzle tile.
/// 
/// Tile IDs are bound to the tileset they come from.
/// They can not (or should not) be used to refer to tiles in another tileset.
/// Tiles are counted from 1, not 0 so TileID presents an API that is based-1.
pub struct TileID<'a, E>(u8, PhantomData<&'a TileSet<E>>);

impl <'a, E> From<TileID<'a, E>> for u8 {
    fn from(value: TileID<'a, E>) -> Self {
        value.0 + 1
    }
}

/// A complete tileset for an eternity-style puzzle.
///
/// Tile sets are indexed from 1 in the puzzle numbering scheme.
/// To make this work well, element 0 is a blank tile, not to be used.
#[derive(Debug)]
pub struct TileSet<E>(Vec<Tile<E>>);

impl <E> TileSet<E> {
    /// Create a new tileset, using the tiles supplied in the vector.
    /// 
    /// The zeroth element of the tiles vector will become the tile with ID 1.
    /// The tiles vector must not exceed 256 in length.
    pub fn new(tiles: Vec<Tile<E>>) -> Self {
        assert!(tiles.len() <= 256);

        TileSet(tiles)
    }

    /// Get the length of this tileset.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl <'a, E> TileSet<E> {
    /// Convert an index into a TileID.
    /// 
    /// During debug, the index is range-checked against `1..=tileset.len()`.
    /// At release, these range-checks are dropped, so please test before you deploy.
    pub fn id(&'a self, idx: usize) -> TileID<'a, E> {
        debug_assert!(idx <= self.len());
        debug_assert_ne!(idx, 0);

        TileID((idx - 1) as u8, PhantomData)
    }
}

impl <E> Index<RangeFull> for TileSet<E> {
    type Output = [Tile<E>];

    fn index(&self, index: RangeFull) -> &Self::Output {
        self.0.index(index)
    }
}

impl <'a, E> Index<TileID<'a, E>> for TileSet<E> {
    type Output = Tile<E>;

    fn index(&self, index: TileID<'a, E>) -> &Self::Output {
        unsafe {
            &self.0.get_unchecked(index.0 as usize)
        }
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

    let mut tiles = Vec::new();
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
                tiles.push(tile);
            }
            l => panic!("Bad number of digits in board file: {}", l),
        }
    }

    TileSet::new(tiles)
}
