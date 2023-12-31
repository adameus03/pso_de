#!/bin/bash
particle_counts=(10 30 50 100 200) #5
iteration_counts=(10 30 50 100 200) #5
social_coefficients=(0 0.25 0.5 0.75 1) #5
cognitive_coefficients=(0 0.25 0.5 0.75 1) #5
inertia_coefficients=(0 0.25 0.5 0.75 1) #5
functions="ackley,beale,goldstein-price,booth,bukin,matyas,levi,himmelblau,three-humps,easom,cross-in-tray,eggholder,holder,mccormick,schaffer2,schaffer4"
runs_per_set=128

rm -rf output
rm -rf output_aggregate
mkdir -p output_aggregate

cargo build --release
for particles in "${particle_counts[@]}"
do
	for iterations in "${iteration_counts[@]}"
	do
		for social_coefficient in "${social_coefficients[@]}"
		do
			for cog_coeff in "${cognitive_coefficients[@]}"
			do
				for inertia_coeff in "${inertia_coefficients[@]}"
				do
					mkdir -p output
					./target/release/particle_swarm --functions $functions --particles $particles --iterations $iterations --social-coeff $social_coefficient --cognitive-coeff $cog_coeff --inertia-coeff $inertia_coeff --try-count $runs_per_set > output/statistics.txt
					echo "particles,iterations,social_coeff,cog_coeff,inertia_coeff" > output/run_data.csv
					echo "$particles,$iterations,$social_coefficient,$cog_coeff,$inertia_coeff" >> output/run_data.csv
					mv output output_aggregate/"$particles"_particles_"$iterations"_iters_"$social_coefficient"_soccoeff_"$cog_coeff"_cogcoeff_"$inertia_coeff"_inercoeff
					echo Finished "$particles"_particles_"$iterations"_iters_"$social_coefficient"_soccoeff_"$cog_coeff"_cogcoeff_"$inertia_coeff"_inercoeff
				done
			done
		done
	done
done