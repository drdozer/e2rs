use std::{mem::transmute, {ops::{Index, IndexMut}}};

use super::{Edge, Rotate, Rotation};

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
        // fixme: this is equivalinent to self.rotate(Rotation::180)
        let s = self as usize;
        let s = s + 2;
        unsafe {
            transmute(s % 4)
        }
    }
}

impl Rotate for Side {
    type ROTATED = Side;

    fn rotate(self, rotation: Rotation) -> Self::ROTATED {
        let side = (self as usize + rotation as usize) % 4;
        unsafe {
            transmute(side)
        }
    }
}

/// All [Side] values, in order.
pub const SIDES: [Side; 4] = [Side::North, Side::East, Side::South, Side::West];


/// A tile is 4 edges, ordered as North, East, South, West, or if you prefer,
/// clockwise starting at the top.
///
/// The tile edges can be accessed by their side directly:
///
/// ```
/// assert_eq!(tile.edges[0], tile[Sides::North]);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Tile<E> {
    edges: [E; 4],
}

impl<E> Tile<E> {
    /// Make a new tile, providing the edges in the order of the parameter names.
    pub fn new(north: E, east: E, south: E, west: E) -> Tile<E> {
        Tile {
            edges: [north, east, south, west],
        }
    }
}

impl<E: Edge> Tile<E> {
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

impl<E> Index<Side> for Tile<E> {
    type Output = E;
    fn index(&self, index: Side) -> &Self::Output {
        &self.edges[index as usize]
    }
}

impl<E> IndexMut<Side> for Tile<E> {
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        &mut self.edges[index as usize]
    }
}

impl <'a, E> Rotate for &'a Tile<E> {
    type ROTATED = RotatedTile<'a, E>;

    fn rotate(self, rotation: Rotation) -> Self::ROTATED {
        RotatedTile {
            tile: &self,
            rotation,
        }
    }

}



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
pub struct RotatedTile<'a, E> {
    /// The tile being rotated.
    pub tile: &'a Tile<E>,
    /// The rotation to apply.
    pub rotation: Rotation,
}

impl<'a, E> RotatedTile<'a, E> {
    /// Rotate this rotated tile.
    ///
    /// The result is a new rotated tile that composes this rotation with the pre-existing one.
    pub fn rotate(self, rotation: Rotation) -> RotatedTile<'a, E> {
        RotatedTile {
            tile: self.tile,
            rotation: self.rotation + rotation,
        }
    }
}

impl<'a, E: Copy> RotatedTile<'a, E> {
    /// Apply the tile rotation to yield a new tile with the edges rotated in-place.
    pub fn apply(&self) -> Tile<E> {
        let &Tile {
            edges: [north, east, south, west],
        } = self.tile;
        match self.rotation {
            Rotation::Rot0 => Tile::new(north, east, south, west),
            Rotation::Rot90 => Tile::new(west, north, east, south),
            Rotation::Rot180 => Tile::new(south, west, north, east),
            Rotation::Rot270 => Tile::new(east, south, west, north),
        }
    }
}

impl<'a, E> Index<Side> for RotatedTile<'a, E> {
    type Output = E;

    fn index(&self, index: Side) -> &Self::Output {
        &self.tile[index.rotate(self.rotation)]
    }
}
