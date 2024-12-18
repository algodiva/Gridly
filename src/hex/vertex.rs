//! Handles vertices in a hexagonal grid.

use crate::edge;

use super::{
    coordinate::{axial, Axial},
    edge::{Edge, EdgeDirection},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Vertex spin is a orientation of the vertex.
///
/// A vertex needs to know its `spin`. Spin correlates to which side [`VertexSpin::Up`] or [`VertexSpin::Down`]
/// has two hexagons.
///
/// see [`Vertex`]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum VertexSpin {
    /// On top of the hex
    Up,
    /// On the bottom of the hex
    Down,
}

/// A vertex direction denotes the direction from the hexagon center the vertex is.
///
/// Reference pointy-top hexagons for vertex direction, where up being directly above the center.
///
/// see [`Vertex`]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum VertexDirection {
    /// The vertex at the top
    Up,
    /// The vertex at the top right
    UpRight,
    /// The vertex at the bottom right
    DownRight,
    /// The vertex at the bottom
    Down,
    /// The vertex at the bottom left
    DownLeft,
    /// The vertex at the top left
    UpLeft,
}

impl From<i32> for VertexDirection {
    fn from(value: i32) -> Self {
        match value.rem_euclid(6) {
            0 => VertexDirection::Up,
            1 => VertexDirection::UpRight,
            2 => VertexDirection::DownRight,
            3 => VertexDirection::Down,
            4 => VertexDirection::DownLeft,
            5 => VertexDirection::UpLeft,
            _ => unreachable!(), // should never reach
        }
    }
}

impl From<VertexDirection> for i32 {
    fn from(value: VertexDirection) -> Self {
        match value {
            VertexDirection::Up => 0,
            VertexDirection::UpRight => 1,
            VertexDirection::DownRight => 2,
            VertexDirection::Down => 3,
            VertexDirection::DownLeft => 4,
            VertexDirection::UpLeft => 5,
        }
    }
}

/// Vertex associated with hexagon grids.
///
/// A hexagonal vertex follows the same ruleset as axial coordinates with one exception.
///
/// It needs to know its `spin`. Spin correlates to which side [`VertexSpin::Up`] or [`VertexSpin::Down`]
/// has two hexagons.
///
/// See [`vertex`] for helper macro to instantiate these structs.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Vertex {
    /// q (x) coordinate of the vertex
    pub q: i32,
    /// r (y) coordinate of the vertex
    pub r: i32,
    /// The vertex orientation
    pub spin: VertexSpin,
}

/// Helper macro to create [`Vertex`] structs.
#[macro_export]
macro_rules! vertex {
    ($q:expr, $r:expr, $sp:expr) => {
        Vertex {
            q: $q,
            r: $r,
            spin: $sp,
        }
    };
}
pub use vertex;

impl Default for Vertex {
    fn default() -> Self {
        Self {
            q: 0,
            r: 0,
            spin: VertexSpin::Up,
        }
    }
}

impl From<VertexDirection> for Vertex {
    fn from(value: VertexDirection) -> Self {
        match value {
            VertexDirection::Up => vertex!(0, 0, VertexSpin::Up),
            VertexDirection::UpRight => vertex!(1, -1, VertexSpin::Down),
            VertexDirection::DownRight => vertex!(0, 1, VertexSpin::Up),
            VertexDirection::Down => vertex!(0, 0, VertexSpin::Down),
            VertexDirection::DownLeft => vertex!(-1, 1, VertexSpin::Up),
            VertexDirection::UpLeft => vertex!(0, -1, VertexSpin::Down),
        }
    }
}

impl From<(Axial, VertexSpin)> for Vertex {
    fn from(value: (Axial, VertexSpin)) -> Self {
        vertex!(value.0.q, value.0.r, value.1)
    }
}

impl Vertex {
    /// Get all 3 adjacent hexes to this vertex.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::vertex::{Vertex, VertexSpin, vertex};
    /// use gridava::hex::coordinate::Axial;
    ///
    /// let coords = vertex!(2, 0, VertexSpin::Down).adjacent_hexes();
    /// ```
    pub fn adjacent_hexes(&self) -> [Axial; 3] {
        if self.spin == VertexSpin::Up {
            [
                axial!(self.q, self.r),
                axial!(self.q, self.r - 1),
                axial!(self.q + 1, self.r - 1),
            ]
        } else {
            // Spin down
            [
                axial!(self.q, self.r),
                axial!(self.q, self.r + 1),
                axial!(self.q - 1, self.r + 1),
            ]
        }
    }

    /// Get all 3 adjacent vertices to this vertex.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::vertex::{Vertex, VertexSpin, vertex};
    /// use gridava::hex::coordinate::Axial;
    ///
    /// let vertices = vertex!(2, 0, VertexSpin::Down).adjacent_vertices();
    /// ```
    pub fn adjacent_vertices(&self) -> [Self; 3] {
        if self.spin == VertexSpin::Up {
            [
                vertex!(self.q + 1, self.r - 1, VertexSpin::Down),
                vertex!(self.q, self.r - 1, VertexSpin::Down),
                vertex!(self.q + 1, self.r - 2, VertexSpin::Down),
            ]
        } else {
            [
                vertex!(self.q, self.r + 1, VertexSpin::Up),
                vertex!(self.q - 1, self.r + 2, VertexSpin::Up),
                vertex!(self.q - 1, self.r + 1, VertexSpin::Up),
            ]
        }
    }

    /// Generate the edges adjacent to this vertex.
    ///
    /// ```
    /// use gridava::hex::vertex::{Vertex, VertexSpin, vertex};
    ///
    /// let edges = vertex!(0,0,VertexSpin::Up).adjacent_edges();
    /// ```
    pub fn adjacent_edges(&self) -> [Edge; 3] {
        match self.spin {
            VertexSpin::Up => [
                edge!(self.q + 1, self.r - 1, EdgeDirection::West),
                edge!(self.q, self.r, EdgeDirection::NorthEast),
                edge!(self.q, self.r, EdgeDirection::NorthWest),
            ],
            VertexSpin::Down => [
                edge!(self.q, self.r + 1, EdgeDirection::NorthWest),
                edge!(self.q, self.r + 1, EdgeDirection::West),
                edge!(self.q - 1, self.r + 1, EdgeDirection::NorthEast),
            ],
        }
    }

    /// Compute the L1 distance between two vertices.
    ///
    /// ```
    /// use gridava::hex::vertex::{Vertex, VertexSpin, vertex};
    ///
    /// let dist = vertex!(0,0,VertexSpin::Up).distance(vertex!(1,0,VertexSpin::Up));
    /// ```
    #[cfg(feature = "std")]
    pub fn distance(&self, b: Self) -> i32 {
        // Check for same coordinate
        if self.q == b.q && self.r == b.r {
            return if self.spin == b.spin { 0 } else { 3 };
        }
        let dist = axial!(self.q, self.r).distance(axial!(b.q, b.r));
        let dir = axial!(self.q, self.r).direction(axial!(b.q, b.r));
        let parity: usize = if self.spin == b.spin {
            0 // Same
        } else if self.spin == VertexSpin::Up && b.spin == VertexSpin::Down {
            1 // NS
        } else {
            2 // SN
        };

        // Define adjustment constants for each parity type
        const PARITY_ADJUSTMENTS: [[[i32; 6]; 3]; 2] = [
            // On Axis
            [
                [0, 0, 0, 0, 0, 0],   // Same
                [1, 3, 3, 1, -1, -1], // NS
                [1, -1, -1, 1, 3, 3], // SN
            ],
            // Off Axis
            [
                [0, 0, 0, 0, 0, 0],    // Same
                [1, 3, 1, -1, -1, -1], // NS
                [-1, -1, -1, 1, 3, 1], // SN
            ],
        ];

        // Determine sector index (0 to 5)
        let sector = ((dir as i32 / 60) % 6) as usize;

        // Coerced bool into usize.
        let on_axis = (dir.round() as i32 % 60 != 0) as usize;

        // Fetch the appropriate adjustment
        let base_adjustment = PARITY_ADJUSTMENTS[on_axis][parity][sector];

        // Calculate final distance
        2 * dist + base_adjustment
    }

    /// Compute the L1 distance between two vertices.
    ///
    /// ```
    /// use gridava::hex::vertex::{Vertex, VertexSpin, vertex};
    ///
    /// let dist = vertex!(0,0,VertexSpin::Up).distance(vertex!(1,0,VertexSpin::Up));
    /// ```
    #[cfg(not(feature = "std"))]
    pub fn distance(&self, b: Self) -> i32 {
        use crate::lib::round;
        // Check for same coordinate
        if self.q == b.q && self.r == b.r {
            return if self.spin == b.spin { 0 } else { 3 };
        }
        let dist = axial!(self.q, self.r).distance(axial!(b.q, b.r));
        let dir = axial!(self.q, self.r).direction(axial!(b.q, b.r));
        let parity: usize = if self.spin == b.spin {
            0 // Same
        } else if self.spin == VertexSpin::Up && b.spin == VertexSpin::Down {
            1 // NS
        } else {
            2 // SN
        };

        // Define adjustment constants for each parity type
        const PARITY_ADJUSTMENTS: [[[i32; 6]; 3]; 2] = [
            // On Axis
            [
                [0, 0, 0, 0, 0, 0],   // Same
                [1, 3, 3, 1, -1, -1], // NS
                [1, -1, -1, 1, 3, 3], // SN
            ],
            // Off Axis
            [
                [0, 0, 0, 0, 0, 0],    // Same
                [1, 3, 1, -1, -1, -1], // NS
                [-1, -1, -1, 1, 3, 1], // SN
            ],
        ];

        // Determine sector index (0 to 5)
        let sector = ((dir as i32 / 60) % 6) as usize;

        // Coerced bool into usize.
        let on_axis = (round(dir) as i32 % 60 != 0) as usize;

        // Fetch the appropriate adjustment
        let base_adjustment = PARITY_ADJUSTMENTS[on_axis][parity][sector];

        // Calculate final distance
        2 * dist + base_adjustment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_axial() {
        assert_eq!(
            Vertex::from((axial!(0, 0), VertexSpin::Up)),
            vertex!(0, 0, VertexSpin::Up)
        );
    }

    #[test]
    fn adjacent_hexes() {
        assert_eq!(
            vertex!(0, 0, VertexSpin::Down).adjacent_hexes(),
            [axial!(0, 0), axial!(0, 1), axial!(-1, 1)]
        );
        assert_eq!(
            vertex!(0, 0, VertexSpin::Up).adjacent_hexes(),
            [axial!(0, 0), axial!(0, -1), axial!(1, -1)]
        );
        assert_eq!(
            vertex!(1, 0, VertexSpin::Down).adjacent_hexes(),
            [axial!(1, 0), axial!(1, 1), axial!(0, 1)]
        );
    }

    #[test]
    fn adjacent_vertices() {
        assert_eq!(
            vertex!(0, 0, VertexSpin::Down).adjacent_vertices(),
            [
                vertex!(0, 1, VertexSpin::Up),
                vertex!(-1, 2, VertexSpin::Up),
                vertex!(-1, 1, VertexSpin::Up)
            ]
        );
        assert_eq!(
            vertex!(0, 0, VertexSpin::Up).adjacent_vertices(),
            [
                vertex!(1, -1, VertexSpin::Down),
                vertex!(0, -1, VertexSpin::Down),
                vertex!(1, -2, VertexSpin::Down)
            ]
        );
        assert_eq!(
            vertex!(1, 0, VertexSpin::Down).adjacent_vertices(),
            [
                vertex!(1, 1, VertexSpin::Up),
                vertex!(0, 2, VertexSpin::Up),
                vertex!(0, 1, VertexSpin::Up)
            ]
        );
    }

    #[test]
    fn adjacent_edges() {
        assert_eq!(
            vertex!(0, 0, VertexSpin::Up).adjacent_edges(),
            [
                edge!(1, -1, EdgeDirection::West),
                edge!(0, 0, EdgeDirection::NorthEast),
                edge!(0, 0, EdgeDirection::NorthWest),
            ]
        );

        assert_eq!(
            vertex!(0, 0, VertexSpin::Down).adjacent_edges(),
            [
                edge!(0, 1, EdgeDirection::NorthWest),
                edge!(0, 1, EdgeDirection::West),
                edge!(-1, 1, EdgeDirection::NorthEast),
            ]
        );
    }

    #[test]
    fn distance() {
        assert_eq!(
            vertex!(0, 0, VertexSpin::Up).distance(vertex!(0, 0, VertexSpin::Up)),
            0
        );

        assert_eq!(
            vertex!(0, 0, VertexSpin::Up).distance(vertex!(0, 0, VertexSpin::Down)),
            3
        );

        assert_eq!(
            vertex!(0, 0, VertexSpin::Up).distance(vertex!(1, 1, VertexSpin::Up)),
            4
        );

        assert_eq!(
            vertex!(-1, 0, VertexSpin::Up).distance(vertex!(1, 1, VertexSpin::Down)),
            7
        );

        assert_eq!(
            vertex!(0, 0, VertexSpin::Down).distance(vertex!(0, 1, VertexSpin::Up)),
            1
        );

        assert_eq!(
            vertex!(0, 0, VertexSpin::Down).distance(vertex!(0, 1, VertexSpin::Down)),
            2
        );

        assert_eq!(
            vertex!(0, 0, VertexSpin::Down).distance(vertex!(1, -1, VertexSpin::Up)),
            5
        );

        assert_eq!(
            vertex!(0, 0, VertexSpin::Up).distance(vertex!(2, -1, VertexSpin::Up)),
            4
        );

        assert_eq!(
            vertex!(-1, 0, VertexSpin::Up).distance(vertex!(1, 1, VertexSpin::Up)),
            6
        );
    }

    #[test]
    fn from_i32() {
        for i in 0..=5 {
            let vd = VertexDirection::from(i);
            assert_eq!(vd, i.into());
        }
    }

    #[test]
    fn from_vd_i32() {
        assert_eq!(i32::from(VertexDirection::Up), 0);
        assert_eq!(i32::from(VertexDirection::UpRight), 1);
        assert_eq!(i32::from(VertexDirection::DownRight), 2);
        assert_eq!(i32::from(VertexDirection::Down), 3);
        assert_eq!(i32::from(VertexDirection::DownLeft), 4);
        assert_eq!(i32::from(VertexDirection::UpLeft), 5);
    }

    #[test]
    fn from_vd() {
        assert_eq!(
            Vertex::from(VertexDirection::Up),
            vertex!(0, 0, VertexSpin::Up)
        );
        assert_eq!(
            Vertex::from(VertexDirection::UpRight),
            vertex!(1, -1, VertexSpin::Down)
        );
        assert_eq!(
            Vertex::from(VertexDirection::DownRight),
            vertex!(0, 1, VertexSpin::Up)
        );
        assert_eq!(
            Vertex::from(VertexDirection::Down),
            vertex!(0, 0, VertexSpin::Down)
        );
        assert_eq!(
            Vertex::from(VertexDirection::DownLeft),
            vertex!(-1, 1, VertexSpin::Up)
        );
        assert_eq!(
            Vertex::from(VertexDirection::UpLeft),
            vertex!(0, -1, VertexSpin::Down)
        );
    }

    #[test]
    fn default() {
        assert_eq!(Vertex::default(), vertex!(0, 0, VertexSpin::Up));
    }
}
