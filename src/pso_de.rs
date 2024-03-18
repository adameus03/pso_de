use libc::{c_double, c_void};

use crate::vector::VectorN;
use crate::de::{self};

#[derive(Debug, Clone)]
pub struct Particle<const N: usize> {
	pub current_speed: VectorN<N>,
	pub coordinates: VectorN<N>,
	pub best_found_solution: VectorN<N>, // of this particle
	best_found_solution_value: f64,
	pub bounds: (f64, f64), // lower, upper
	pub social_coefficient: f64,
	pub cognitive_coefficient: f64,
	pub inertia_coefficient: f64,
}

impl<const N: usize> Particle<N> {
	fn move_particle(&mut self, best_global_solution: VectorN<N>, random_source: &mut fastrand::Rng) {
		let inertia_part = self.current_speed * self.inertia_coefficient;
		let social_part = (best_global_solution - self.coordinates) * self.social_coefficient * random_source.f64();
		let self_part = (self.best_found_solution - self.coordinates) * self.cognitive_coefficient * random_source.f64();
		self.current_speed = inertia_part + social_part + self_part;
		self.coordinates += self.current_speed * 1.0;

		self.coordinates.clamp(self.bounds);
	}
}


pub unsafe extern "C" fn c_optimization_function_for_pso_control_params<const DIMENSIONS: usize>(input: de::Vector, user_data: *mut c_void) -> c_double {
	let world_state = &mut *(user_data as *mut WorldState<DIMENSIONS>);
	return optimization_function_for_pso_control_params(world_state, input);
}

fn optimization_function_for_pso_control_params<const DIMENSIONS: usize>(ws: &mut WorldState<DIMENSIONS>, control_coeffs: de::Vector) -> f64 {
	// Calculate and return loss
	unsafe {
		ws.social_coefficient = *control_coeffs.coordinates.offset(0);
		ws.cognitive_coefficient = *control_coeffs.coordinates.offset(1);
		ws.inertia_coefficient = *control_coeffs.coordinates.offset(2);
	}
	for particle in &mut ws.particles {
		particle.social_coefficient = ws.social_coefficient;
		particle.cognitive_coefficient = ws.cognitive_coefficient;
		particle.inertia_coefficient = ws.inertia_coefficient;
	}
	
	
	for particle in &mut ws.particles {
		particle.move_particle(ws.best_solution, &mut ws.random_generator);
	}

	let mut best_solution_value_for_pso_iteration = f64::INFINITY;
	for particle in &mut ws.particles {
		let particle_solution_val = (ws.function)(particle.coordinates);
		if particle_solution_val < best_solution_value_for_pso_iteration {
			best_solution_value_for_pso_iteration = particle_solution_val;
		}
	}
	
	return best_solution_value_for_pso_iteration;

}

#[derive(Debug, Clone)]
pub struct WorldState<const DIMENSIONS: usize> {
	pub particles: Vec<Particle<DIMENSIONS>>,
	pub function: fn(VectorN<DIMENSIONS>) -> f64,
	pub best_solution: VectorN<DIMENSIONS>,
	best_solution_value: f64,
	bounds: (f64, f64),
	particle_count: usize,
	social_coefficient: f64,
	cognitive_coefficient: f64,
	inertia_coefficient: f64,
	random_generator: fastrand::Rng,

	/* DE - specific */
	pub de_population_size: usize,
	pub de_crossover_probability: f64,
	pub de_diff_weight: f64,
	pub de_lambda: f64,
	pub de_num_iters: usize
}

impl<const DIMENSIONS: usize> WorldState<DIMENSIONS> {
	pub fn new(particle_count: usize, function: fn(VectorN<DIMENSIONS>) -> f64, bounds: (f64, f64), social_coefficient: f64, cognitive_coefficient: f64, inertia_coefficient: f64, 
			/*DE-specific */ de_population_size: usize, de_crossover_probability: f64, de_diff_weight: f64, de_lambda: f64, de_num_iters: usize) -> Self {
		if bounds.0 >= bounds.1 {
			panic!("Incorrect order of bounds or zero size");
		}
		let mut result = Self {
			particles: Vec::with_capacity(particle_count),
			function,
			best_solution: VectorN::<DIMENSIONS>::default(),
			best_solution_value: f64::INFINITY,
			bounds,
			particle_count,
			social_coefficient,
			cognitive_coefficient,
			inertia_coefficient,
			random_generator: fastrand::Rng::new(),

			/* DE-specific */
			de_population_size,
			de_crossover_probability,
			de_diff_weight,
			de_lambda,
			de_num_iters
		};

		result.create_particles();

		return result;
	}

	fn create_particles(&mut self) {
		let size = self.bounds.1 - self.bounds.0;
		let mut best_solution = f64::INFINITY;
		for _ in 0..self.particle_count {
			let mut coords = [0.0; DIMENSIONS];
			coords.fill_with(|| self.random_generator.f64() * size + self.bounds.0);
			let value_at_coords = (self.function)(VectorN::<DIMENSIONS>::new(coords));
			self.particles.push(Particle::<DIMENSIONS> {
				current_speed: VectorN::<DIMENSIONS>::default(),
				coordinates: VectorN::<DIMENSIONS>::new(coords),
				best_found_solution: VectorN::<DIMENSIONS>::new(coords),
				best_found_solution_value: value_at_coords,
				bounds: self.bounds,
				social_coefficient: self.social_coefficient,
				cognitive_coefficient: self.cognitive_coefficient,
				inertia_coefficient: self.inertia_coefficient,
			});
			if value_at_coords < best_solution {
				best_solution = value_at_coords;
				self.best_solution = VectorN::<DIMENSIONS>::new(coords);
			}
		}
	}

	pub fn reset(&mut self) {
		let size = self.bounds.1 - self.bounds.0;
		let mut best_solution = f64::INFINITY;
		for particle in self.particles.iter_mut() {
			let mut coords = [0.0; DIMENSIONS];
			coords.fill_with(|| self.random_generator.f64() * size + self.bounds.0);
			particle.current_speed = VectorN::<DIMENSIONS>::default();
			particle.coordinates = VectorN::<DIMENSIONS>::new(coords);
			particle.best_found_solution = VectorN::<DIMENSIONS>::new(coords);
			let particle_solution = (self.function)(VectorN::<DIMENSIONS>::new(coords));
			if particle_solution < best_solution {
				best_solution = particle_solution;
				self.best_solution = VectorN::<DIMENSIONS>::new(coords);
				self.best_solution_value = (self.function)(self.best_solution);
			}
		}
	}

	pub fn update_best_solutions(&mut self) {
		for particle in &mut self.particles {
			let particle_solution = (self.function)(particle.coordinates);
			if particle_solution < self.best_solution_value {
				self.best_solution_value = particle_solution;
				self.best_solution = particle.coordinates;
			}
			if particle_solution < particle.best_found_solution_value {
				particle.best_found_solution = particle.coordinates;
				particle.best_found_solution_value = particle_solution;
			}
		}
	}

	pub fn move_particles(&mut self) { // Use DE here

		let de_stop_condition = de::DeStopCondition {
			stype: de::DeStopType::StopAfterIters,
			union: de::DeLimitation { iters: self.de_num_iters as u64 }
		};

		let mut de_config = de::DeConfig {
			population_size: self.de_population_size as u32,
			crossover_probability: self.de_crossover_probability,
			amplification_factor: self.de_diff_weight,
			lambda: self.de_lambda,
			stop_condition: de_stop_condition
		};

		let mut de_target = de::DeOptimizationTarget {
			f: c_optimization_function_for_pso_control_params::<DIMENSIONS>,
			num_dimensions: 3,
			left_bound: 0.0,
			right_bound: 1.0,
		};

		let mut de_manipulated_coeffs = unsafe { de::de_minimum(&mut de_target, &mut de_config, &mut self.clone() as *mut WorldState<DIMENSIONS> as *mut c_void) };
		
		// Update the coefficients
		unsafe {
			self.social_coefficient = *de_manipulated_coeffs.coordinates.offset(0);
			self.cognitive_coefficient = *de_manipulated_coeffs.coordinates.offset(1);
			self.inertia_coefficient = *de_manipulated_coeffs.coordinates.offset(2);
		}

		// Free the memory allocated inside de_manipulated_coeffs
		unsafe {
			de::de_vector_free_coordinates(&mut de_manipulated_coeffs);
		}

		for particle in &mut self.particles {
			particle.social_coefficient = self.social_coefficient;
			particle.cognitive_coefficient = self.cognitive_coefficient;
			particle.inertia_coefficient = self.inertia_coefficient;
		}

		for particle in &mut self.particles {
			particle.move_particle(self.best_solution, &mut self.random_generator);
		}
	}

	pub fn do_iteration(&mut self) {
		self.move_particles();
		self.update_best_solutions();
	}

	pub fn do_all_iterations(&mut self, iteration_count: usize) {
		for _ in 0..iteration_count {
			self.do_iteration();
		}
	}

	pub fn do_all_iters_with_record(&mut self, iteration_count: usize) -> Vec<Vec<VectorN<DIMENSIONS>>> {
		let mut snapshots = Vec::with_capacity(iteration_count); // each entry is state of particles after one iteration
		for _ in 0..iteration_count {
			self.do_iteration();
			snapshots.push(self.particles.iter().map(|particle| particle.coordinates ).collect::<Vec<_>>());
		}
		return snapshots;

	}
}