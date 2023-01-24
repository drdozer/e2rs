use std::mem::transmute;

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
pub const ROTATIONS: [Rotation; 4] = [
    Rotation::Rot0,
    Rotation::Rot90,
    Rotation::Rot180,
    Rotation::Rot270,
];

/// Encapsulation of a generic 'rotate' operation.
/// 
/// The trait is written as consuming `self`.
/// To avoid consuming the original representation, implement the trait on a (consumable) reference.
pub trait Rotate {
    /// The type of the rotated version.
    type ROTATED;

    /// Make a rotated version of this entity.
    fn rotate(self, rotation: Rotation) -> Self::ROTATED;
}