use std::f64::consts::{PI, E};
use particle_swarm::Vector2D;

pub fn ackley(input: Vector2D) -> f64 {
	return -20.0 * (-0.2 * (0.5 * (input.x.powi(2) + input.y.powi(2))).sqrt()).exp() -
		(0.5 * ((2.0 * PI * input.x).cos() + (2.0 * PI * input.y).cos())).exp() +
		E + 20.0;
}

pub fn beale(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (1.5 - x + x * y).powi(2) + (2.25 - x + x * y.powi(2)).powi(2) + (2.625 - x + x * y.powi(3)).powi(2);
}

pub fn goldstein_price(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (1.0 + (x + y + 1.0).powi(2) * (19.0 - 14.0 * x + 3.0 * x.powi(2) - 14.0 * y + 6.0 * x * y + 3.0 * y.powi(2))) *
		(30.0 + (2.0 * x - 3.0 * y).powi(2) * (18.0 - 32.0 * x + 12.0 * x.powi(2) + 48.0 * y - 36.0 * x * y + 27.0 * y.powi(2)));
}

pub fn booth(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (x + 2.0 * y - 7.0).powi(2) + (2.0 * x + y - 5.0).powi(2);
}

pub fn bukin(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return 100.0 * (y - 0.01 * x.powi(2)).abs().sqrt() + 0.01 * (x + 10.0).abs();
}

pub fn matyas(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return 0.26 * (x.powi(2) + y.powi(2)) - 0.48 * x * y;
}

pub fn levi(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (3.0 * PI * x).sin().sin() + (x - 1.0).powi(2) * (1.0 + (3.0 * PI * y).sin().sin()) + ((y - 1.0).powi(2) * (1.0 + (2.0 * PI * y).sin().sin()));
}

pub fn himmelblau(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (x.powi(2) + y - 11.0).powi(2) + (x + y.powi(2) - 7.0).powi(2);
}

pub fn three_humps(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return 2.0 * x.powi(2) - 1.05 * x.powi(4) + x.powi(6) / 6.0 + x * y + y.powi(2);
}

pub fn easom(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return -(x.cos()) * y.cos() * (-((x - PI).powi(2) + (y - PI).powi(2))).exp();
}

pub fn cross_in_tray(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return -0.0001 * ((x.sin() * y.sin() * (100.0 - (x.powi(2) + y.powi(2)).sqrt() / PI).abs().exp()).abs()).powf(0.1);
}

pub fn eggholder(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return -(y + 47.0) * (x / 2.0 + y + 47.0).abs().sqrt().sin() -
		x * (x - (y + 47.0)).abs().sqrt().sin();
}

pub fn holder(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return -((x.sin() * y.cos() * (1.0 - (x.powi(2) + y.powi(2)).sqrt() / PI).abs().exp()).abs());
}

pub fn mccormick(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (x + y).sin() + (x - y).powi(2) - 1.5 * x + 2.5 * y + 1.0;
}

pub fn schaffer2(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return 0.5 + ((x.powi(2) - y.powi(2)).sin().sin() - 0.5) / (1.0 + 0.001 * (x.powi(2) + y.powi(2))).powi(2);
}

pub fn schaffer4(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	// Error on wikipedia
	// it claims that it's `cos^2 [sin(|x^2 - y^2|)]` but the result doesn't match up
	// but implementation over at https://www.sfu.ca/~ssurjano/Code/schaffer4m.html shows that it's an exponent instead of a second cosine
	return 0.5 + ((x.powi(2) - y.powi(2)).abs().sin().cos().powi(2) - 0.5) / (1.0 + 0.001 * (x.powi(2) + y.powi(2))).powi(2);
}