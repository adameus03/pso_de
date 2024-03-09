use std::ops::AddAssign;

use clap::Parser;
use particle_swarm::{de, functions::Function, pso::WorldState};

#[derive(Parser, Clone, Debug)]
struct Config {
	#[arg(long, value_delimiter = ',', num_args = 1.., required = true)]
	functions: Vec<String>,
	#[arg(long)]
	particles: usize,
	#[arg(long, name = "social-coeff")]
	social_coeff: f64,
	#[arg(long, name = "cog-coeff")]
	cognitive_coeff: f64,
	#[arg(long, name = "inertia-coeff")]
	inertia_coeff: f64,
	#[arg(long)]
	iterations: usize,
	#[arg(long, name = "try-count", conflicts_with = "record")]
	try_count: Option<usize>,
	#[arg(long, conflicts_with = "try-count")]
	record: bool,
}

struct BatchRunData {
	pub min_result: f64,
	pub max_result: f64,
	pub average: f64,
	pub run_count: u32,
}

impl BatchRunData {
	fn new() -> Self {
		return Self {
			min_result: f64::MAX,
			max_result: f64::MIN,
			average: 0.0,
			run_count: 0,
		};
	}
}

impl AddAssign for BatchRunData {
	fn add_assign(&mut self, other: Self) {
		if other.max_result > self.max_result {
			self.max_result = other.max_result;
		}
		if other.min_result < self.min_result {
			self.min_result = other.min_result;
		}
		let self_sum = self.average * self.run_count as f64;
		let other_sum = other.average * other.run_count as f64;
		self.run_count += other.run_count;
		self.average = (self_sum + other_sum) / self.run_count as f64;
	}
}

impl AddAssign<f64> for BatchRunData {
	fn add_assign(&mut self, rhs: f64) {
		if rhs > self.max_result {
			self.max_result = rhs;
		}
		if rhs < self.min_result {
			self.min_result = rhs;
		}
		let previous_sum = self.average * self.run_count as f64;
		self.run_count += 1;
		self.average = (previous_sum + rhs) / self.run_count as f64;
		
	}
}

///<test functions for DE>
#[no_mangle]
pub unsafe extern "C" fn sphere(input: de::Vector) -> f64 {
	let mut result = 0.0;
	for i in 0..input.num_dimensions as isize {
		unsafe {
			result += *input.coordinates.offset(i) * *input.coordinates.offset(i);
		}
	}
	return result;
}

#[no_mangle]
pub unsafe extern "C" fn shifted_sphere(input: de::Vector) -> f64 {
	let mut result = 0.0;
	for i in 0..input.num_dimensions as isize {
		let mut a;
		unsafe {
			a = *input.coordinates.offset(i);
		}
		// Substract i+1
		a -= (i + 1) as f64;
		result += a * a;
	}
	return result;
}

// Quick&dirty test for de_minimum_stub and de_minimum
fn de_test() {
	println!("Hello from rust!");
	
	// [Test the DE library]
	let stop_condition = de::DeStopCondition {
		stype: de::DeStopType::StopAfterIters,
		union: de::DeLimitation { iters: 100 }
	};
	let mut config = de::DeConfig {
		population_size: 100,
		crossover_probability: 0.5,
		amplification_factor: 0.5,
		lambda: 0.5,
		stop_condition: stop_condition
	};
	// Use the holder function and its bounds (declared in src/functions.rs)
	let mut target = de::DeOptimizationTarget {
		//f: particle_swarm::functions::Holder::get_function(&self),
		f: shifted_sphere,
		num_dimensions: 2,
		/*left_bound::get_bounds().0[0],
		right_bound::get_bounds().1[0]*/
		left_bound: -10.0,
		right_bound: 10.0
	};
	// Call the DE library
	println!("Calling de_minimum_stub");
	let mut result = unsafe { de::de_minimum_stub(&mut target, &mut config) };
	println!("de_minimum_stub call returned.");
	// Print the de_minimum_stub result coordinates
	for i in 0..result.num_dimensions as isize {
		unsafe {
			println!("Result coordinate {}: {}", i, *result.coordinates.offset(i));
		}
	}
	
	// Free the result
	unsafe { de::de_vector_free_coordinates(&mut result) };

	println!("Calling de_minimum");
	result = unsafe { de::de_minimum(&mut target, &mut config) };
	println!("de_minimum call returned.");
	// Print the de_minimum result coordinates
	for i in 0..result.num_dimensions as isize {
		unsafe {
			println!("Result coordinate {}: {}", i, *result.coordinates.offset(i));
		}
	}
	
	// Free the result
	unsafe { de::de_vector_free_coordinates(&mut result) };
	
	// [/Test the DE library]
}
///</test functions for DE>

fn main() {
	de_test();
	return;

	let builtin_fns = particle_swarm::functions::create_function_list();
	let config = Config::parse();
	if config.functions.is_empty() {
		panic!("No functions given");
	}
	let test_functions = config.functions.into_iter().map(|s| {
		return (s.clone(), builtin_fns.get(&s).unwrap());
	}).collect::<Vec<_>>();

	if let Some(tries) = config.try_count {
		for (function_name, function) in test_functions {
			let (x_bounds, y_bounds) = function.get_bounds();
			let world = WorldState::new(config.particles, function.get_function(), x_bounds, y_bounds, config.social_coeff, config.cognitive_coeff, config.inertia_coeff);
			let tries_per_thread = tries.div_ceil(num_cpus::get());
			let mut threads = Vec::with_capacity(num_cpus::get());
			for _ in 0..num_cpus::get() {
				let mut thread_world = world.clone();
				threads.push(std::thread::spawn(move || {
					let mut run_stats = BatchRunData::new();
					for _ in 0..tries_per_thread {
						thread_world.do_all_iterations(config.iterations);
						run_stats += (world.function)(thread_world.best_solution);
						thread_world.reset();
					}
					return run_stats;
				}));
			}
			let result = threads.into_iter().map(|handle| handle.join().unwrap()).reduce(|mut a, b| {
				a += b;
				return a;
			}).unwrap();
			println!("{}: Finished {} runs. Max solution is {}. Average solution is {}. Min solution is {}.", function_name, result.run_count, result.max_result, result.average, result.min_result);
		}
	} else {
		let mut threads = Vec::new();
		for (function_name, function) in test_functions {
			let func = function.get_function();
			let (x_bounds, y_bounds) = function.get_bounds();
			threads.push(std::thread::spawn(move || {
				let mut world = WorldState::new(config.particles, func, x_bounds, y_bounds, config.social_coeff, config.cognitive_coeff, config.inertia_coeff);
				if config.record {
					let serialized_string = serde_json::to_string(&world.do_all_iters_with_record(config.iterations)).unwrap();
					std::fs::create_dir_all("output").unwrap();
					std::fs::write(format!("output/{}.json", function_name), serialized_string).unwrap();
				} else {
					world.do_all_iterations(config.iterations);
				}
				println!("{}: Found optimum at ({}; {}) = {}", function_name, world.best_solution.x, world.best_solution.y, func(world.best_solution));
			}));
		}
		for thread in threads {
			thread.join().unwrap();
		}
	}
}
