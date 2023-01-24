//! Representation of a generic Eternity 2 style puzzle.
//!
//! This includes tiles, puzzle boards, clues and other commonly-needed types.
//! Applications coded against these APIs will be able to work with any puzzle.
//! To work specifically with the Eternith 2 Puzzle, look in [crate::e2] for specialised types
//! and data.

mod tiles;
pub use tiles::*;

mod rotation;
pub use rotation::*;

mod tileset;
pub use tileset::*;

/// Shared operations on things that are edges.
pub trait Edge {
    /// Check if the edge is a border, that must be placed to the outside of the puzzle.
    fn is_border(&self) -> bool;
}



mod board;
pub use board::*;