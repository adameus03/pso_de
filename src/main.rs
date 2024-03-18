#![feature(generic_arg_infer)]
#![allow(clippy::needless_return)]

const FN_SIZE: usize = 30;

use std::ops::AddAssign;
use std::ptr;

use clap::{Parser, Subcommand};
use particle_swarm::pso_de::WorldState;
use particle_swarm::de;

#[derive(Parser, Clone, Debug)]
struct Config {
	#[arg(long, value_delimiter = ',', num_args = 1.., required = true)]
	functions: Vec<String>,
	#[arg(long = "crossover")]
	crossover_possibility: f64,
	#[arg(long = "amplifier")]
	diff_weight: f64,
	#[arg(long = "diff-pop")]
	diff_population: usize,
	#[arg(long = "diff-iters")]
	differential_iterations: usize,
	#[arg(long = "lambda")]
	lambda: f64,
	#[arg(long = "try-count")]
	try_count: Option<usize>,
	#[command(subcommand)]
	command: Option<ComputationMode>,
}

#[derive(Subcommand, Clone, Debug)]
enum ComputationMode {
	DiffPart {
		#[arg(long)]
		particles: usize,
		#[arg(long = "part-iters")]
		particle_iterations: usize,
		#[arg(long)]
		social_coefficient: f64,
		#[arg(long)]
		cognitive_coefficient: f64,
		#[arg(long)]
		inertia_coefficient: f64
	},
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

fn main() {
	let builtin_fns = particle_swarm::functions::create_function_list::<FN_SIZE>();
	let config = Config::parse();
	if config.functions.is_empty() {
		panic!("No functions given");
	}
	let test_functions = config.functions.into_iter().map(|s| {
		return (s.clone(), builtin_fns.get(&s).unwrap());
	}).collect::<Vec<_>>();


	if let Some(tries) = config.try_count {
		for (function_name, function) in test_functions {
			let bounds = function.get_bounds();
			let func = function.get_function();
			let c_func = function.get_c_function();
			let tries_per_thread = tries.div_ceil(num_cpus::get());
			let mut threads = Vec::with_capacity(num_cpus::get());
			
			match config.command {
				Some(ComputationMode::DiffPart { particles, particle_iterations, social_coefficient, cognitive_coefficient, inertia_coefficient }) => {
					let world = WorldState::new(particles, func, bounds, social_coefficient, cognitive_coefficient, inertia_coefficient, config.diff_population, config.crossover_possibility, config.diff_weight, config.lambda, config.differential_iterations);
					for _ in 0..num_cpus::get() {
						let mut thread_world = world.clone();
						threads.push(std::thread::spawn(move || {
							let mut run_stats = BatchRunData::new();
							for _ in 0..tries_per_thread {
								thread_world.do_all_iterations(particle_iterations);
								run_stats += func(thread_world.best_solution);
								thread_world.reset();
							}
							return run_stats;
						}));
					}
				}
				None => {
					let stop_condition = de::DeStopCondition {
						stype: de::DeStopType::StopAfterIters,
						union: de::DeLimitation { iters: config.differential_iterations as u64 }
					};
					let mut config = de::DeConfig {
						population_size: config.diff_population as u32,
						crossover_probability: config.crossover_possibility,
						amplification_factor: config.diff_weight,
						lambda: 0.5,
						stop_condition: stop_condition
					};

					let mut target = de::DeOptimizationTarget {
						f: c_func,
						num_dimensions: 30,
						left_bound: -10.0,
						right_bound: 10.0
					};

					for _ in 0..num_cpus::get() {
						threads.push(std::thread::spawn(move || {
							let mut run_stats = BatchRunData::new();
							for _ in 0..tries_per_thread {
								let result = unsafe { de::de_minimum(&mut target, &mut config, ptr::null_mut()) };
								unsafe { run_stats += c_func(result, ptr::null_mut()); }
							}
							return run_stats;
						}));
					}
					
				}
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
			let c_func = function.get_c_function();
			let func = function.get_function();
			let bounds = function.get_bounds();
			match config.command {
				Some(ComputationMode::DiffPart { particles, particle_iterations, social_coefficient, cognitive_coefficient, inertia_coefficient }) => {
					threads.push(std::thread::spawn(move || {
						let mut world = WorldState::new(particles, func, bounds, social_coefficient, cognitive_coefficient, inertia_coefficient, config.diff_population, config.crossover_possibility, config.diff_weight, config.lambda, config.differential_iterations);
						world.do_all_iterations(particle_iterations);
						println!("{}: Found optimum at {:?} = {}", function_name, world.best_solution.coordinates, func(world.best_solution));
					}));
				}
				None => {
					threads.push(std::thread::spawn(move || {
						let stop_condition = de::DeStopCondition {
							stype: de::DeStopType::StopAfterIters,
							union: de::DeLimitation { iters: config.differential_iterations as u64 }
						};
						let mut config = de::DeConfig {
							population_size: config.diff_population as u32,
							crossover_probability: config.crossover_possibility,
							amplification_factor: config.diff_weight,
							lambda: 0.5,
							stop_condition: stop_condition
						};
						let mut target = de::DeOptimizationTarget {
							f: c_func,
							num_dimensions: 30,
							left_bound: -10.0,
							right_bound: 10.0
						};
						println!("Calling de_minimum");
						let mut result = unsafe { de::de_minimum(&mut target, &mut config, ptr::null_mut()) };
						println!("de_minimum call returned.");
						// Print the de_minimum result coordinates
						for i in 0..result.num_dimensions as isize {
							unsafe {
								println!("Result coordinate {}: {}", i, *result.coordinates.offset(i));
							}
						}

						unsafe {
							println!("Extreme function value: {}", c_func(result, ptr::null_mut()));
						}
						
						// Free the result
						unsafe { de::de_vector_free_coordinates(&mut result) };
					}));
				}
			}
		}
		for thread in threads {
			thread.join().unwrap();
		}
	}
}
