#include "de.h"
#include <string.h>
#include <stdio.h> // for logging

typedef struct {
    vectorBuf_t members; /** @optimize memory */
    uint64_t size;
} de_population_t;

typedef de_population_t* pDePopulation_t;

void de_population_allocate_members (pDePopulation_t pPop) { pPop->members = malloc(pPop->size * sizeof(vector_t)); }
void de_population_free_members (pDePopulation_t pPop) { free(pPop->members); }

typedef struct {
    de_population_t main_population;
    de_population_t probe_population;
} de_workspace_t;

typedef de_workspace_t* pDeWorkspace_t;

void de_workspace_init (pDeWorkspace_t pWorkspace, uint32_t populationSize) {
    pWorkspace->main_population.size = populationSize;
    pWorkspace->probe_population.size = populationSize;
    de_population_allocate_members (&pWorkspace->main_population);
    de_population_allocate_members (&pWorkspace->probe_population);
}

void de_workspae_deinit (pDeWorkspace_t pWorkspace) {
    de_population_free_members (&pWorkspace->main_population);
    de_population_free_members (&pWorkspace->probe_population);
}

vector_t de_minimum(de_optimization_target_t* pOptimizationTarget, de_config_t* pConfig) {
    
    vector_t v = { .num_dimensions = pOptimizationTarget->num_dimensions };
    vector_allocate_coordinates (&v);
    //memset (v.coordinates, 1, v.num_dimensions * sizeof(double));
    for (uint32_t i = 0; i < v.num_dimensions; i++) {
        v.coordinates[i] = v.num_dimensions - i - 0.35;
    }
    return v;
}