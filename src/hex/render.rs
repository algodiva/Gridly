// SVG file generation for hex grids

use svg::Document;
use svg::node::element::{Path,SVG,Text};
use svg::node::element::path::Data;

use crate::hex::grid::{HexGrid,HexOrientation};
use crate::core::tile::Tile;

#[allow(clippy::excessive_precision)]
const SQRT3: f64 = 1.732050807568877293527446341505872367_f64;
// Constant for now, longer-term should be configurable
const PAD: f64 = 10.0;


/// Create a SVG object containing a rendering of this grid.
///
/// # Example
/// ```
/// /// ...
/// use gridava::hex::grid::render_svg;
///
/// let my_grid = HexGrid::<i32, ()>::default();
/// let svg = render_svg(my_grid);
/// ```
pub fn render_svg<T: Clone>(grid: HexGrid<i32, Tile<T>>) -> SVG {
    let size_short = grid.hex_size as f64 * 0.5;
    let size_long = size_short * SQRT3;

    let mut doc = Document::new();
    let mut max_q = size_long;
    let mut min_q = -max_q;
    let mut max_r = size_short * 2.0;
    let mut min_r = -max_r;

    // For now, tile is unused
    for (coords, _tile) in grid.tiles.iter() {
        let (base_q, base_r) = grid.hex_to_world(*coords);
        let mut data = Data::new();

        // Do these apply for FlatTop as well as PointyTop?
        if grid.orientation == HexOrientation::PointyTop {
            if base_q - size_long < min_q { min_q = base_q - size_long; }
            if base_q + size_long > max_q { max_q = base_q + size_long; }

            if base_r - size_short * 2.0 < min_r { min_r = base_r - size_short * 2.0; }
            if base_r + size_short * 2.0 > max_r { max_r = base_r + size_short * 2.0; }

            data = data
                .move_to((base_q, base_r + size_short * 2.0))
                .line_to((base_q + size_long, base_r + size_short))
                .line_to((base_q + size_long, base_r - size_short))
                .line_to((base_q, base_r - size_short * 2.0))
                .line_to((base_q - size_long, base_r - size_short))
                .line_to((base_q - size_long, base_r + size_short))
                .line_to((base_q, base_r + size_short * 2.0));
        } else {
            if base_q - size_short * 2.0 < min_q { min_q = base_q - size_short * 2.0; }
            if base_q + size_short * 2.0 > max_q { max_q = base_q + size_short * 2.0; }

            if base_r - size_long < min_r { min_r = base_r - size_long; }
            if base_r + size_long > max_r { max_r = base_r + size_long; }

            data = data
                .move_to((base_q + size_short * 2.0, base_r))
                .line_to((base_q + size_short, base_r + size_long))
                .line_to((base_q - size_short, base_r + size_long))
                .line_to((base_q - size_short * 2.0, base_r))
                .line_to((base_q - size_short, base_r - size_long))
                .line_to((base_q + size_short, base_r - size_long))
                .line_to((base_q + size_short * 2.0, base_r));
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 2)
            .set("d", data);

        let txt = format!("{},{}", coords.q, coords.r);
        let text = Text::new(txt).set("x", base_q).set("y", base_r + 4.0).set("text-anchor", "middle").set("font-size", 12);

        doc = doc.clone().add(path).add(text);
    }

    min_q -= PAD;
    max_q += PAD;
    min_r -= PAD;
    max_r += PAD;

    let border = Data::new()
        .move_to((min_q, min_r))
        .line_to((min_q, max_r))
        .line_to((max_q, max_r))
        .line_to((max_q, min_r))
        .line_to((min_q, min_r));

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke_width", 1)
        .set("d", border);

    doc 
        .add(path)
        .set("viewBox", (min_q, min_r, max_q - min_q, max_r - min_r))
        .set("style", "background-color: #DDDDDD; stroke-width: 1px")
}

/// Save an SVG rendering in a file.
///
/// # Example
/// ```
/// /// ...
/// use gridava::hex::grid::{render_svg,save_svg};
///
/// let my_grid = HexGrid::<i32, ()>::default();
/// let svg = render_svg(my_grid);
/// save_svg("save.svg", svg);
/// ```
pub fn save_svg(path: &str, document: SVG) -> Result<(), std::io::Error> {
    svg::save(path, &document)
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::core::tile::Tile;
    use crate::hex::shape::HexShape;
    use crate::hex::grid::{HexGrid,HexOrientation};

    #[test]
    fn test_render_pointy_top() {
        let shape = HexShape::make_rhombus(2, 0, true, || 1);
        let mut grid = HexGrid::<i32, Tile<i32>>{ orientation: HexOrientation::PointyTop, ..HexGrid::default() };

        grid.apply_shape(&shape);

        // TODO: figure out how to test the output for correctness as the result is nondeterministic
        let svg = render_svg::<i32>(grid);
        let ret = save_svg("test.svg", svg);

        assert!(ret.is_ok());
    }

    #[test]
    fn test_render_flat_top() {
        let shape = HexShape::make_rhombus(3, 0, true, || 1);
        let mut grid = HexGrid::<i32, Tile<i32>>{ orientation: HexOrientation::FlatTop, hex_size: 100.0, ..HexGrid::default() };

        grid.apply_shape(&shape);

        let svg = render_svg::<i32>(grid);
        let ret = save_svg("test.svg", svg);

        assert!(ret.is_ok());
    }
}