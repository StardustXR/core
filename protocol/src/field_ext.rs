use crate::protocol::field::{CubicBezierControlPoint, Shape};
use crate::protocol::lines::{Line, LinePoint};
use crate::protocol::types::Color;

impl Shape {
    /// Convert a CubicBezierSpline shape to a Line for visualization.
    /// Returns `None` if this shape is not a CubicBezierSpline.
    pub fn spline_to_lines(&self, curve_segment_count: usize) -> Option<Line> {
        match self {
            Shape::CubicBezierSpline { points, cyclic } => {
                Some(cubic_bezier_to_line(points, *cyclic, curve_segment_count))
            }
            _ => None,
        }
    }
}

/// Convert cubic bezier control points to a polyline.
pub fn cubic_bezier_to_line(
    control_points: &[CubicBezierControlPoint],
    cyclic: bool,
    curve_segment_count: usize,
) -> Line {
    let mut points = Vec::new();

    if control_points.len() < 2 {
        for cp in control_points {
            points.push(LinePoint {
                point: cp.anchor,
                thickness: cp.thickness,
                color: Color::WHITE,
            });
        }
    } else {
        let segment_count = if cyclic {
            control_points.len()
        } else {
            control_points.len() - 1
        };

        for i in 0..segment_count {
            let p0 = &control_points[i];
            let p1 = &control_points[(i + 1) % control_points.len()];

            let a = p0.anchor;
            let b = p0.handle_out;
            let c = p1.handle_in;
            let d = p1.anchor;

            let is_last = i == segment_count - 1;
            let include_endpoint = is_last && !cyclic;
            let samples = if include_endpoint {
                curve_segment_count + 1
            } else {
                curve_segment_count
            };

            for j in 0..samples {
                let t = j as f32 / curve_segment_count as f32;
                let inv = 1.0 - t;
                let inv2 = inv * inv;
                let t2 = t * t;

                let x =
                    inv2 * inv * a.x + 3.0 * inv2 * t * b.x + 3.0 * inv * t2 * c.x + t2 * t * d.x;
                let y =
                    inv2 * inv * a.y + 3.0 * inv2 * t * b.y + 3.0 * inv * t2 * c.y + t2 * t * d.y;
                let z =
                    inv2 * inv * a.z + 3.0 * inv2 * t * b.z + 3.0 * inv * t2 * c.z + t2 * t * d.z;

                let thickness = inv * p0.thickness + t * p1.thickness;

                points.push(LinePoint {
                    point: [x, y, z].into(),
                    thickness,
                    color: Color::WHITE,
                });
            }
        }
    }

    Line { points, cyclic }
}
