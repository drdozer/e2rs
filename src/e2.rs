//! Types and data for the Eternity 2 Puzzle.
//! 
//! This module provides several types and type aliasses to represent the Eternity 2 Puzzle in a type-safe manner.
//! It also provides functions to load copies of the data for this specific puzzle.
//! 
//! For datastructures useful for prepresenting an arbitrary Eternity 2 style puzzle, see [crate::board].

use lazy_static::lazy_static;

use std::mem::transmute;

use embed_doc_image::embed_doc_image;

use crate::board::{Board, Tile, TileSet, BoardSpec, Dimensions, Clue};

/// Number of columns in the Eternity 2 Puzzle.
pub const E2_COLUMNS: usize = 16;

/// Number of rows in the Eternity 2 Puzzle.
pub const E2_ROWS: usize = 16;

/// Dimensions of the Eternity 2 Puzzle.
pub const E2_DIMENSIONS: Dimensions = Dimensions { columns: E2_COLUMNS, rows: E2_ROWS };

/// Number of tiles in the Eternity 2 Puzzle.
pub const E2_TILE_COUNT: usize = E2_COLUMNS * E2_ROWS;

/// A board configured to the Eternity 2 Puzzle specs.
pub type E2Board = Board<E2Edge>;

/// A tile configured to the Eternity 2 Puzzle specs.
pub type E2Tile = Tile<E2Edge>;

/// A tileset configured to the Eternity 2 Puzzle specs.
pub type E2TileSet = TileSet<E2Edge>;

/// Number of edges in the Eternity 2 Puzzle specs.
pub const E2_EDGE_COUNT: usize = 23;

/// Create a new board configured for the Eternity 2 Puzzle specs.
pub fn new_e2board() -> E2Board {
    E2_DIMENSIONS.new_board()
}


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
/// A rust edge is either *outside* (grey) or one of the 22 two-color patterns.
/// 
/// The e2 edges are as follows:
/// 
/// | Number | Letter | Edge        |
/// |--------|--------|-------------|
/// | 0      | a      | ![e0][e0]   |
/// | 1      | b      | ![e1][e1]   |
/// | 2      | c      | ![e2][e2]   |
/// | 3      | d      | ![e3][e3]   |
/// | 4      | e      | ![e4][e4]   |
/// | 5      | f      | ![e5][e5]   |
/// | 6      | g      | ![e6][e6]   |
/// | 7      | h      | ![e7][e7]   |
/// | 8      | i      | ![e8][e8]   |
/// | 9      | j      | ![e9][e9]   |
/// | 10     | k      | ![e10][e10] |
/// | 11     | l      | ![e11][e11] |
/// | 12     | m      | ![e12][e12] |
/// | 13     | n      | ![e13][e13] |
/// | 14     | o      | ![e14][e14] |
/// | 15     | p      | ![e15][e15] |
/// | 16     | q      | ![e16][e16] |
/// | 17     | r      | ![e17][e17] |
/// | 18     | s      | ![e18][e18] |
/// | 19     | t      | ![e19][e19] |
/// | 20     | u      | ![e20][e20] |
/// | 21     | v      | ![e21][e21] |
/// | 22     | w      | ![e22][e22] |
#[embed_doc_image("e2-edges", "data/E2-Colors.png")]
#[embed_doc_image("e0", "data/edge_images/0.png")]
#[embed_doc_image("e1", "data/edge_images/1.png")]
#[embed_doc_image("e2", "data/edge_images/2.png")]
#[embed_doc_image("e3", "data/edge_images/3.png")]
#[embed_doc_image("e4", "data/edge_images/4.png")]
#[embed_doc_image("e5", "data/edge_images/5.png")]
#[embed_doc_image("e6", "data/edge_images/6.png")]
#[embed_doc_image("e7", "data/edge_images/7.png")]
#[embed_doc_image("e8", "data/edge_images/8.png")]
#[embed_doc_image("e9", "data/edge_images/9.png")]
#[embed_doc_image("e10", "data/edge_images/10.png")]
#[embed_doc_image("e11", "data/edge_images/11.png")]
#[embed_doc_image("e12", "data/edge_images/12.png")]
#[embed_doc_image("e13", "data/edge_images/13.png")]
#[embed_doc_image("e14", "data/edge_images/14.png")]
#[embed_doc_image("e15", "data/edge_images/15.png")]
#[embed_doc_image("e16", "data/edge_images/16.png")]
#[embed_doc_image("e17", "data/edge_images/17.png")]
#[embed_doc_image("e18", "data/edge_images/18.png")]
#[embed_doc_image("e19", "data/edge_images/19.png")]
#[embed_doc_image("e20", "data/edge_images/20.png")]
#[embed_doc_image("e21", "data/edge_images/21.png")]
#[embed_doc_image("e22", "data/edge_images/22.png")]
pub enum E2Edge {
    /// The outside of the puzzle.
    Outside = 0,
    /// An internal edge.
    Edge1,
    /// An internal edge.
    Edge2,
    /// An internal edge.
    Edge3,
    /// An internal edge.
    Edge4,
    /// An internal edge.
    Edge5,
    /// An internal edge.
    Edge6,
    /// An internal edge.
    Edge7,
    /// An internal edge.
    Edge8,
    /// An internal edge.
    Edge9,
    /// An internal edge.
    Edge10,
    /// An internal edge.
    Edge11,
    /// An internal edge.
    Edge12,
    /// An internal edge.
    Edge13,
    /// An internal edge.
    Edge14,
    /// An internal edge.
    Edge15,
    /// An internal edge.
    Edge16,
    /// An internal edge.
    Edge17,
    /// An internal edge.
    Edge18,
    /// An internal edge.
    Edge19,
    /// An internal edge.
    Edge20,
    /// An internal edge.
    Edge21,
    /// An internal edge.
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

lazy_static! {
    /// The board spec for the Eternity 2 Puzzle.
    pub static ref E2_BOARD_SPEC: BoardSpec<E2Edge> = board_spec();
}

/// Retrieve a new copy of the Eternity 2 Puzzle tileset.
pub fn board_spec() -> BoardSpec<E2Edge> {
    use crate::board::Side::*;
    let bs = crate::board::parse_tiles::<
        E2Edge, 
        { North }, { East }, { South }, { West }>(TILE_DATA);
    assert_eq!(bs.tiles.len(), E2_TILE_COUNT + 1); // +1 for the blank tile at element 0
    match bs.dimensions {
        None => BoardSpec { dimensions: Some(E2_DIMENSIONS), tiles: bs.tiles },
        Some(_) => bs,
    }
}

/// E2 clues data string literal.
static CLUE_DATA: &str = include_str!("../data/e2clues.txt");

lazy_static! {
    /// The five Eternity 2 Puzzle clues.
    pub static ref E2_CLUES: Vec<Clue<E2Edge>> = E2_BOARD_SPEC.tiles.parse_clues(CLUE_DATA, false );
}