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
    use super::Vec3;

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
}
