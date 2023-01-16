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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Side {
    /// The North, top side.
    North = 0,
    /// The East, right side.
    East,
    /// The South, bottom side.
    South,
    /// The West, left side.
    West,
}

impl Side {
    /// Flip the direction of the side, north <-> south and east <-> west.
    pub const fn flip(self) -> Self {
        let s = self as usize;
        let s = s + 2;
        unsafe { transmute(s % 4) }
    }
}

/// All [Side] values, in order.
pub const SIDES: [Side;4] = [
    Side::North,
    Side::East,
    Side::South,
    Side::West,
];

/// Shared operations on things that are edges.
pub trait Edge {
    /// Check if the edge is a border, that must be placed to the outside of the puzzle.
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


impl <E> Index<Side> for Tile<E> {
    type Output = E;
    fn index(&self, index: Side) -> &Self::Output {
        &self.edges[index as usize]
    }
}

impl <E> IndexMut<Side> for Tile<E> {
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        &mut self.edges[index as usize]
    }
}


/// A complete tileset for an eternity-style puzzle.
/// 
/// Tile sets are indexed from 1 in the puzzle numbering scheme.
/// To make this work well, element 0 is a blank tile, not to be used.
#[derive(Debug)]
pub struct TileSet<E>(Vec<Tile<E>>);

impl <E: Default> TileSet<E> {
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

impl <E: Copy> TileSet<E> {
    /// Parse a clues file.
    /// 
    /// The format is:
    /// <tile number> <column> <row> <rotation>
    /// 
    /// If the clockwise flag is true, the rotation is taken to be clockwise.
    /// If false, anti-clockwise.
    pub fn parse_clues(&self, txt: &str, clockwise: bool) -> Vec<Clue<E>> {
        let mut clues: Vec<_> = Vec::new();

        for line in txt.lines() {
            let digits: Vec<_> = line
                .trim()
                .split(" ")
                .map(|d| d.parse::<usize>().unwrap())
                .collect();
            let tile = self[digits[0]];
            let col = digits[1];
            let row = digits[2];
            let at = Indx { col, row };
            let mut rotation= ROTATIONS[digits[3]];
            if clockwise {
                rotation = rotation.reverse();
            }

            clues.push(Clue{ tile, rotation, at })
        }

        clues


    }
}

impl <E, Idx> Index<Idx> for TileSet<E>
where
    Idx: std::slice::SliceIndex<[Tile<E>]> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

// impl <E> Index<usize> for TileSet<E> {
//     type Output = Tile<E>;
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.0.index(index)
//     }
// }

impl <'a, E> IntoIterator for &'a TileSet<E> {
    type Item = &'a Tile<E>;
    type IntoIter = <&'a Vec<Tile<E>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}


/// The rotation of a tile.
/// 
/// When a tile is rotated, the edges shift around in a cycle, conter-clockwise.
/// For example, Rot90 will make the new north the old east, the new east the old south and so on.
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Rotation {
    /// No rotation
    Rot0 = 0,
    /// Rotation 90 degrees counter-clockwise.
    Rot90,
    /// Rotation 180 degrees (flip).
    Rot180,
    /// Rotation 270 degrees counter-clockwise, 90 degrees clockwise.
    Rot270,
}

impl Rotation {
    /// Reverse a rotation.
    /// 
    /// Effectively this translates between clockwise and anti-clockwise rotations.
    pub fn reverse(self) -> Self {
        // fixme: there's probably a simple arithmetic trick for this,
        // which may or may not be quicker than the lookup
        use Rotation::*;
        match self {
            Rot0 => Rot0,
            Rot90 => Rot270,
            Rot180 => Rot180,
            Rot270 => Rot90,
        }
    }
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
    /// The tile being rotated.
    pub tile: Tile<E>,
    /// The rotation to apply.
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
/// 
/// Each cell is empty, or contains a tile with the specified edge type.
#[derive(Debug, Clone)]
pub struct Board<E> {
    /// Number of columns in the board (its width).
    pub cols: usize,

    /// Number of rows in the board (its height).
    pub rows: usize,

    /// The squares on the board.
    squares: Vec<Option<Tile<E>>>
}

impl <E: Clone> Board<E> {
    /// Create a new, empty board.
    fn new(cols: usize, rows: usize) -> Board<E> {
        Board {
            cols, rows,
            squares: vec![None; cols * rows]
        }
    }
}

impl <E> Board<E> {
        fn indx(&self, c: usize, r: usize) -> usize {
        debug_assert!(c < self.cols);
        debug_assert!(r < self.rows);

        let idx = c + r * self.cols;
        // println!("indx: {},{}->{}", c, r, idx);
        idx
    }
}

impl <E> Index<(usize, usize)> for Board<E> {
    type Output = Option<Tile<E>>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (c, r) = index;
        let index = self.indx(c, r);
        unsafe {
            self.squares.get_unchecked(index)
        }
    }
}

impl <E> Index<Indx> for Board<E> {
    type Output = <Board<E> as Index<(usize, usize)>>::Output;

    fn index(&self, index: Indx) -> &Self::Output {
        self.index((index.col, index.row))
    }
}

impl <E> IndexMut<(usize, usize)> for Board<E> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (c, r) = index;
        let index = self.indx(c, r);
        unsafe {
            self.squares.get_unchecked_mut(index)
        }
    }
}

impl <E> IndexMut<Indx> for Board<E> {
    fn index_mut(&mut self, index: Indx) -> &mut Self::Output {
        self.index_mut((index.col, index.row))
    }
}

/// A board specification.
/// 
/// This is composed from an optional board dimensions, and a tileset.
pub struct BoardSpec<E> {
    /// The specified dimensions of boards.
    pub dimensions: Option<Dimensions>,

    /// The tileset to fill boards with.
    pub tiles: TileSet<E>,
}


/// A clue, giving the tile, its rotation and it's position within the puzzle.
#[derive(Clone, Copy, Debug)]
pub struct Clue<E> {
    /// The clue tile.
    pub tile: Tile<E>,
    /// How to rotate the clue tile.
    pub rotation: Rotation,
    /// Where to place the clue tile.
    pub at: Indx,
}

impl <E: Copy + std::fmt::Debug> Clue<E> {
    /// Apply a clue to a board.
    pub fn apply(&self, board: &mut Board<E>) {
        let rott = self.tile.rotate(self.rotation);
        // println!("Writing rotated tile {:?} at {:?}", rott, self.at);
        board[self.at] = Some(rott.apply());
    }
}

/// A location within a board.
#[derive(Clone, Copy, Debug)]
pub struct Indx {
    /// Column position.
    pub col: usize,
    /// Row position.
    pub row: usize,
}

/// The dimensions of a board.
pub struct Dimensions {
    /// Column count
    pub columns: usize,
    /// Row count
    pub rows: usize,
}

impl Dimensions {
    /// Make a new, blank board with the specified dimensions.
    pub fn new_board<E: Clone>(&self) -> Board<E> {
        Board::new(self.columns, self.rows)
    }
}

/// Parse a tiles file.
/// 
/// Each row is expected to contain exactly 4 numbers separated by whitespace.
/// The S1..S4 parameters specify which sides the 4 digits correspond to.
/// So if S1 is North, the first edge in a row will be an edge assigned to the north side of a tile.
pub fn parse_tiles<E, const S1: Side, const S2: Side, const S3: Side, const S4: Side>
    (txt: &str) -> BoardSpec<E>
where E: From<u8> + Copy + Default 
{
    let blank: Tile<E> = Default::default();

    let mut tiles = TileSet::new();
    let mut dimensions = None;
    for line in txt.lines() {
        // println!("`{}'", line);
        let digits: Vec<_> = line
            .trim()
            .split(" ")
            // .map(|d| { println!("`{}', ", d); d})
            .collect();

        match digits.len() {
            0 => (),
            1 => dimensions = {
                let column_count = digits[0].parse().unwrap();
                Some(Dimensions { columns: column_count, rows: column_count } )
            },
            2 => dimensions = {
                let column_count = digits[0].parse().unwrap();
                let row_count = digits[1].parse().unwrap();
                Some(Dimensions { columns: column_count, rows: row_count } )
            },
            4 => {
                let mut tile = blank.clone();
                let digits: Vec<_> = digits.iter()
                    .map(|d| d.parse::<u8>().unwrap())
                    .map(From::from).collect();
                tile[S1] = digits[0];
                tile[S2] = digits[1];
                tile[S3] = digits[2];
                tile[S4] = digits[3];
                tiles.0.push(tile);
            },
            l => panic!("Bad number of digits in board file: {}", l)
        }
    }

    BoardSpec{ dimensions, tiles }
}
