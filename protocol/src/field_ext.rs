//! Signed-distance-field closest-point queries for interaction fields.
//!
//! Design
//! ──────
//! Every [`Shape`] variant implements a `query(point) → FieldSample` that returns:
//!   • the closest surface point   (world-space)
//!   • the signed Euclidean distance (negative = inside, positive = outside)
//!   • the outward unit surface normal (gradient of the SDF, always outward)
//!
//! Primitives are evaluated in **local space** (centred at origin, canonical
//! orientation).  [`Shape::Transform`] applies the affine map following the
//! five-step plan:
//!
//!   1. p_local  = M⁻¹ · p_world           [one matrix mul]
//!   2+3. q_local = shape.query(p_local)    [exact in undeformed local space]
//!   4. q_world  = M · q_local             [one matrix mul]
//!   5. dist     = sign(d_local)·|p_world−q_world|  [true world distance]
//!
//! Under non-uniform scale the returned distance is a conservative
//! **overestimate** of the true minimum world-distance by at most (κ−1)·d
//! where κ = σ_max/σ_min.  For gltf parents with moderate eccentricity
//! (κ ≲ 1.5) the error is small.  If exactness is critical, add a Newton
//! refinement pass after step 5.
//!
//! [`Shape::Union`] uses the *routed-VDF* strategy:
//!   • exterior → hard min (exact)
//!   • interior → route to the boundary surface whose closest point lies
//!     outside all sibling shapes (correct gradient, underestimate distance)
//!   • corner-corner fallback → hard min (may have interior seam artefacts
//!     in the geometrically tiny corner-corner zone)
//!
//! Dependencies: `glam = "0.27"` (or any compatible 0.2x).
//! Swap `types::Vec3f`/`types::Mat4` for glam's `Vec3`/`Mat4` at the boundary
//! to your existing code.
//!

use crate::field::FieldSample;
use crate::protocol::field::{CubicBezierControlPoint, Shape};
use crate::protocol::lines::{Line, LinePoint};
use crate::protocol::types::Color;
use glam::{Mat4, Vec3, vec3};

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

// ─── Public types ─────────────────────────────────────────────────────────────

impl FieldSample {
	fn infinite() -> Self {
		Self {
			closest_point: Vec3::ZERO.into(),
			distance: f32::INFINITY,
			gradient: Vec3::Y.into(),
		}
	}
}

pub trait ShapeExt {
	fn query(&self, point: Vec3) -> FieldSample;
}
impl ShapeExt for Shape {
	/// Query the closest surface point and signed distance from `point`.
	///
	/// Pass world-space coordinates; [`Shape::Transform`] nodes handle the
	/// world→local→world conversion internally.
	fn query(&self, point: Vec3) -> FieldSample {
		match self {
			Shape::Box { size } => box_sample(point, vec3(size.x, size.y, size.z) * 0.5),
			Shape::Sphere { radius } => sphere_sample(point, *radius),
			Shape::Capsule { length, radius } => capsule_sample(point, length * 0.5, *radius),
			Shape::Cylinder { length, radius } => cylinder_sample(point, length * 0.5, *radius),
			Shape::Torus {
				major_radius,
				minor_radius,
			} => torus_sample(point, *major_radius, *minor_radius),
			Shape::CubicBezierSpline { points, cyclic } => spline_sample(point, points, *cyclic),
			Shape::Transform { shape, transform } => {
				transform_sample(point, shape, &transform.mint())
			}
			Shape::Union { shapes } => union_sample(point, shapes),
			Shape::SmoothUnion { shapes, smoothing } => {
				smooth_union_sample(point, shapes, *smoothing)
			}
			Shape::Sweep { surface, sweeper } => sweep_sample(point, surface, sweeper),
		}
	}
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Any unit vector orthogonal to `v`.
fn orthogonal(v: Vec3) -> Vec3 {
	let v = v.normalize();
	if v.x.abs() <= v.y.abs() && v.x.abs() <= v.z.abs() {
		Vec3::new(0.0, -v.z, v.y).normalize()
	} else if v.y.abs() <= v.z.abs() {
		Vec3::new(-v.z, 0.0, v.x).normalize()
	} else {
		Vec3::new(-v.y, v.x, 0.0).normalize()
	}
}

// ─── Primitives (centred at origin, canonical orientation) ────────────────────

/// Axis-aligned box with given half-extents.
fn box_sample(p: Vec3, half: Vec3) -> FieldSample {
	let q = p.clamp(-half, half); // closest point on/in box
	let outside = p - q; // zero vector when p is inside
	let ext_len = outside.length();

	if ext_len > 1e-7 {
		// Exterior: q is exactly on the box surface.
		FieldSample {
			closest_point: q.into(),
			distance: ext_len,
			gradient: (outside / ext_len).into(),
		}
	} else {
		// Interior: nearest face = axis with smallest clearance.
		let dtf = half - p.abs(); // clearance to each face pair
		let (min_d, n) = if dtf.x <= dtf.y && dtf.x <= dtf.z {
			(dtf.x, Vec3::new(p.x.signum(), 0.0, 0.0))
		} else if dtf.y <= dtf.z {
			(dtf.y, Vec3::new(0.0, p.y.signum(), 0.0))
		} else {
			(dtf.z, Vec3::new(0.0, 0.0, p.z.signum()))
		};

		FieldSample {
			closest_point: (p + n * min_d).into(), // push to face
			distance: -min_d,                      // negative = inside
			gradient: n.into(),                    // outward face normal
		}
	}
}

/// Sphere of given radius, centred at origin.
fn sphere_sample(p: Vec3, r: f32) -> FieldSample {
	let len = p.length();
	// dir = radially outward direction; stable even at the origin.
	let dir = if len > 1e-7 { p / len } else { Vec3::Y };
	FieldSample {
		closest_point: (dir * r).into(),
		distance: len - r,
		// For sphere, normalize(p) is the outward normal everywhere,
		// interior *and* exterior — the closest surface point is always
		// on the same side of centre as p.
		gradient: dir.into(),
	}
}

/// Capsule: cylinder of `half_len` along Y with hemispherical caps, radius `r`.
fn capsule_sample(p: Vec3, half_len: f32, r: f32) -> FieldSample {
	// Project onto the Y segment [−half_len, half_len].
	let axis_pt = Vec3::new(0.0, p.y.clamp(-half_len, half_len), 0.0);
	let off = p - axis_pt;
	let len = off.length();
	// Radially outward direction from the capsule axis; correct for interior too.
	let dir = if len > 1e-7 {
		off / len
	} else {
		orthogonal(Vec3::Y)
	};
	FieldSample {
		closest_point: (axis_pt + dir * r).into(),
		distance: len - r,
		gradient: dir.into(),
	}
}

/// Flat-capped cylinder: `half_len` along Y, radius `r` in XZ.
fn cylinder_sample(p: Vec3, half_len: f32, r: f32) -> FieldSample {
	let xz_len = (p.x * p.x + p.z * p.z).sqrt();
	// Radially outward in XZ; fallback to +X when on the Y axis.
	let xz_dir = if xz_len > 1e-7 {
		Vec3::new(p.x / xz_len, 0.0, p.z / xz_len)
	} else {
		Vec3::X
	};

	let d_side = xz_len - r; // signed dist to barrel (neg = inside)
	let d_cap = p.y.abs() - half_len; // signed dist to caps   (neg = inside)

	// IQ formula: exterior corner → Euclidean; otherwise max of two components.
	let dist = if d_side > 0.0 && d_cap > 0.0 {
		(d_side * d_side + d_cap * d_cap).sqrt()
	} else {
		d_side.max(d_cap)
	};

	// Closest point and outward normal:
	let (closest, gradient) = if d_side > 0.0 && d_cap > 0.0 {
		// Exterior corner: nearest point is on the rim circle.
		let c = xz_dir * r + Vec3::new(0.0, p.y.signum() * half_len, 0.0);
		let off = p - c;
		let ol = off.length();
		(c, if ol > 1e-7 { off / ol } else { xz_dir })
	} else if d_side >= d_cap {
		// Closest to the barrel surface (interior or exterior).
		// Outward normal is radially outward in XZ — no sign flip for interior.
		let c = xz_dir * r + Vec3::new(0.0, p.y.clamp(-half_len, half_len), 0.0);
		(c, xz_dir)
	} else {
		// Closest to a cap face (interior or exterior).
		// Outward normal is ±Y — no sign flip for interior.
		let c = Vec3::new(p.x, p.y.signum() * half_len, p.z);
		(c, Vec3::new(0.0, p.y.signum(), 0.0))
	};

	FieldSample {
		closest_point: closest.into(),
		distance: dist,
		gradient: gradient.into(),
	}
}

/// Torus: ring of `major_r` in the XZ plane, tube of `minor_r`.
fn torus_sample(p: Vec3, major_r: f32, minor_r: f32) -> FieldSample {
	let xz_len = (p.x * p.x + p.z * p.z).sqrt();
	// Closest point on the major ring circle.
	let ring = if xz_len > 1e-7 {
		Vec3::new(p.x / xz_len * major_r, 0.0, p.z / xz_len * major_r)
	} else {
		Vec3::new(major_r, 0.0, 0.0) // degenerate: on the torus axis
	};

	let to_p = p - ring;
	let to_len = to_p.length();
	// Outward from the tube centre — correct interior and exterior.
	let dir = if to_len > 1e-7 {
		to_p / to_len
	} else {
		Vec3::Y
	};

	FieldSample {
		closest_point: (ring + dir * minor_r).into(),
		distance: to_len - minor_r,
		gradient: dir.into(),
	}
}

// ─── Cubic Bézier spline ──────────────────────────────────────────────────────

fn cubic_bezier(b0: Vec3, b1: Vec3, b2: Vec3, b3: Vec3, t: f32) -> Vec3 {
	let u = 1.0 - t;
	b0 * (u * u * u) + b1 * (3.0 * u * u * t) + b2 * (3.0 * u * t * t) + b3 * (t * t * t)
}

/// Tapered capsule segment from `(a, ra)` to `(b, rb)`: a varying-radius tube.
fn tapered_segment_sample(p: Vec3, a: Vec3, ra: f32, b: Vec3, rb: f32) -> FieldSample {
	let seg = b - a;
	let seg_sq = seg.length_squared();

	let t = if seg_sq > 1e-10 {
		((p - a).dot(seg) / seg_sq).clamp(0.0, 1.0)
	} else {
		0.0
	};

	let axis_pt = a + seg * t;
	let r = ra + (rb - ra) * t;

	let off = p - axis_pt;
	let len = off.length();
	let dir = if len > 1e-7 {
		off / len
	} else if seg_sq > 1e-10 {
		orthogonal(seg.normalize())
	} else {
		Vec3::X
	};

	FieldSample {
		closest_point: (axis_pt + dir * r).into(),
		distance: len - r,
		gradient: dir.into(), // radially outward from the segment axis
	}
}

fn spline_sample(p: Vec3, cps: &[CubicBezierControlPoint], cyclic: bool) -> FieldSample {
	const SUBS: usize = 10; // linear subdivisions per cubic segment

	if cps.is_empty() {
		return FieldSample::infinite();
	}

	let n_segs = if cyclic {
		cps.len()
	} else {
		cps.len().saturating_sub(1)
	};
	let mut best: Option<FieldSample> = None;

	for i in 0..n_segs {
		let c0 = &cps[i];
		let c1 = &cps[(i + 1) % cps.len()];
		let (b0, b1, b2, b3) = (
			c0.anchor.mint(),
			c0.handle_out.mint(),
			c1.handle_in.mint(),
			c1.anchor.mint(),
		);

		let mut prev: Option<(Vec3, f32)> = None;
		for j in 0..=SUBS {
			let t = j as f32 / SUBS as f32;
			let pt = cubic_bezier(b0, b1, b2, b3, t);
			let r = c0.thickness + (c1.thickness - c0.thickness) * t;

			if let Some((pp, pr)) = prev {
				let res = tapered_segment_sample(p, pp, pr, pt, r);
				best = Some(match best {
					None => res,
					Some(b) if res.distance < b.distance => res,
					Some(b) => b,
				});
			}
			prev = Some((pt, r));
		}
	}

	best.unwrap_or_else(FieldSample::infinite)
}

// ─── Transform (five-step world↔local) ───────────────────────────────────────

fn transform_sample(p: Vec3, shape: &Shape, m: &Mat4) -> FieldSample {
	// 1. Transform query point to local (undeformed) space.
	let p_local = m.inverse().transform_point3(p);

	// 2+3. Closest point in local space — no scale distortion here.
	let local = shape.query(p_local);

	// 4. Transform closest point back to world space (exact surface point).
	let q_world = m.transform_point3(local.closest_point.mint());

	// 5. True world-space distance to that surface point.
	//    Under non-uniform scale this may not be the *minimum* world distance,
	//    but it is the distance to a valid surface point (bounded overestimate).
	let off = p - q_world;
	let world_dist = off.length();
	let signed_dist = local.distance.signum() * world_dist;

	// Gradient: surface normals transform by (M⁻¹)ᵀ.
	// The local gradient is already outward; (M⁻¹)ᵀ preserves that for
	// positive-determinant transforms (no reflections needed in XR).
	let grad_world = m
		.inverse()
		.transpose()
		.transform_vector3(local.gradient.mint())
		.normalize_or_zero();

	FieldSample {
		closest_point: q_world.into(),
		distance: signed_dist,
		gradient: grad_world.into(),
	}
}

// ─── Union (routed VDF) ───────────────────────────────────────────────────────

/// Union with routed-VDF interior handling.
///
/// **Exterior** (min distance ≥ 0): `min(d_i)` — exact Euclidean distance.
///
/// **Interior**: for each shape whose surface contains the query point, test
/// whether its closest surface point is *outside* every sibling shape.  If so,
/// that surface point is on the actual union boundary — return it immediately.
/// This gives the correct gradient and an underestimate of the true interior
/// distance (it may not be the absolute minimum, but it is a valid boundary
/// point and avoids seam artefacts in all but the corner-corner zone).
///
/// **Corner-corner fallback**: if routing cannot identify a clean boundary point,
/// fall back to hard `min`.  Interior seam artefacts can appear here, but the
/// zone is geometrically small (it requires both shapes' closest surface points
/// to be mutually occluded).
fn union_sample(p: Vec3, shapes: &[Shape]) -> FieldSample {
	if shapes.is_empty() {
		return FieldSample::infinite();
	}

	let results: Vec<FieldSample> = shapes.iter().map(|s| s.query(p)).collect();

	// Hard min — exact for exterior, fallback for interior.
	let min_result = results
		.iter()
		.copied()
		.min_by(|a, b| {
			a.distance
				.partial_cmp(&b.distance)
				.unwrap_or(std::cmp::Ordering::Equal)
		})
		.unwrap();

	// Exterior: hard min is the exact union SDF.
	if min_result.distance >= 0.0 {
		return min_result;
	}

	// Interior: attempt to route to the true boundary surface.
	for (i, result) in results.iter().enumerate() {
		if result.distance >= 0.0 {
			continue;
		} // p not inside shape i

		let cp = result.closest_point;

		// Is this shape's closest point outside (or on) every sibling?
		let on_boundary = results.iter().enumerate().all(|(j, _)| {
			if i == j {
				return true;
			}
			// A small negative tolerance avoids numerical rejection at
			// near-tangent intersections.
			shapes[j].query(cp.mint()).distance >= -1e-3
		});

		if on_boundary {
			return *result; // exact boundary point, correct gradient
		}
	}

	// Corner-corner zone: fall back to hard min.
	min_result
}

// ─── Smooth union ─────────────────────────────────────────────────────────────

fn smooth_union_sample(p: Vec3, shapes: &[Shape], k: f32) -> FieldSample {
	if shapes.is_empty() {
		return FieldSample::infinite();
	}

	let mut acc = shapes[0].query(p);

	for shape in &shapes[1..] {
		let b = shape.query(p);
		// h = 1 → prefer acc, h = 0 → prefer b
		let h = (0.5 + 0.5 * (b.distance - acc.distance) / k).clamp(0.0, 1.0);
		let d = acc.distance * h + b.distance * (1.0 - h) - k * h * (1.0 - h);
		// Blend closest points and gradients proportionally.
		let closest = Vec3::lerp(b.closest_point.mint(), acc.closest_point.mint(), h).into();
		let gradient = Vec3::lerp(b.gradient.mint(), acc.gradient.mint(), h)
			.normalize_or_zero()
			.into();
		acc = FieldSample {
			closest_point: closest,
			distance: d,
			gradient,
		};
	}

	acc
}

// ─── Sweep / Minkowski sum ────────────────────────────────────────────────────

/// Support function `h_K(d) = max_{x∈K} ⟨x, d⟩` for a unit direction `d`.
///
/// Transforms correctly under any linear map A via `h_{AK}(d) = h_K(Aᵀd)`,
/// so this naturally handles child transforms (see the `Transform` match arm).
fn support_fn(shape: &Shape, d: Vec3) -> f32 {
	match shape {
		Shape::Sphere { radius } => *radius,

		Shape::Box { size } => {
			let hs = (*size).mint::<Vec3>() * 0.5;
			hs.x * d.x.abs() + hs.y * d.y.abs() + hs.z * d.z.abs()
		}

		// Capsule = line_segment ⊕ Sphere  →  support adds linearly.
		Shape::Capsule { length, radius } => d.y.abs() * length * 0.5 + radius,

		// Cylinder support: half-length contributes along Y, radius in XZ.
		Shape::Cylinder { length, radius } => {
			let xz = (d.x * d.x + d.z * d.z).sqrt();
			d.y.abs() * length * 0.5 + xz * radius
		}

		// Torus ≈ ring_circle ⊕ disk  →  approximate support.
		Shape::Torus {
			major_radius,
			minor_radius,
		} => {
			let xz = (d.x * d.x + d.z * d.z).sqrt();
			xz * major_radius + minor_radius
		}

		// Transform: h_{AK}(d) = h_K(Aᵀd).  Magnitude of Aᵀd scales the result.
		Shape::Transform { shape, transform } => {
			let d_local = transform.mint::<Mat4>().transpose().transform_vector3(d);
			let scale = d_local.length();
			if scale > 1e-10 {
				support_fn(shape, d_local / scale) * scale
			} else {
				0.0
			}
		}

		// Union of convex shapes: max support.
		Shape::Union { shapes } => shapes
			.iter()
			.map(|s| support_fn(s, d))
			.fold(0.0_f32, f32::max),

		// Spline and other complex shapes: conservative underestimate (0).
		// TODO: implement via convex hull approximation.
		_ => 0.0,
	}
}

/// Minkowski sum approximation: `d(A ⊕ B)(p) ≈ d(A)(p) − h_B(∇d_A(p))`.
///
/// Exact when A is convex and a true SDF.  The gradient is invariant under
/// Minkowski sum with a convex body (it only shifts the surface outward).
///
/// Common use: `Sweep { surface: Box, sweeper: Sphere { radius: 0.05 } }`
/// produces a rounded box at zero extra query cost.
fn sweep_sample(p: Vec3, surface: &Shape, sweeper: &Shape) -> FieldSample {
	let mut res = surface.query(p);
	let s = support_fn(sweeper, res.gradient.mint());
	res.distance -= s;
	// Shift the closest point outward by the sweep extent along the gradient.
	res.closest_point =
		(res.closest_point.mint::<Vec3>() - (res.gradient.mint::<Vec3>() * s)).into();
	res
}

// ─── Tests ────────────────────────────────────────────────────────────────────

fn near(a: f32, b: f32, eps: f32, msg: &str) {
	assert!((a - b).abs() < eps, "{msg}: got {a:.6}, expected {b:.6}");
}

#[test]
fn field_shape_sphere_exterior_distance_and_gradient() {
	let r = sphere_sample(Vec3::new(3.0, 0.0, 0.0), 1.0);
	near(r.distance, 2.0, 1e-5, "sphere exterior dist");
	assert!(
		r.gradient.mint::<Vec3>().dot(Vec3::X) > 0.999,
		"gradient outward"
	);
}

#[test]
fn field_shape_sphere_interior_gradient_still_outward() {
	let r = sphere_sample(Vec3::new(0.5, 0.0, 0.0), 1.0);
	near(r.distance, -0.5, 1e-5, "sphere interior dist");
	// Gradient must point *outward* (same direction as the position) even inside.
	assert!(
		r.gradient.mint::<Vec3>().dot(Vec3::X) > 0.999,
		"interior gradient outward"
	);
}

#[test]
fn field_shape_box_exterior() {
	let r = box_sample(Vec3::new(1.0, 0.0, 0.0), Vec3::splat(0.5));
	near(r.distance, 0.5, 1e-5, "box exterior dist");
	assert!(
		r.gradient.mint::<Vec3>().dot(Vec3::X) > 0.999,
		"box exterior gradient"
	);
}

#[test]
fn field_shape_box_interior_nearest_face() {
	let r = box_sample(Vec3::new(0.4, 0.0, 0.0), Vec3::splat(0.5));
	near(r.distance, -0.1, 1e-5, "box interior dist");
	assert!(
		r.gradient.mint::<Vec3>().dot(Vec3::X) > 0.999,
		"box interior gradient toward +X face"
	);
}

#[test]
fn field_shape_cylinder_interior_gradient_outward() {
	// Interior point near the barrel, not the caps.
	let r = cylinder_sample(Vec3::new(0.3, 0.0, 0.0), 1.0, 0.5);
	assert!(r.distance < 0.0, "inside cylinder");
	// Gradient must point outward (+X direction for this point).
	assert!(
		r.gradient.mint::<Vec3>().dot(Vec3::X) > 0.5,
		"cylinder interior gradient outward"
	);
}

#[test]
fn field_shape_capsule_contains_point_on_axis() {
	let r = capsule_sample(Vec3::ZERO, 1.0, 0.5);
	near(r.distance, -0.5, 1e-5, "capsule centre dist");
}

#[test]
fn field_shape_torus_exterior_equatorial() {
	let r = torus_sample(Vec3::new(2.5, 0.0, 0.0), 2.0, 0.3);
	near(r.distance, 0.2, 1e-4, "torus exterior dist");
}

#[test]
fn field_shape_torus_interior_gradient_outward() {
	// Point just inside the tube, displaced in Y from the ring plane.
	// Ring point is at (2.0, 0.0, 0.0); to_p = (0, 0.1, 0) → gradient = +Y.
	let r = torus_sample(Vec3::new(2.0, 0.1, 0.0), 2.0, 0.3);
	assert!(r.distance < 0.0, "inside torus tube");
	assert!(
		r.gradient.mint::<Vec3>().dot(Vec3::Y) > 0.9,
		"torus interior gradient outward in Y"
	);
}

#[test]
fn field_shape_transform_uniform_scale() {
	// Sphere radius 1 scaled uniformly × 2 → effective radius 2.
	let s = Shape::Transform {
		shape: Box::new(Shape::Sphere { radius: 1.0 }),
		transform: Mat4::from_scale(Vec3::splat(2.0)).into(),
	};
	let r = s.query(Vec3::new(4.0, 0.0, 0.0));
	near(r.distance, 2.0, 1e-4, "scaled sphere world dist");
}

#[test]
fn field_shape_union_exterior_exact() {
	// Two spheres at (±1.5, 0, 0); query at (3, 0, 0) → dist to nearer sphere = 0.5.
	let shapes = vec![
		Shape::Transform {
			shape: Box::new(Shape::Sphere { radius: 1.0 }),
			transform: Mat4::from_translation(Vec3::new(1.5, 0.0, 0.0)).into(),
		},
		Shape::Transform {
			shape: Box::new(Shape::Sphere { radius: 1.0 }),
			transform: Mat4::from_translation(Vec3::new(-1.5, 0.0, 0.0)).into(),
		},
	];
	let u = Shape::Union { shapes };
	let r = u.query(Vec3::new(3.0, 0.0, 0.0));
	near(r.distance, 0.5, 1e-4, "union exterior dist");
	assert!(
		r.gradient.mint::<Vec3>().dot(Vec3::X) > 0.99,
		"union exterior gradient"
	);
}

#[test]
fn field_shape_sweep_sphere_rounds_box() {
	// Box(1×1×1) ⊕ Sphere(0.1): point at (0.6, 0, 0) → box dist = 0.1, rounded = 0.0.
	let s = Shape::Sweep {
		surface: Box::new(Shape::Box {
			size: Vec3::splat(1.0).into(),
		}),
		sweeper: Box::new(Shape::Sphere { radius: 0.1 }),
	};
	let r = s.query(Vec3::new(0.6, 0.0, 0.0));
	near(r.distance, 0.0, 1e-4, "rounded box surface");
}
