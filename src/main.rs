mod functions;

use particle_swarm::WorldState;



fn main() {
	let mut a = WorldState::new(10, functions::ackley, (-5.0, 5.0), (-5.0, 5.0), 0.4, 0.7, 0.3);
	a.do_all_iterations(1000);
	println!("{} at {:#?}", functions::ackley(a.best_solution), a.best_solution)
}
