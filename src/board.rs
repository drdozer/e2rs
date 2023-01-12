use std::{ops::{Index, IndexMut}, usize};


#[derive(Debug, Clone, Copy, Default)]
/// A tile is 4 edges of type `E`, ordered as North, East, South, West, or if you prefer,
/// clockwise starting at the top.
pub struct Tile<E>(pub[E;4]);

impl <E> Tile<E> {
    pub fn new(north: E, east: E, south: E, west: E) -> Tile<E> {
        Tile([north, east, south, west])
    }
}


#[derive(Debug)]
pub struct Tiles<E, const TILE_COUNT: usize>(pub [Tile<E>; TILE_COUNT]);

impl <E, const TILE_COUNT: usize> Index<usize> for Tiles<E, TILE_COUNT> {
    type Output = Tile<E>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0.index(index)
    }
}

impl <'a, E, const TILE_COUNT: usize> IntoIterator for &'a Tiles<E, TILE_COUNT> {
    type Item = &'a Tile<E>;
    type IntoIter = <&'a [Tile<E>; TILE_COUNT] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}
    
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Rotation {
    Zero = 0,
    Clockwise = 1,
    Flip = 2,
    Anti = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct PlacedTile<E> {
    pub tile: Tile<E>,
    pub rotation: Rotation,
}


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

#[derive(Clone, Copy, Debug)]
pub struct Clue<E> {
    pub tile: Tile<E>,
    pub rotation: Rotation,
    pub at: Indx,
}

#[derive(Clone, Copy, Debug)]
pub struct Indx {
    pub col: usize,
    pub row: usize,
}

// If we get really keen, we can make indexing use opaque Col, Row, Cell structs and then
// guarantee that unsafe lookups are in bounds.
// This would require some boilerplate to make ergonomic, so a job for later.