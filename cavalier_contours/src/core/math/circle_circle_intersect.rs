use super::Vector2;
use crate::core::traits::Real;

/// Holds the result of finding the intersect between two circles.
#[derive(Debug, Copy, Clone)]
pub enum CircleCircleIntr<T>
where
    T: Real,
{
    /// No intersects found.
    NoIntersect,
    /// One tangent intersect point found.
    TangentIntersect {
        /// Holds the tangent intersect point.
        point: Vector2<T>,
    },
    /// Simple case of two intersect points found.
    TwoIntersects {
        /// Holds the first intersect point.
        point1: Vector2<T>,
        /// Holds the second intersect point.
        point2: Vector2<T>,
    },
    /// Circles overlap each other (same circle).
    Overlapping,
}

/// Finds the intersects between two circles.
/// The circles are defined by their radii: `radius1`, `radius2` and their centers: `center1`, `center2`.
///
/// # Examples
///
/// ```
/// # use cavalier_contours::core::math::*;
/// // the trusty old unit-circle
/// let unit_center: Vector2<f64> = Vector2::zero();
/// let unit_radius = 1.0f64;
///
/// // intersect with some other circle
/// let intersect = circle_circle_intr(unit_radius, unit_center, 2.0f64.sqrt(), Vector2::new(0.0, 1.0));
///
/// match intersect {
///     CircleCircleIntr::TwoIntersects{point1, point2} => {
///         assert!(point1.fuzzy_eq(Vector2::new(1.0, 0.0)));
///         assert!(point2.fuzzy_eq(Vector2::new(-1.0, 0.0)));
///     },
///     _ => assert!(false)
/// }
/// ```
///
pub fn circle_circle_intr<T>(
    radius1: T,
    center1: Vector2<T>,
    radius2: T,
    center2: Vector2<T>,
) -> CircleCircleIntr<T>
where
    T: Real,
{
    // Reference algorithm: http://paulbourke.net/geometry/circlesphere/
    use CircleCircleIntr::*;

    let cv = center2 - center1;
    let d2 = cv.dot(cv);
    let d = d2.sqrt();

    if d.fuzzy_eq_zero() {
        // same center position
        if radius1.fuzzy_eq(radius2) {
            return Overlapping;
        }
        return NoIntersect;
    }

    // different center position
    if !d.fuzzy_lt(radius1 + radius2) || !d.fuzzy_gt((radius1 - radius2).abs()) {
        // distance relative to radii is too large or too small for intersects to occur
        return NoIntersect;
    }

    let rad1_sq = radius1 * radius1;
    let a = (rad1_sq - radius2 * radius2 + d2) / (T::two() * d);
    let midpoint = center1 + cv.scale(a / d);
    let diff = rad1_sq - a * a;

    if diff < T::zero() {
        return TangentIntersect { point: midpoint };
    }

    let h = diff.sqrt();
    let h_over_d = h / d;
    let x_term = h_over_d * cv.y;
    let y_term = h_over_d * cv.x;

    let pt1 = Vector2::new(midpoint.x + x_term, midpoint.y - y_term);
    let pt2 = Vector2::new(midpoint.x - x_term, midpoint.y + y_term);

    if pt1.fuzzy_eq(pt2) {
        return TangentIntersect { point: pt1 };
    }

    TwoIntersects {
        point1: pt1,
        point2: pt2,
    }
}
