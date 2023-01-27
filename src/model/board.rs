use std::ops::IndexMut;

use std::ops::Index;

use super::ROTATIONS;
use super::Rotate;
use super::Rotation;
use super::Tile;
use super::TileSet;

/// A (partially filled) board.
///
/// Each cell is empty, or contains a tile with the specified edge type.
#[derive(Debug)]
pub struct Board<E> {
    /// Number of columns in the board (its width).
    pub columns: usize,

    /// Number of rows in the board (its height).
    pub rows: usize,

    /// The squares on the board.
    pub(crate) squares: Vec<Option<Tile<E>>>,
}

impl<E: Clone> Board<E> {
    /// Create a new, empty board.
    pub(crate) fn new(columns: usize, rows: usize) -> Board<E> {
        Board {
            columns,
            rows,
            squares: vec![None; columns * rows],
        }
    }
}

impl<E> Board<E> {
    pub(crate) fn indx(&self, c: usize, r: usize) -> usize {
        debug_assert!(c < self.columns);
        debug_assert!(r < self.rows);

        let idx = c + r * self.columns;
        // println!("indx: {},{}->{}", c, r, idx);
        idx
    }
}

impl<E> Index<(usize, usize)> for Board<E> {
    type Output = Option<Tile<E>>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (c, r) = index;
        let index = self.indx(c, r);
        unsafe { self.squares.get_unchecked(index) }
    }
}

impl<E> Index<Indx> for Board<E> {
    type Output = <Board<E> as Index<(usize, usize)>>::Output;

    fn index(&self, index: Indx) -> &Self::Output {
        self.index((index.col, index.row))
    }
}

impl<E> IndexMut<(usize, usize)> for Board<E> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (c, r) = index;
        let index = self.indx(c, r);
        unsafe { self.squares.get_unchecked_mut(index) }
    }
}

impl<E> IndexMut<Indx> for Board<E> {
    fn index_mut(&mut self, index: Indx) -> &mut Self::Output {
        self.index_mut((index.col, index.row))
    }
}

/// A board specification.
///
/// This is composed from an optional board dimensions, and a tileset.
pub struct BoardSpec<E> {
    /// The specified dimensions of boards.
    pub dimensions: BoardShape,

    /// The tileset to fill boards with.
    pub tiles: TileSet<E>,
}


impl<E: Copy> BoardSpec<E> {
    /// Parse a clues file.
    ///
    /// The format is described in the formats document.
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
            let tile = self.tiles[self.tiles.id(digits[0])];
            let col = digits[1];
            let row = digits[2];
            let at = Indx { col, row };
            let mut rotation = ROTATIONS[digits[3]];
            if clockwise {
                rotation = rotation.reverse();
            }

            clues.push(Clue { tile, rotation, at })
        }

        clues
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

#[derive(Clone, Copy, Debug)]
/// The shape of a board.
pub struct BoardShape {
    /// Column count
    pub columns: usize,
    /// Row count
    pub rows: usize,
}

impl BoardShape {
    /// Make a new, blank board with the specified shape.
    pub fn new_board<E: Clone>(&self) -> Board<E> {
        Board::new(self.columns, self.rows)
    }
}


/// A clue, giving the tile, its rotation and its position within the puzzle.
#[derive(Clone, Copy, Debug)]
pub struct Clue<E> {
    /// The clue tile.
    pub tile: Tile<E>,
    /// How to rotate the clue tile.
    pub rotation: Rotation,
    /// Where to place the clue tile.
    pub at: Indx,
}

impl<E: Copy + std::fmt::Debug> Clue<E> {
    /// Apply a clue to a board.
    pub fn apply(&self, board: &mut Board<E>) {
        let rott = self.tile.rotate(self.rotation);
        // println!("Writing rotated tile {:?} at {:?}", rott, self.at);
        board[self.at] = Some(rott.apply());
    }
}