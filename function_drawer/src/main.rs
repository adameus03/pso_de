use linspace::Linspace;
use particle_swarm::{functions, vector::Vector2D};

fn main() {
    for (name, function) in functions::create_function_list() {
		if name != "eggholder" {
			continue;
		}
		let (x_bounds, y_bounds) = function.get_bounds();
		let x_vals = (x_bounds.0..x_bounds.1).linspace(125);
		let y_vals = (y_bounds.0..y_bounds.1).linspace(125);
		let mut output = Vec::with_capacity(1000 * 1000);
		for x in x_vals {
			for y in &y_vals {
				output.push(format!("{};{};{}", x, y, function.get_function()(Vector2D { x, y: *y })));
			}
		}
		std::fs::write(format!("./{}.csv", name), format!("x;y;z\n{}", output.join("\n"))).unwrap();
	}
}
