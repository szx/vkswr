//! Primitive rasterization.

use common::math::{Color, Fragment, Position, Vertex};
use log::warn;

/// Rasterizes a line segment with Bresenham's algorithm.
///
/// TODO: Replace with a line rasterization that follows the Vulkan rules.
/// See <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-basic>.
pub fn draw_line_bresenham(v0: Vertex, v1: Vertex, fragments: &mut Vec<Fragment>, color: Color) {
    warn!("TODO: Replace line segment rasterization.");
    let v0 = v0.position;
    let v1 = v1.position;

    let (mut x0, mut y0) = (v0.get_as_sfloat32(0) as i32, v0.get_as_sfloat32(1) as i32);
    let (mut x1, mut y1) = (v1.get_as_sfloat32(0) as i32, v1.get_as_sfloat32(1) as i32);

    let steep = (y1 - y0).abs() > (x1 - x0).abs();
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let d_err = (y1 - y0).abs();
    let d_x = x1 - x0;
    let y_step = if y0 > y1 { -1 } else { 1 };

    let mut err = d_x / 2; // Pixel center.
    let mut y = y0;
    for x in x0..=x1 {
        // TODO: Compute z_screen.
        let (fragment_x, fragment_y) = if steep {
            (y as f32, x as f32)
        } else {
            (x as f32, y as f32)
        };
        fragments.push(Fragment {
            // TODO: Get z and w from the vertex shader.
            position: Position::from_sfloat32_raw(fragment_x, fragment_y, 0.0, 1.0),
            color,
        });
        err -= d_err;
        if err < 0 {
            y += y_step;
            err += d_x;
        }
    }
}

/// Rasterizes a filled triangle with the edge-function method.
pub fn draw_triangle(
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
    width: u32,
    height: u32,
    fragments: &mut Vec<Fragment>,
    color: Color,
) {
    if width == 0 || height == 0 {
        return;
    }

    let [x0, x1, x2] = [v0, v1, v2].map(|vertex| vertex.position.get_as_sfloat32(0));
    let [y0, y1, y2] = [v0, v1, v2].map(|vertex| vertex.position.get_as_sfloat32(1));

    let area = (x1 - x0).mul_add(y2 - y0, -((x2 - x0) * (y1 - y0)));
    if area == 0.0 {
        return;
    }

    let min_x = x0.min(x1).min(x2).floor().max(0.0) as i32;
    let max_x = (x0.max(x1).max(x2).ceil() as i32).min(width as i32 - 1);
    let min_y = y0.min(y1).min(y2).floor().max(0.0) as i32;
    let max_y = (y0.max(y1).max(y2).ceil() as i32).min(height as i32 - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let w0 = (x2 - x1).mul_add(py - y1, -((y2 - y1) * (px - x1)));
            let w1 = (x0 - x2).mul_add(py - y2, -((y0 - y2) * (px - x2)));
            let w2 = (x1 - x0).mul_add(py - y0, -((y1 - y0) * (px - x0)));
            let inside =
                (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0);
            if inside {
                fragments.push(Fragment {
                    position: Position::from_sfloat32_raw(x as f32, y as f32, 0.0, 1.0),
                    color,
                });
            }
        }
    }
}

/// Emits one point fragment per input vertex.
pub fn draw_points(
    vertices: impl IntoIterator<Item = Vertex>,
    fragments: &mut Vec<Fragment>,
    color: Color,
) {
    for vertex in vertices {
        fragments.push(Fragment {
            position: Position::from_sfloat32(vertex.position),
            color,
        });
    }
}
