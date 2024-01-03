fn main() {
	let header = "particles,iterations,social_coeff,cog_coeff,inertia_coeff,fn_name,max_solution,avg_solution,min_solution";
	let gex = regex::Regex::new(r"(.*): .*is (-?\d*(?:\.\d*)?)\..*is (-?\d*(?:\.\d*)?)\. .*is (-?\d*(?:\.\d*)?)").unwrap();

	println!("{}", header);

	for filename in glob::glob("./output_aggregate/*/run_data.csv").unwrap() {
		let filename = filename.unwrap();
		let run_data = std::fs::read_to_string(&filename).unwrap();
		let run_data = run_data.lines().nth(1).unwrap();
		let stat_file = filename.to_str().unwrap().replace("run_data.csv", "statistics.txt");
		let stat_data = std::fs::read_to_string(&stat_file).unwrap().lines().map(|line| {
			let captures = gex.captures(line).unwrap();
			//                                         fn_name       max_sol       avg_sol       min_sol
			return format!("{},{},{},{},{}", run_data, &captures[1], &captures[2], &captures[3], &captures[4]);
		}).collect::<Vec<_>>().join("\n");
		println!("{}", stat_data);
	}
}
