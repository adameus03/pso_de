use std::{f64::consts::{PI, E}, collections::HashMap};
use crate::vector::Vector2D;

pub trait Function {
	fn get_function(&self) -> fn(input: Vector2D) -> f64;
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)); // x bounds, y bounds
}

fn ackley(input: Vector2D) -> f64 {
	return -20.0 * (-0.2 * (0.5 * (input.x.powi(2) + input.y.powi(2))).sqrt()).exp() -
		(0.5 * ((2.0 * PI * input.x).cos() + (2.0 * PI * input.y).cos())).exp() +
		E + 20.0;
}

struct Ackley {}
impl Function for Ackley {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return ackley;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-5.0, 5.0), (-5.0, 5.0));
	}
}

fn beale(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (1.5 - x + x * y).powi(2) + (2.25 - x + x * y.powi(2)).powi(2) + (2.625 - x + x * y.powi(3)).powi(2);
}

struct Beale {}
impl Function for Beale {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return beale;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-4.5, 4.5), (-4.5, 4.5));
	}
}

fn goldstein_price(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (1.0 + (x + y + 1.0).powi(2) * (19.0 - 14.0 * x + 3.0 * x.powi(2) - 14.0 * y + 6.0 * x * y + 3.0 * y.powi(2))) *
		(30.0 + (2.0 * x - 3.0 * y).powi(2) * (18.0 - 32.0 * x + 12.0 * x.powi(2) + 48.0 * y - 36.0 * x * y + 27.0 * y.powi(2)));
}

struct GoldsteinPrice {}
impl Function for GoldsteinPrice {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return goldstein_price;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-2.0, 2.0), (-2.0, 2.0));
	}
}

fn booth(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (x + 2.0 * y - 7.0).powi(2) + (2.0 * x + y - 5.0).powi(2);
}

struct Booth {}
impl Function for Booth {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return booth;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-10.0, 10.0), (-10.0, 10.0));
	}
}

fn bukin(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return 100.0 * (y - 0.01 * x.powi(2)).abs().sqrt() + 0.01 * (x + 10.0).abs();
}

struct Bukin {}
impl Function for Bukin {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return bukin;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-15.0, -5.0), (-3.0, 3.0));
	}
}

fn matyas(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return 0.26 * (x.powi(2) + y.powi(2)) - 0.48 * x * y;
}

struct Matyas {}
impl Function for Matyas {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return matyas;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-10.0, 10.0), (-10.0, 10.0));
	}
}

fn levi(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (3.0 * PI * x).sin().sin() + (x - 1.0).powi(2) * (1.0 + (3.0 * PI * y).sin().sin()) + ((y - 1.0).powi(2) * (1.0 + (2.0 * PI * y).sin().sin()));
}

struct Levi {}
impl Function for Levi {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return levi;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-10.0, 10.0), (-10.0, 10.0));
	}
}

fn himmelblau(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (x.powi(2) + y - 11.0).powi(2) + (x + y.powi(2) - 7.0).powi(2);
}

struct Himmelblau {}
impl Function for Himmelblau {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return himmelblau;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-5.0, 5.0), (-5.0, 5.0));
	}
}

fn three_humps(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return 2.0 * x.powi(2) - 1.05 * x.powi(4) + x.powi(6) / 6.0 + x * y + y.powi(2);
}

struct ThreeHumps {}
impl Function for ThreeHumps {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return three_humps;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-5.0, 5.0), (-5.0, 5.0));
	}
}

fn easom(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return -(x.cos()) * y.cos() * (-((x - PI).powi(2) + (y - PI).powi(2))).exp();
}

struct Easom {}
impl Function for Easom {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return easom;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-100.0, 100.0), (-100.0, 100.0));
	}
}

fn cross_in_tray(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return -0.0001 * ((x.sin() * y.sin() * (100.0 - (x.powi(2) + y.powi(2)).sqrt() / PI).abs().exp()).abs()).powf(0.1);
}

struct CrossInTray {}
impl Function for CrossInTray {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return cross_in_tray;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-10.0, 10.0), (-10.0, 10.0));
	}
}

fn eggholder(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return -(y + 47.0) * (x / 2.0 + y + 47.0).abs().sqrt().sin() -
		x * (x - (y + 47.0)).abs().sqrt().sin();
}

struct Eggholder {}
impl Function for Eggholder {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return eggholder;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-512.0, 512.0), (-512.0, 512.0));
	}
}

fn holder(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return -((x.sin() * y.cos() * (1.0 - (x.powi(2) + y.powi(2)).sqrt() / PI).abs().exp()).abs());
}

struct Holder {}
impl Function for Holder {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return holder;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-10.0, 10.0), (-10.0, 10.0));
	}
}

fn mccormick(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return (x + y).sin() + (x - y).powi(2) - 1.5 * x + 2.5 * y + 1.0;
}

struct McCormick {}
impl Function for McCormick {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return mccormick;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-1.5, 4.0), (-3.0, 4.0));
	}
}

fn schaffer2(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	return 0.5 + ((x.powi(2) - y.powi(2)).sin().powi(2) - 0.5) / (1.0 + 0.001 * (x.powi(2) + y.powi(2))).powi(2);
}

struct Schaffer2 {}
impl Function for Schaffer2 {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return schaffer2;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-100.0, 100.0), (-100.0, 100.0));
	}
}

fn schaffer4(input: Vector2D) -> f64 {
	let Vector2D { x, y } = input;
	// Error on wikipedia
	// it claims that it's `cos^2 [sin(|x^2 - y^2|)]` but the result doesn't match up
	// but implementation over at https://www.sfu.ca/~ssurjano/Code/schaffer4m.html shows that it's an exponent instead of a second cosine
	return 0.5 + ((x.powi(2) - y.powi(2)).abs().sin().cos().powi(2) - 0.5) / (1.0 + 0.001 * (x.powi(2) + y.powi(2))).powi(2);
}

pub struct Schaffer4 {}
impl Function for Schaffer4 {
	fn get_function(&self) -> fn(input: Vector2D) -> f64 {
		return schaffer4;
	}
	fn get_bounds(&self) -> ((f64, f64), (f64, f64)) {
		return ((-100.0, 100.0), (-100.0, 100.0));
	}
}

pub fn create_function_list() -> HashMap<String, Box<dyn Function>> {
	let mut result: HashMap<String, Box<dyn Function>> = HashMap::new();
	result.insert(String::from("ackley"), Box::new(Ackley {}));
	result.insert(String::from("beale"), Box::new(Beale {}));
	result.insert(String::from("goldstein-price"), Box::new(GoldsteinPrice {}));
	result.insert(String::from("booth"), Box::new(Booth {}));
	result.insert(String::from("bukin"), Box::new(Bukin {}));
	result.insert(String::from("matyas"), Box::new(Matyas {}));
	result.insert(String::from("levi"), Box::new(Levi {}));
	result.insert(String::from("himmelblau"), Box::new(Himmelblau {}));
	result.insert(String::from("three-humps"), Box::new(ThreeHumps {}));
	result.insert(String::from("easom"), Box::new(Easom {}));
	result.insert(String::from("cross-in-tray"), Box::new(CrossInTray {}));
	result.insert(String::from("eggholder"), Box::new(Eggholder {}));
	result.insert(String::from("holder"), Box::new(Holder {}));
	result.insert(String::from("mccormick"), Box::new(McCormick {}));
	result.insert(String::from("schaffer2"), Box::new(Schaffer2 {}));
	result.insert(String::from("schaffer4"), Box::new(Schaffer4 {}));
	return result;
}