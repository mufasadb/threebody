use nannou::prelude::*;
use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalise(&self) -> Self {
        let m = self.magnitude();
        Vec3::new(self.x / m, self.y / m, self.z / m)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

// Scalar multiply: vec * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// Component-wise multiply: vec * vec
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

// ------- Body -------

const SOFTENING: f64 = 5.0;  // prevents force spike when bodies pass very close

#[derive(Clone, Debug)]
struct Body {
    mass:   f64,
    radius: f64,   // visual only — doesn't affect physics
    pos:    Vec3,
    vel:    Vec3,
    acc:    Vec3,  // stored between frames for leapfrog half-kick
}

impl Body {
    fn new(mass: f64, radius: f64, pos: Vec3, vel: Vec3) -> Self {
        Body { mass, radius, pos, vel, acc: Vec3::new(0.0, 0.0, 0.0) }
    }

    /// Acceleration this body receives due to gravity from `other`.
    /// Returns a/m (acceleration, not force) so we don't need to divide by self.mass.
    fn acc_from(&self, other: &Body) -> Vec3 {
        let r = other.pos - self.pos;          // vector pointing toward other body
        let dist_sq = r.x*r.x + r.y*r.y + r.z*r.z + SOFTENING*SOFTENING;
        let dist = dist_sq.sqrt();
        let mag = other.mass / (dist_sq * dist); // G=1 units; GM/r²
        r * mag
    }
}

fn main() {
    nannou::app(model).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Three Body Problem")
        .size(900, 900)
        .view(view)
        .build()
        .unwrap();

    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

#[cfg(test)]
mod tests {
    use super::{Vec3, Body};

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    fn vec_approx_eq(a: Vec3, b: Vec3) -> bool {
        approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
    }

    #[test]
    fn test_add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let result = a + b;
        assert!(vec_approx_eq(result, Vec3::new(5.0, 7.0, 9.0)));
    }

    #[test]
    fn test_sub() {
        let a = Vec3::new(4.0, 5.0, 6.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        let result = a - b;
        assert!(vec_approx_eq(result, Vec3::new(3.0, 3.0, 3.0)));
    }

    #[test]
    fn test_scalar_mul_clean() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let result = v * 3.0;
        assert!(vec_approx_eq(result, Vec3::new(6.0, 12.0, 18.0)));
    }

    #[test]
    fn test_scalar_mul_messy() {
        // 0.1 can't be represented exactly in binary floating point
        // this checks we stay within acceptable precision
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = v * 0.1;
        assert!(vec_approx_eq(result, Vec3::new(0.1, 0.2, 0.3)));
    }

    #[test]
    fn test_scalar_mul_irrational() {
        // multiply by something that produces an irrational-ish result
        let v = Vec3::new(1.0, 1.0, 1.0);
        let result = v * std::f64::consts::PI;
        assert!(vec_approx_eq(result, Vec3::new(std::f64::consts::PI, std::f64::consts::PI, std::f64::consts::PI)));
    }

    #[test]
    fn test_magnitude_pythagorean() {
        // 3,4,0 triangle — magnitude should be exactly 5
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert!(approx_eq(v.magnitude(), 5.0));
    }

    #[test]
    fn test_magnitude_3d() {
        // (1,1,1) — magnitude is √3
        let v = Vec3::new(1.0, 1.0, 1.0);
        assert!(approx_eq(v.magnitude(), 3.0_f64.sqrt()));
    }

    #[test]
    fn test_normalise_gives_unit_length() {
        let v = Vec3::new(3.0, 4.0, 5.0);
        let n = v.normalise();
        assert!(approx_eq(n.magnitude(), 1.0));
    }

    #[test]
    fn test_normalise_preserves_direction() {
        // normalising shouldn't change the ratio between components
        let v = Vec3::new(0.0, 0.0, 7.0);
        let n = v.normalise();
        assert!(vec_approx_eq(n, Vec3::new(0.0, 0.0, 1.0)));
    }

    // ------- Body tests -------

    fn make_body(mass: f64, x: f64, y: f64) -> Body {
        Body::new(mass, 5.0, Vec3::new(x, y, 0.0), Vec3::new(0.0, 0.0, 0.0))
    }

    #[test]
    fn test_acc_from_points_toward_other_body() {
        // body at origin, other to the right — acceleration should be positive x
        let a = make_body(1.0, 0.0, 0.0);
        let b = make_body(1.0, 100.0, 0.0);
        let acc = a.acc_from(&b);
        assert!(acc.x > 0.0, "should pull toward +x");
        assert!(approx_eq(acc.y, 0.0));
        assert!(approx_eq(acc.z, 0.0));
    }

    #[test]
    fn test_acc_from_is_symmetric_in_direction() {
        // reaction should be equal and opposite
        let a = make_body(1.0, 0.0, 0.0);
        let b = make_body(1.0, 100.0, 0.0);
        let acc_a = a.acc_from(&b);
        let acc_b = b.acc_from(&a);
        assert!(approx_eq(acc_a.x, -acc_b.x));
    }

    #[test]
    fn test_acc_from_scales_with_mass() {
        // doubling the attracting body's mass should double the acceleration
        let a = make_body(1.0, 0.0, 0.0);
        let b1 = make_body(1.0, 100.0, 0.0);
        let b2 = make_body(2.0, 100.0, 0.0);
        let acc1 = a.acc_from(&b1);
        let acc2 = a.acc_from(&b2);
        assert!(approx_eq(acc2.x, acc1.x * 2.0));
    }

    #[test]
    fn test_acc_from_softening_prevents_singularity() {
        // bodies at the same position should not produce infinite/NaN acceleration
        let a = make_body(1e10, 0.0, 0.0);
        let b = make_body(1e10, 0.0, 0.0);
        let acc = a.acc_from(&b);
        assert!(acc.x.is_finite());
        assert!(acc.y.is_finite());
        assert!(acc.z.is_finite());
    }
}
