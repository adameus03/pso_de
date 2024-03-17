use crate::vector::VectorN;

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
}

impl<const DIMENSIONS: usize> WorldState<DIMENSIONS> {
	pub fn new(particle_count: usize, function: fn(VectorN<DIMENSIONS>) -> f64, bounds: (f64, f64), social_coefficient: f64, cognitive_coefficient: f64, inertia_coefficient: f64) -> Self {
		if bounds.0 >= bounds.1 {
			panic!("Incorrect order of bounds or zero size");
		}
		let mut result = Self {
			random_generator: fastrand::Rng::new(),
			particles: Vec::with_capacity(particle_count),
			function,
			best_solution: VectorN::<DIMENSIONS>::default(),
			best_solution_value: f64::INFINITY,
			bounds,
			particle_count,
			social_coefficient,
			cognitive_coefficient,
			inertia_coefficient,
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

	pub fn move_particles(&mut self) {
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