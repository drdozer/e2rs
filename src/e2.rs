//! Types and data for the Eternity 2 Puzzle.
//! 
//! This module provides several types and type aliasses to represent the Eternity 2 Puzzle in a type-safe manner.
//! It also provides functions to load copies of the data for this specific puzzle.
//! 
//! For datastructures useful for prepresenting an arbitrary Eternity 2 style puzzle, see [crate::board].

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
static TILE_DATA: &str = include_str!("../data/e2pieces-nswe.txt");

/// Retrieve a new copy of the Eternity 2 Puzzle tileset.
pub fn tiles() -> E2TileSet {
    let blank: E2Tile = Default::default();

    let mut tiles = crate::board::TileSet([blank; E2_TILE_COUNT]);
    for (tile_number, line) in TILE_DATA.lines().enumerate() {
        // println!("`{}'", line);
        let mut digits = line
            .trim()
            .splitn(4, " ")
            // .map(|d| { println!("`{}', ", d); d})
            .map(|d| d.parse::<u8>().unwrap())
            .map(|i| EDGES[i as usize]);

        let north = digits.next().unwrap();
        let south = digits.next().unwrap();
        let west  = digits.next().unwrap();
        let east  = digits.next().unwrap();

        let tile = crate::board::Tile::new(north, east, south, west);

        tiles.0[tile_number] = tile;
    }

    tiles
}