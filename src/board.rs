//! Representation of a generic Eternity 2 style puzzle.
//! 
//! This includes tiles, puzzle boards, clues and other commonly-needed types.
//! Applications coded against these APIs will be able to work with any puzzle.
//! To work specifically with the Eternith 2 Puzzle, look in [crate::e2] for specialised types
//! and data.

use std::{ops::{Index, IndexMut}, usize};


/// A tile is 4 edges, ordered as North, East, South, West, or if you prefer,
/// clockwise starting at the top.
#[derive(Debug, Clone, Copy, Default)]
pub struct Tile<E>([E;4]);

impl <E> Tile<E> {
    /// Make a new tile, providing the edges in the order of the parameter names.
    pub fn new(north: E, east: E, south: E, west: E) -> Tile<E> {
        Tile([north, east, south, west])
    }
}


/// A complete tileset for an eternity-style puzzle.
#[derive(Debug)]
pub struct TileSet<E, const TILE_COUNT: usize>(pub [Tile<E>; TILE_COUNT]);

impl <E, const TILE_COUNT: usize> Index<usize> for TileSet<E, TILE_COUNT> {
    type Output = Tile<E>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0.index(index)
    }
}

impl <'a, E, const TILE_COUNT: usize> IntoIterator for &'a TileSet<E, TILE_COUNT> {
    type Item = &'a Tile<E>;
    type IntoIter = <&'a [Tile<E>; TILE_COUNT] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

/// The rotation of a tile.
/// 
/// When a tile is rotated, the edges shift around in a cycle.
/// For example, Rotation::Clockwise will map north to east, east to south, south to west and west to north.
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Rotation {
    Zero = 0,
    Clockwise = 1,
    Flip = 2,
    Anti = 3,
}

/// A tile with a rotation.
/// 
/// The underlying tile is unaltered. 
#[derive(Debug, Clone, Copy)]
pub struct RotatedTile<E> {
    pub tile: Tile<E>,
    pub rotation: Rotation,
}

impl <E: Copy> RotatedTile<E> {
    /// Apply the tile rotation to yield a new tile with the edges rotated in-place.
    pub fn apply(&self) -> Tile<E> {
        let Tile([north, east, south, west]) = self.tile;
        match self.rotation {
            Rotation::Zero      => Tile::new(north, east, south, west),
            Rotation::Clockwise => Tile::new(west, north, east, south),
            Rotation::Flip      => Tile::new(south, west, north, east),
            Rotation::Anti      => Tile::new(east, south, west, north),
        }
    }
}

/// A (partially filled) board.
#[derive(Debug, Clone, Copy)]
pub struct Board<E, const COLUMNS: usize, const ROWS: usize>([Option<E>; COLUMNS * ROWS])
where [(); COLUMNS * ROWS]:;

impl <E, const COLUMNS: usize, const ROWS: usize> Board<E, COLUMNS, ROWS>
where [(); COLUMNS * ROWS]: {
    fn indx(c: usize, r: usize) -> usize {
        debug_assert!(c < COLUMNS);
        debug_assert!(r < ROWS);

        // this should compile down to (c | r >> 4) or equivalent
        c + r * COLUMNS
    }

    fn columns() -> std::ops::Range<usize> {
        0..COLUMNS
    }

    fn rows() -> std::ops::Range<usize> {
        0..ROWS
    }
}

impl <E, const COLUMNS: usize, const ROWS: usize> Index<(usize, usize)> for Board<E, COLUMNS, ROWS>
where [(); COLUMNS * ROWS]: {
    type Output = Option<E>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (c, r) = index;
        &self.0[Board::<E, COLUMNS, ROWS>::indx(c, r)]
    }
}

impl <E, const COLUMNS: usize, const ROWS: usize> IndexMut<(usize, usize)> for Board<E, COLUMNS, ROWS>
where [(); COLUMNS * ROWS]: {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (c, r) = index;
        &mut self.0[Board::<E, COLUMNS, ROWS>::indx(c, r)]
    }
}

/// A clue, giving the tile, its rotation and it's position within the puzzle.
#[derive(Clone, Copy, Debug)]
pub struct Clue<E> {
    pub tile: Tile<E>,
    pub rotation: Rotation,
    pub at: Indx,
}

/// A location within a board.
#[derive(Clone, Copy, Debug)]
pub struct Indx {
    pub col: usize,
    pub row: usize,
}

// If we get really keen, we can make indexing use opaque Col, Row, Cell structs and then
// guarantee that unsafe lookups are in bounds.
// This would require some boilerplate to make ergonomic, so a job for later.