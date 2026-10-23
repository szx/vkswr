use common::math::{Color, Fragment, Position, Vertex};
use log::warn;

pub fn draw_line_bresenham(v0: Vertex, v1: Vertex, fragments: &mut Vec<Fragment>, color: Color) {
    // Bresenham's line algorithm
    warn!("TODO: Replace line segment rasterization.");
    // https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-basic
    let v0 = v0.position;
    let v1 = v1.position;

    let (mut x0, mut y0) = (
        (v0.get_as_sfloat32(0)) as i32,
        (v0.get_as_sfloat32(1)) as i32,
    );
    let (mut x1, mut y1) = (
        (v1.get_as_sfloat32(0)) as i32,
        (v1.get_as_sfloat32(1)) as i32,
    );
    let steep = if (y1 - y0).abs() > (x1 - x0).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        true
    } else {
        false
    };
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let d_err = (y1 - y0).abs();
    let d_x = x1 - x0;
    let y_step = if y0 > y1 { -1_i32 } else { 1 };

    let mut err = d_x / 2; // Pixel center.
    let mut y = y0;
    for x in x0..=x1 {
        // TODO: z_screen
        let (x_fragment, y_fragment) = if steep {
            (y as f32, x as f32)
        } else {
            (x as f32, y as f32)
        };
        fragments.push(Fragment {
            position: Position::from_sfloat32_raw(x_fragment, y_fragment, 0.0f32, 1.0f32), // TODO: Get z and w from vertex shader.
            color,
        });
        err -= d_err;
        if err < 0 {
            y += y_step;
            err += d_x;
        }
    }
}

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

    let p0 = v0.position;
    let p1 = v1.position;
    let p2 = v2.position;
    let (x0, y0) = (p0.get_as_sfloat32(0), p0.get_as_sfloat32(1));
    let (x1, y1) = (p1.get_as_sfloat32(0), p1.get_as_sfloat32(1));
    let (x2, y2) = (p2.get_as_sfloat32(0), p2.get_as_sfloat32(1));

    let area = (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0);
    if area == 0.0f32 {
        return;
    }

    let min_x = x0.min(x1).min(x2).floor().max(0.0f32) as i32;
    let max_x = (x0.max(x1).max(x2).ceil() as i32).min(width as i32 - 1);
    let min_y = y0.min(y1).min(y2).floor().max(0.0f32) as i32;
    let max_y = (y0.max(y1).max(y2).ceil() as i32).min(height as i32 - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32 + 0.5f32;
            let py = y as f32 + 0.5f32;
            let w0 = (x2 - x1) * (py - y1) - (y2 - y1) * (px - x1);
            let w1 = (x0 - x2) * (py - y2) - (y0 - y2) * (px - x2);
            let w2 = (x1 - x0) * (py - y0) - (y1 - y0) * (px - x0);
            let inside = (w0 >= 0.0f32 && w1 >= 0.0f32 && w2 >= 0.0f32)
                || (w0 <= 0.0f32 && w1 <= 0.0f32 && w2 <= 0.0f32);
            if inside {
                fragments.push(Fragment {
                    position: Position::from_sfloat32_raw(x as f32, y as f32, 0.0f32, 1.0f32),
                    color,
                });
            }
        }
    }
}

pub fn draw_points(
    vertices: impl IntoIterator<Item = Vertex>,
    fragments: &mut Vec<Fragment>,
    color: Color,
) {
    for vertex in vertices {
        let position = Position::from_sfloat32(vertex.position);
        fragments.push(Fragment { position, color });
    }
}
