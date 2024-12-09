//! The entity that owns tiles.

use std::collections::HashMap;

use super::vertex::Vertex;
use super::{coordinate::Axial, edge::Edge};

use crate::axial;

use super::shape::HexShape;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enum denoting orientation of hexagons in a grid.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexOrientation {
    /// The top of a hexagon is flat
    FlatTop,
    /// The top of a hexagon is pointy
    PointyTop,
}

// Because Rust has determined to hide a constant behind an 'unstable' tag we restate it here.
/// Constant calculation of square root of 3.
#[allow(clippy::excessive_precision)]
pub const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;

/// A grid of tiles.
///
/// This entity owns the tiles in its coordinate system.
///
/// Contains useful functions to convert to and from world space and grid coordinates.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct HexGrid<T: Clone, V, E> {
    /// Orientation of a hexagonal grid
    pub orientation: HexOrientation,
    /// Size of an individual hexagon
    pub hex_size: f32,
    /// Collection of tiles for the gird
    pub tiles: HashMap<Axial, T>,
    /// Collection of vertices for the grid
    pub vertices: HashMap<Vertex, V>,
    /// Collection of edges for the grid
    pub edges: HashMap<Edge, E>,
}

impl<T: Clone, V, E> Default for HexGrid<T, V, E> {
    fn default() -> Self {
        Self {
            orientation: HexOrientation::PointyTop,
            hex_size: 32.0,
            tiles: Default::default(),
            vertices: Default::default(),
            edges: Default::default(),
        }
    }
}

impl<T: Clone, V, E> HexGrid<T, V, E> {
    /// Convert from worldspace to hex coordinates.
    ///
    /// Takes in a float 64 tuple of the form (x, y) and outputs the coordinates of the nearest tile.
    ///
    /// # Example
    /// ```
    /// let my_object_pos = (100.0, 432.0);
    /// /// ...
    /// use gridava::hex::grid::HexGrid;
    ///
    /// let my_grid = HexGrid::<i32, (), ()>::default();
    /// let nearest_tile = my_grid.world_to_hex(my_object_pos);
    /// ```
    ///
    /// The parent world space can be anything not just a 'game world.' For instance, the screen width and height could be your worldspace.
    /// The grid could even exist in a 3d space and your world's x and y component used.
    pub fn world_to_hex(&self, worldspace: (f64, f64)) -> Axial {
        use crate::axial;

        match self.orientation {
            HexOrientation::PointyTop => {
                let x = worldspace.0 / (SQRT_3 * self.hex_size as f64);
                let y = -worldspace.1 / (SQRT_3 * self.hex_size as f64);
                let t = SQRT_3 * y + 1.0;
                let temp1 = f64::floor(t + x);
                let temp2 = t - x;
                let temp3 = 2.0 * x + 1.0;
                let qf = (temp1 + temp3) / 3.0;
                let rf = (temp1 + temp2) / 3.0;
                axial!(f64::floor(qf) as i32, -f64::floor(rf) as i32)
            }
            HexOrientation::FlatTop => {
                let y = worldspace.0 / (SQRT_3 * self.hex_size as f64);
                let x = -worldspace.1 / (SQRT_3 * self.hex_size as f64);
                let t = SQRT_3 * y + 1.0;
                let temp1 = f64::floor(t + x);
                let temp2 = t - x;
                let temp3 = 2.0 * x + 1.0;
                let rf = (temp1 + temp3) / 3.0;
                let qf = (temp1 + temp2) / 3.0;
                axial!(f64::floor(qf) as i32, -f64::floor(rf) as i32)
            }
        }
    }

    /// Convert from hex to worldspace coordinates.
    ///
    /// Takes in a hex coordinate and outputs the worldspace coordinates of the tile's center.
    ///
    /// # Example
    /// ```
    /// /// ...
    /// use gridava::hex::grid::HexGrid;
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// let my_grid = HexGrid::<i32, (), ()>::default();
    /// let world_pos = my_grid.hex_to_world(axial!(12, 33));
    /// ```
    ///
    /// The parent world space can be anything not just a 'game world.' For instance, the screen width and height could be your worldspace.
    /// The grid could even exist in a 3d space and your world's x and y component used.
    pub fn hex_to_world(&self, coord: Axial) -> (f64, f64) {
        match self.orientation {
            HexOrientation::PointyTop => {
                let x = self.hex_size as f64
                    * (SQRT_3 * coord.q as f64 + SQRT_3 / 2.0 * coord.r as f64);
                let y = self.hex_size as f64 * (3.0 / 2.0 * coord.r as f64);
                (x, y)
            }
            HexOrientation::FlatTop => {
                let x = self.hex_size as f64 * (3.0 / 2.0 * coord.q as f64);
                let y = self.hex_size as f64
                    * (SQRT_3 / 2.0 * coord.q as f64 + SQRT_3 * coord.r as f64);
                (x, y)
            }
        }
    }

    /// Apply the shape onto the grid's collection.
    ///
    /// This will take every element inside a shape that is wrapped in Some()
    /// and apply it into the grid's collection.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::grid::HexGrid;
    /// use gridava::hex::shape::HexShape;
    ///
    /// #[derive(Default, Clone)]
    /// pub struct MyData {
    ///     pub my_int: i32,
    /// }
    ///
    /// let mut my_grid = HexGrid::<MyData, (), ()>::default();
    /// let my_shape = HexShape::make_triangle(1, 0, true, MyData::default);
    ///
    /// my_grid.apply_shape(&my_shape);
    /// ```
    pub fn apply_shape(&mut self, shape: &HexShape<T>) -> &Self {
        shape.get_hexes().indexed_iter().for_each(|ele| {
            if let Some(value) = ele.1.as_ref() {
                // Apply transform
                let coord =
                    axial!(ele.0 .0 as i32, ele.0 .1 as i32).apply_transform(shape.transform);

                self.tiles.insert(coord, value.clone());
            }
        });
        self
    }

    /// Extract data from the grid into the shape.
    ///
    /// Clones data from the grid's collection into the shape's internal working collection.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::grid::HexGrid;
    /// use gridava::hex::shape::HexShape;
    ///
    /// #[derive(Default, Clone)]
    /// pub struct MyData {
    ///     pub my_int: i32,
    /// }
    ///
    /// let my_grid = HexGrid::<MyData, (), ()>::default();
    ///
    /// // ... generate data in grid
    ///
    /// let mut my_shape = HexShape::make_triangle(1, 0, true, MyData::default);
    /// my_grid.extract_shape(&mut my_shape);
    /// ```
    pub fn extract_shape(&self, shape: &mut HexShape<T>) {
        let transform = shape.transform;

        shape.get_hexes_mut().indexed_iter_mut().for_each(|ele| {
            if ele.1.is_some() {
                // Apply transform
                let coord = axial!(ele.0 .0 as i32, ele.0 .1 as i32).apply_transform(transform);
                *ele.1 = self.tiles.get(&coord).cloned();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::axial;
    use assert_float_eq::*;

    #[test]
    fn default() {
        let def = HexGrid::<(), (), ()>::default();

        assert_eq!(def.orientation, HexOrientation::PointyTop);
        assert_eq!(def.hex_size, 32.0);
        assert!(def.tiles.is_empty());
    }

    #[test]
    fn world_to_hex() {
        // Size 10 PT
        let grid10p = HexGrid::<(), (), ()> {
            hex_size: 10.0,
            ..HexGrid::default()
        };

        assert_eq!(grid10p.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(grid10p.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(11, 0));
        assert_eq!(
            grid10p.world_to_hex((SQRT_3 * 56.0, -470.0)),
            axial!(21, -31)
        );
        assert_eq!(grid10p.world_to_hex((0.0, 640.0)), axial!(-21, 42));
        assert_eq!(
            grid10p.world_to_hex((SQRT_3 * 144.0, 640.0)),
            axial!(-7, 43)
        );

        // size 32 PT
        let grid32p = HexGrid {
            hex_size: 32.0,
            ..grid10p
        };

        assert_eq!(grid32p.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(grid32p.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(4, 0));
        assert_eq!(
            grid32p.world_to_hex((SQRT_3 * 56.0, -470.0)),
            axial!(7, -10)
        );
        assert_eq!(grid32p.world_to_hex((0.0, 640.0)), axial!(-6, 13));
        assert_eq!(
            grid32p.world_to_hex((SQRT_3 * 144.0, 640.0)),
            axial!(-2, 13)
        );

        // size 10 FT
        let grid10f = HexGrid {
            hex_size: 10.0,
            orientation: HexOrientation::FlatTop,
            ..grid32p
        };

        assert_eq!(grid10f.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(grid10f.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(13, -7)); // TODO: should this not give (13, -6)?
        assert_eq!(
            grid10f.world_to_hex((SQRT_3 * 56.0, -470.0)),
            axial!(6, -30)
        );
        assert_eq!(grid10f.world_to_hex((0.0, 640.0)), axial!(0, 37));
        assert_eq!(
            grid10f.world_to_hex((SQRT_3 * 144.0, 640.0)),
            axial!(16, 29)
        );

        // size 32 FT
        let grid32f = HexGrid {
            hex_size: 32.0,
            ..grid10f
        };

        assert_eq!(grid32f.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(grid32f.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(4, -2));
        assert_eq!(grid32f.world_to_hex((SQRT_3 * 56.0, -470.0)), axial!(2, -9));
        assert_eq!(grid32f.world_to_hex((0.0, 640.0)), axial!(0, 12));
        assert_eq!(grid32f.world_to_hex((SQRT_3 * 144.0, 640.0)), axial!(5, 9));
    }

    macro_rules! assert_f64_tuples_near {
        ($tup:expr, $cmp:expr) => {
            let (tup, cmp) = ($tup, $cmp);
            assert_f64_near!(tup.0, cmp.0, 4);
            assert_f64_near!(tup.1, cmp.1, 4);
        };
    }

    #[test]
    fn hex_to_world() {
        // Size 10 PT
        let grid = HexGrid::<(), (), ()> {
            hex_size: 10.0,
            ..HexGrid::default()
        };

        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, -15)), (SQRT_3 * -75.0, -225.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 15)), (SQRT_3 * 75.0, 225.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(8, -12)), (SQRT_3 * 20.0, -180.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(8, 12)), (SQRT_3 * 140.0, 180.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(12, -8)), (SQRT_3 * 80.0, -120.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(12, 8)), (SQRT_3 * 160.0, 120.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(15, 0)), (SQRT_3 * 150.0, 0.0));
        assert_f64_tuples_near!(
            grid.hex_to_world(axial!(-8, -12)),
            (SQRT_3 * -140.0, -180.0)
        );
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-8, 12)), (SQRT_3 * -20.0, 180.0));
        assert_f64_tuples_near!(
            grid.hex_to_world(axial!(-12, -8)),
            (SQRT_3 * -160.0, -120.0)
        );
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-12, 8)), (SQRT_3 * -80.0, 120.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-15, 0)), (SQRT_3 * -150.0, 0.0));

        // Size 40 PT
        let grid = HexGrid {
            hex_size: 40.0,
            ..grid
        };

        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, -15)), (SQRT_3 * -300.0, -900.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(15, 0)), (SQRT_3 * 600.0, 0.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-12, 8)), (SQRT_3 * -320.0, 480.0));

        // Size 40 FT
        let grid = HexGrid {
            orientation: HexOrientation::FlatTop,
            ..grid
        };

        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, -15)), (0.0, SQRT_3 * -600.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(15, 0)), (900.0, SQRT_3 * 300.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-12, 8)), (-720.0, SQRT_3 * 80.0));

        // Size 10 FT
        let grid = HexGrid {
            hex_size: 10.0,
            ..grid
        };

        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, -15)), (0.0, SQRT_3 * -150.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 15)), (0.0, SQRT_3 * 150.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(8, -12)), (120.0, SQRT_3 * -80.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(8, 12)), (120.0, SQRT_3 * 160.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(12, -8)), (180.0, SQRT_3 * -20.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(12, 8)), (180.0, SQRT_3 * 140.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(15, 0)), (225.0, SQRT_3 * 75.0));
        assert_f64_tuples_near!(
            grid.hex_to_world(axial!(-8, -12)),
            (-120.0, SQRT_3 * -160.0)
        );
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-8, 12)), (-120.0, SQRT_3 * 80.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-10, 10)), (-150.0, SQRT_3 * 50.0));
        assert_f64_tuples_near!(
            grid.hex_to_world(axial!(-12, -8)),
            (-180.0, SQRT_3 * -140.0)
        );
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-12, 8)), (-180.0, SQRT_3 * 20.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-15, 0)), (-225.0, SQRT_3 * -75.0));
    }

    macro_rules! two_way_conversion {
        ($grid:expr, $tup:expr) => {
            let (grid, tup) = ($grid, $tup);

            assert_eq!(grid.world_to_hex(grid.hex_to_world(tup)), tup);
        };
    }

    #[test]
    fn two_way_identity() {
        let pt10p = HexGrid::<(), (), ()> {
            hex_size: 10.0,
            ..HexGrid::default()
        };

        two_way_conversion!(pt10p.clone(), axial!(0, 0));
        two_way_conversion!(pt10p.clone(), axial!(12, -8));
        two_way_conversion!(pt10p.clone(), axial!(15, 0));
        two_way_conversion!(pt10p.clone(), axial!(0, -15));

        let ft10p = HexGrid {
            orientation: HexOrientation::FlatTop,
            ..pt10p
        };

        two_way_conversion!(ft10p.clone(), axial!(0, 0));
        two_way_conversion!(ft10p.clone(), axial!(12, -8));
        two_way_conversion!(ft10p.clone(), axial!(15, 0));
        two_way_conversion!(ft10p.clone(), axial!(0, -15));
    }

    #[test]
    fn apply_shape() {
        let shape = HexShape::make_rhombus(1, 0, true, || 1);
        let mut grid = HexGrid::<i32, (), ()>::default();

        grid.apply_shape(&shape);

        shape.get_hexes().indexed_iter().for_each(|ele| {
            assert_eq!(
                *grid
                    .tiles
                    .get(&axial!(ele.0 .0 as i32, ele.0 .1 as i32))
                    .unwrap(),
                ele.1.unwrap()
            )
        });
    }

    #[test]
    fn extract_shape() {
        let shape = HexShape::make_rhombus(1, 0, true, || 2);
        let mut grid = HexGrid::<i32, (), ()>::default();

        grid.apply_shape(&shape);

        let mut shape = HexShape::make_rhombus(1, 0, true, || 0);
        grid.extract_shape(&mut shape);

        shape
            .get_hexes()
            .iter()
            .for_each(|ele| assert_eq!(ele.unwrap(), 2));
    }
}
