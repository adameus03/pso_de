use std::ops::{Mul, Add, Sub, AddAssign};

use serde::Serialize;

#[derive(Clone, Debug, Copy, Serialize)]
pub struct Vector2D {
	pub x: f64,
	pub y: f64
}

impl Vector2D {
	pub fn new(x: f64, y: f64) -> Self {
		return Vector2D {
			x, y
		};
	}
	pub fn clamp(&mut self, x_bounds: (f64, f64), y_bounds: (f64, f64)) {
		self.x = self.x.clamp(x_bounds.0, x_bounds.1);
		self.y = self.y.clamp(y_bounds.0, y_bounds.1);
	}
}

impl Mul<f64> for Vector2D {
	type Output = Vector2D;

	fn mul(self, rhs: f64) -> Self::Output {
		return Vector2D {
			x: self.x * rhs,
			y: self.y * rhs,
		};
	}
}

impl Sub for Vector2D {
	type Output = Vector2D;

	fn sub(self, rhs: Self) -> Self::Output {
		return Vector2D {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		};
	}
}

impl Add for Vector2D {
	type Output = Vector2D;

	fn add(self, rhs: Self) -> Self::Output {
		return Vector2D {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		};
	}
}

impl AddAssign for Vector2D {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl Add<f64> for Vector2D {
	type Output = Vector2D;

	fn add(self, rhs: f64) -> Self::Output {
		return Vector2D {
			x: self.x + rhs,
			y: self.y + rhs,
		};
	}
}