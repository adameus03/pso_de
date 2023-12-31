pub mod vector;

use vector::Vector2D;
use rand::Rng;



#[derive(Debug, Clone)]
pub struct Particle {
	pub current_speed: Vector2D,
	pub coordinates: Vector2D,
	pub best_found_solution: Vector2D, // of this particle
	pub x_bounds: (f64, f64), // lower, upper
	pub y_bounds: (f64, f64), // lower, upper,
	pub social_coefficient: f64,
	pub cognitive_coefficient: f64,
	pub inertia_coefficient: f64,
}

impl Particle {
	fn move_particle(&mut self, best_global_solution: Vector2D) {
		let inertia_part = self.current_speed * self.inertia_coefficient;
		let social_part = (best_global_solution - self.coordinates) * self.social_coefficient * rand::random::<f64>();
		let self_part = (self.best_found_solution - self.coordinates) * self.cognitive_coefficient * rand::random::<f64>();
		self.current_speed = inertia_part + social_part + self_part;
		self.coordinates += self.current_speed * 1.0;

		self.coordinates.clamp(self.x_bounds, self.y_bounds);
	}
}



#[derive(Debug, Clone)]
pub struct WorldState {
	pub particles: Vec<Particle>,
	pub function: fn(Vector2D) -> f64,
	pub best_solution: Vector2D,
	x_bounds: (f64, f64),
	y_bounds: (f64, f64),
	particle_count: usize,
	social_coefficient: f64,
	cognitive_coefficient: f64,
	inertia_coefficient: f64,
}

impl WorldState {
	pub fn new(particle_count: usize, function: fn(Vector2D) -> f64, x_bounds: (f64, f64), y_bounds: (f64, f64), social_coefficient: f64, cognitive_coefficient: f64, inertia_coefficient: f64) -> Self {
		if x_bounds.0 >= x_bounds.1 || y_bounds.0 >= y_bounds.1 {
			panic!("Incorrect order of bounds or zero size");
		}
		let mut result = WorldState {
			particles: Vec::with_capacity(particle_count),
			function,
			best_solution: Vector2D::new(0.0, 0.0),
			x_bounds,
			y_bounds,
			particle_count,
			social_coefficient,
			cognitive_coefficient,
			inertia_coefficient,
		};

		result.create_particles();

		return result;
	}

	fn create_particles(&mut self) {
		let mut generator = rand::thread_rng();
		let x_size = self.x_bounds.1 - self.x_bounds.0;
		let y_size = self.y_bounds.1 - self.y_bounds.0;
		let mut best_solution = f64::INFINITY;
		for _ in 0..self.particle_count {
			let x_coord = generator.gen::<f64>() * x_size + self.x_bounds.0;
			let y_coord = generator.gen::<f64>() * y_size + self.y_bounds.0;
			self.particles.push(Particle {
				current_speed: Vector2D::new(0.0, 0.0),
				coordinates: Vector2D::new(x_coord, y_coord),
				best_found_solution: Vector2D::new(x_coord, y_coord),
				x_bounds: self.x_bounds,
				y_bounds: self.y_bounds,
				social_coefficient: self.social_coefficient,
				cognitive_coefficient: self.cognitive_coefficient,
				inertia_coefficient: self.inertia_coefficient,
			});
			let particle_solution = (self.function)(Vector2D::new(x_coord, y_coord));
			if particle_solution < best_solution {
				best_solution = particle_solution;
				self.best_solution = Vector2D::new(x_coord, y_coord);
			}
		}
	}

	pub fn reset(&mut self) {
		let mut generator = rand::thread_rng();
		let x_size = self.x_bounds.1 - self.x_bounds.0;
		let y_size = self.y_bounds.1 - self.y_bounds.0;
		let mut best_solution = f64::INFINITY;
		for particle in self.particles.iter_mut() {
			let x_coord = generator.gen::<f64>() * x_size + self.x_bounds.0;
			let y_coord = generator.gen::<f64>() * y_size + self.y_bounds.0;
			particle.current_speed = Vector2D::new(0.0, 0.0);
			particle.coordinates = Vector2D::new(x_coord, y_coord);
			particle.best_found_solution = Vector2D::new(x_coord, y_coord);
			let particle_solution = (self.function)(Vector2D::new(x_coord, y_coord));
			if particle_solution < best_solution {
				best_solution = particle_solution;
				self.best_solution = Vector2D::new(x_coord, y_coord);
			}
		}
	}

	pub fn update_best_solutions(&mut self) {
		let mut best_global_solution = (self.function)(self.best_solution);
		for particle in &mut self.particles {
			let particle_solution = (self.function)(particle.coordinates);
			if particle_solution < best_global_solution {
				best_global_solution = particle_solution;
				self.best_solution = particle.coordinates;
			}
			if particle_solution < (self.function)(particle.best_found_solution) {
				particle.best_found_solution = particle.coordinates;
			}
		}
	}

	pub fn move_particles(&mut self) {
		for particle in &mut self.particles {
			particle.move_particle(self.best_solution);
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

	pub fn do_all_iters_with_record(&mut self, iteration_count: usize) -> Vec<Vec<Vector2D>> {
		let mut snapshots = Vec::with_capacity(iteration_count); // each entry is state of particles after one iteration
		for _ in 0..iteration_count {
			self.do_iteration();
			snapshots.push(self.particles.iter().map(|particle| particle.coordinates ).collect::<Vec<_>>());
		}
		return snapshots;

	}
}