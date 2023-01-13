//! Representation of a generic Eternity 2 style puzzle.
//! 
//! This includes tiles, puzzle boards, clues and other commonly-needed types.
//! Applications coded against these APIs will be able to work with any puzzle.
//! To work specifically with the Eternith 2 Puzzle, look in [crate::e2] for specialised types
//! and data.

use std::{ops::{Index, IndexMut}, usize, mem::transmute};

/// The four sides of a tile.
/// 
/// Sides are identified by their compas cardinalities.
/// North/south point up/down in columns.
/// East/west point left/right in rows.
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Side {
    North = 0,
    East,
    South,
    West,
}

/// All [Side] values, in order.
pub const SIDES: [Side;4] = [
    Side::North,
    Side::East,
    Side::South,
    Side::West,
];

pub trait Edge {
    fn is_border(&self) -> bool;
}


/// A tile is 4 edges, ordered as North, East, South, West, or if you prefer,
/// clockwise starting at the top.
/// 
/// The tile edges can be accessed by their side directly:
/// 
/// ```
/// assert_eq!(tile.edges[0], tile[Sides::North]);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Tile<E> { edges: [E;4] }

impl <E> Tile<E> {
    /// Make a new tile, providing the edges in the order of the parameter names.
    pub fn new(north: E, east: E, south: E, west: E) -> Tile<E> {
        Tile { edges: [north, east, south, west] }
    }

    /// Rotate this tile.
    pub fn rotate(self, rotation: Rotation) -> RotatedTile<E> {
        RotatedTile { tile: self, rotation }
    }
}

impl <E: Edge> Tile<E> {
    fn count_border(&self) -> usize {
        // this doesn't compile
        /*self.edges.iter().filter(Edge::is_border).count()*/
        // this also fails to compile, which I think is the direct desugaring of the above
        /*self.edges.iter().filter(|e| Edge::is_border(e)).count()*/
        // so instead we use an ugly closure

        self.edges.iter().filter(|e| e.is_border()).count()
    }

    /// Test if this tile is a corner piece
    /// 
    /// It assumes that there are never pieces with two opposite outside edges (e.g. N/S or E/W).
    pub fn is_corner(&self) -> bool {
        self.count_border() == 2
    }

    /// Test if this tile is an edge piece
    pub fn is_edge(&self) -> bool {
        self.count_border() == 1
    }

    /// Test if this tile goes on the outside border.
    /// 
    /// The border pieces are all corners and edges.
    pub fn is_border(&self) -> bool {
        self.count_border() > 0
    }
}


impl <E: Edge> Index<Side> for Tile<E> {
    type Output = E;
    fn index(&self, index: Side) -> &Self::Output {
        &self.edges[index as usize]
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
    Rot0 = 0,
    Rot90,
    Rot180,
    Rot270,
}

impl std::ops::Add for Rotation {
    type Output = Rotation;
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self as usize;
        let rhs = rhs as usize;
        let sum = (lhs + rhs) % 4;
        
        unsafe { transmute(sum) }
    }
}

/// All [Rotation] values, in order.
pub const ROTATIONS: [Rotation;4] = [
    Rotation::Rot0,
    Rotation::Rot90,
    Rotation::Rot180,
    Rotation::Rot270,
];

/// A tile with a rotation.
/// 
/// The underlying tile is unaltered. 
/// The rotated tile edges can be accessed by their side directly, taking into account the rotation:
/// 
/// ```
/// let tile = Tile::new(1,2,3,4);
/// let rott = tile.rotate(Rotation::Rot0);
/// assert_eq!(tile[Sides::West], rott[Sides::South]);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RotatedTile<E> {
    pub tile: Tile<E>,
    pub rotation: Rotation,
}

impl <E> RotatedTile<E> {
    /// Rotate this rotated tile.
    /// 
    /// The result is a new rotated tile that composes this rotation with the pre-existing one.
    pub fn rotate(self, rotation: Rotation) -> RotatedTile<E> {
        RotatedTile {
            tile: self.tile,
            rotation: self.rotation + rotation
        }
    }

}

impl <E: Copy> RotatedTile<E> {
    /// Apply the tile rotation to yield a new tile with the edges rotated in-place.
    pub fn apply(&self) -> Tile<E> {
        let Tile { edges: [north, east, south, west] } = self.tile;
        match self.rotation {
            Rotation::Rot0   => Tile::new(north, east, south, west),
            Rotation::Rot90  => Tile::new(west,  north, east,  south),
            Rotation::Rot180 => Tile::new(south, west,  north, east),
            Rotation::Rot270 => Tile::new(east,  south, west,  north),
        }
    }
}

impl <E> Index<Side> for RotatedTile<E> {
    type Output = E;

    fn index(&self, index: Side) -> &Self::Output {
        let i = index as usize + self.rotation as usize;
        let i = i % 4;
        &self.tile.edges[i]
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