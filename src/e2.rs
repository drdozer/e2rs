//! Types and data for the Eternity 2 Puzzle.
//! 
//! This module provides several types and type aliasses to represent the Eternity 2 Puzzle in a type-safe manner.
//! It also provides functions to load copies of the data for this specific puzzle.
//! 
//! For datastructures useful for prepresenting an arbitrary Eternity 2 style puzzle, see [crate::board].

use std::mem::transmute;

use embed_doc_image::embed_doc_image;

/// Number of columns in the Eternity 2 Puzzle.
pub const E2_COLUMNS: usize = 16;


/// Number of rows in the Eternity 2 Puzzle.
pub const E2_ROWS: usize = 16;

/// Number of tiles in the Eternity 2 Puzzle.
pub const E2_TILE_COUNT: usize = E2_COLUMNS * E2_ROWS;

/// A board configured to the Eternity 2 Puzzle specs.
pub type E2Board = crate::board::Board<E2Edge, E2_COLUMNS, E2_ROWS>;

/// A tile configured to the Eternity 2 Puzzle specs.
pub type E2Tile = crate::board::Tile<E2Edge>;

/// A tileset configured to the Eternity 2 Puzzle specs.
pub type E2TileSet = crate::board::TileSet<E2Edge, E2_TILE_COUNT>;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
/// A rust edge is either *outside* (grey) or one of the 22 two-color patterns.
/// 
/// The e2 edges are as follows:
/// ![E2 Edges][e2-edges]
#[embed_doc_image("e2-edges", "data/E2-Colors.png")]
pub enum E2Edge {
    Outside = 0,
    Edge1,
    Edge2,
    Edge3,
    Edge4,
    Edge5,
    Edge6,
    Edge7,
    Edge8,
    Edge9,
    Edge10,
    Edge11,
    Edge12,
    Edge13,
    Edge14,
    Edge15,
    Edge16,
    Edge17,
    Edge18,
    Edge19,
    Edge20,
    Edge21,
    Edge22,
}

impl Default for E2Edge {
    fn default() -> Self {
        E2Edge::Outside
    }
}

impl crate::board::Edge for E2Edge {
    fn is_border(&self) -> bool {
        match self {
            E2Edge::Outside => true,
            _ => false,
        }
    }
}

impl From<u8> for E2Edge {
    fn from(value: u8) -> Self {
        unsafe { transmute(value) }
    }
}

/// Error case for an invalid edge letter.
#[derive(Debug)]
pub struct InvalidEdge(char);

/// Parse edge letters into edges.
/// 
/// Note - the lettering matches the standardised lettering, using column-major indexing.
/// Some applications, such asthe [bucas board renderer][e2.bucas.name] use row-major indexing.
impl TryFrom<char> for E2Edge {
    // perhaps consider refactoring this so it can be generic for Edge
    type Error = InvalidEdge;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value as i32 - 'a' as i32 {
            v if v < 0 => Err(InvalidEdge(value)),
            v if v >= EDGES.len() as i32 => Err(InvalidEdge(value)),
            v => Ok(From::from(v as u8))
        }
    }
}

/// All edge types from the Eternity 2 Puzzle as an array, for easy indexing.
/// 
/// Element zero is the outside (grey) edge type.
pub const EDGES: [E2Edge; 23] = {
    use E2Edge::*;
    [
        Outside,
        Edge1,
        Edge2,
        Edge3,
        Edge4,
        Edge5,
        Edge6,
        Edge7,
        Edge8,
        Edge9,
        Edge10,
        Edge11,
        Edge12,
        Edge13,
        Edge14,
        Edge15,
        Edge16,
        Edge17,
        Edge18,
        Edge19,
        Edge20,
        Edge21,
        Edge22,
    ]
};

/// Pieces data string literal.
/// 
/// The format is exactly that expected by the `tiles()` fn.
/// 
/// Each line contains data for one tile.
/// Each tile is represented as 4 digits representing the edges in the order north, south, west east.
/// Tiles are numbered as in ![E2 Edges][e2-edges]
#[embed_doc_image("e2-edges", "data/E2-Colors.png")]
static TILE_DATA: &str = include_str!("../data/e2pieces-nesw.txt");

/// Retrieve a new copy of the Eternity 2 Puzzle tileset.
pub fn tiles() -> E2TileSet {
    use crate::board::Side::*;
    crate::board::parse_tiles::<
        E2Edge, 
        { North }, { East }, { South }, { West },
        E2_TILE_COUNT>(TILE_DATA)
}
