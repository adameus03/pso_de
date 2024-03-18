#include "de.h"
#include <stdio.h> // for printing to stderr
#include <time.h> // for srand

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
    pVector_t p_main_best; double p_main_best_val;
} de_workspace_t;

typedef de_workspace_t* pDeWorkspace_t;

void de_workspace_init (pDeWorkspace_t pWorkspace, uint32_t populationSize, uint32_t numDimensions) {
    pWorkspace->main_population.size = populationSize;
    pWorkspace->probe_population.size = populationSize;
    de_population_allocate_members (&pWorkspace->main_population);
    de_population_allocate_members (&pWorkspace->probe_population);

    for (uint32_t i = 0; i < pWorkspace->main_population.size; i++) {
        pVector_t pVec = &pWorkspace->main_population.members[i];
        pVec->num_dimensions = numDimensions;
        de_vector_allocate_coordinates (pVec);
    }

    for (uint32_t i = 0; i < pWorkspace->probe_population.size; i++) {
        pVector_t pVec = &pWorkspace->probe_population.members[i];
        pVec->num_dimensions = numDimensions;
        de_vector_allocate_coordinates (pVec);
    }
}

void de_workspace_deinit (pDeWorkspace_t pWorkspace) {
    for (uint32_t i = 0; i < pWorkspace->main_population.size; i++) {
        pVector_t pVec = &pWorkspace->main_population.members[i];
        de_vector_free_coordinates(pVec);
    }

    for (uint32_t i = 0; i < pWorkspace->probe_population.size; i++) {
        pVector_t pVec = &pWorkspace->probe_population.members[i];
        de_vector_free_coordinates(pVec);
    }

    de_population_free_members (&pWorkspace->main_population);
    de_population_free_members (&pWorkspace->probe_population);
}

void de_workspace_set_best(pDeWorkspace_t pWorkspace, pVector_t pBest, pDeOptimizationTarget_t pTarget, void* pUserData) {
    pWorkspace->p_main_best = pBest;
    pWorkspace->p_main_best_val = pTarget->f(*pBest, pUserData);
}

void de_generate_main_population(pDeWorkspace_t pWorkspace, uint32_t numDimensions, double leftBound, double rightBound) {
    for (uint32_t i = 0; i < pWorkspace->main_population.size; i++) {
        pVector_t pVec = &pWorkspace->main_population.members[i];
        for (uint32_t j = 0; j < numDimensions; j++) {
            pVec->coordinates[j] = leftBound + (rightBound - leftBound) * (double)rand() / RAND_MAX;
        }
    }
}

pVector_t de_get_best(pDePopulation_t pPopulation, RdR_Function f, void* pUserData) {
    pVector_t pBest = &pPopulation->members[0];
    double bestVal = f(*pBest, pUserData);
    for (uint32_t i = 0; i < pPopulation->size; i++) {
        pVector_t pVec = &pPopulation->members[i];
        double val = f(*pVec, pUserData);
        if (val < bestVal) {
            pBest = pVec;
            bestVal = val;
        }
    }
    return pBest;
}

/**
 * Reproduce the population, so that it can be mutated with DE/rand_best/1/bin
*/
void de_reproduce(pDeWorkspace_t pWorkspace, pDeOptimizationTarget_t pTarget, pDeConfig_t pConfig, void* pUserData) {
    pVector_t pBest = de_get_best(&pWorkspace->main_population, pTarget->f, pUserData);
    de_workspace_set_best(pWorkspace, pBest, pTarget, pUserData); // Let the workspace know who is the best
    double lambda = pConfig->lambda;

    for (uint32_t i = 0; i < pWorkspace->main_population.size; i++) {
        pVector_t pMainVec = &pWorkspace->main_population.members[i];
        pVector_t pProbeVec = &pWorkspace->probe_population.members[i];
        //memcpy (pProbeVec->coordinates, pMainVec->coordinates, pMainVec->num_dimensions * sizeof(double));
        for (uint32_t j = 0; j < pProbeVec->num_dimensions; j++) {
            pProbeVec->coordinates[j] = lambda * pBest->coordinates[j] + (1 - lambda) * pMainVec->coordinates[j];
        }
    }
}

void de_mutate(pDeWorkspace_t pWorkspace, pDeOptimizationTarget_t pTarget, pDeConfig_t pConfig) {
    double F = pConfig->amplification_factor;
    double leftBound = pTarget->left_bound;
    double rightBound = pTarget->right_bound;

    for (uint32_t i = 0; i < pWorkspace->probe_population.size; i++) {
        pVector_t pProbeVec = &pWorkspace->probe_population.members[i];
        uint32_t r2;
        uint32_t r3;
        do {
            r2 = rand() % pWorkspace->main_population.size;
        } while (r2 == i);
        do {
            r3 = rand() % pWorkspace->main_population.size;
        } while (r3 == i || r3 == r2);
        pVector_t pX2 = &pWorkspace->main_population.members[r2];
        pVector_t pX3 = &pWorkspace->main_population.members[r3];
        for (uint32_t j = 0; j < pProbeVec->num_dimensions; j++) {
            pProbeVec->coordinates[j] += F * (pX2->coordinates[j] - pX3->coordinates[j]);
            if (pProbeVec->coordinates[j] < leftBound) {
                pProbeVec->coordinates[j] = leftBound;
            } else if (pProbeVec->coordinates[j] > rightBound) {
                pProbeVec->coordinates[j] = rightBound;
            }
            if (pProbeVec->coordinates[j] < leftBound || pProbeVec->coordinates[j] > rightBound) {
                fprintf(stderr, "[DE] Error: Coordinate out of bounds in mutation\n");
            }
        }
    }
}

void de_crossover(pDeWorkspace_t pWorkspace, pDeConfig_t pConfig) {
    double CR = pConfig->crossover_probability;

    for (uint32_t i = 0; i < pWorkspace->probe_population.size; i++) {
        pVector_t pProbeVec = &pWorkspace->probe_population.members[i];
        pVector_t pMainVec = &pWorkspace->main_population.members[i];

        uint32_t d = rand() % pProbeVec->num_dimensions;
        for (uint32_t j = 0; j < pProbeVec->num_dimensions; j++) {
            double r = rand() % pProbeVec->num_dimensions;
            if (r < CR || j == d) {
                // do nothing
            } else {
                pProbeVec->coordinates[j] = pMainVec->coordinates[j];
            }
        }
    }
}

void de_select(pDeWorkspace_t pWorkspace, pDeOptimizationTarget_t pTarget, void* pUserData) {
    for (uint32_t i = 0; i < pWorkspace->main_population.size; i++) {
        pVector_t pMainVec = &pWorkspace->main_population.members[i];
        pVector_t pProbeVec = &pWorkspace->probe_population.members[i];
        double mainVal = pTarget->f(*pMainVec, pUserData);
        double probeVal = pTarget->f(*pProbeVec, pUserData);
        if (probeVal < mainVal) {
            for (uint32_t j = 0; j < pMainVec->num_dimensions; j++) {
                pMainVec->coordinates[j] = pProbeVec->coordinates[j];
            }
        }
    }
}

/**
 * @returns 1 if stop condition is satisfied, otherwise 0
*/
uint8_t de_check_stop_condition(uint64_t iter_count, pDeConfig_t pConfig, pDeWorkspace_t pWorkspace) {
    switch (pConfig->stop_condition.type) {
        case STOP_AFTER_ITERS:
            return iter_count > pConfig->stop_condition.limitation.iters;
        case STOP_WHEN_SATISFIED:
            return pWorkspace->p_main_best_val < pConfig->stop_condition.limitation.accuracy;
        default:
            fprintf(stderr, "[DE] Error: Unknown stop condition provided\n");
            return 1U;
    }
}

vector_t de_minimum(de_optimization_target_t* pOptimizationTarget, de_config_t* pConfig, void* pUserData) {
    srand(time(NULL)); // Initialize rng
    printf("I'm here\n");//debug
    de_workspace_t workspace;
    de_workspace_init (&workspace, pConfig->population_size, pOptimizationTarget->num_dimensions);
    de_generate_main_population (&workspace, pOptimizationTarget->num_dimensions, pOptimizationTarget->left_bound, pOptimizationTarget->right_bound);
    
    uint64_t iter_count = 0U;
    do {
        de_reproduce (&workspace, pOptimizationTarget, pConfig, pUserData);
        de_mutate (&workspace, pOptimizationTarget, pConfig);
        de_crossover (&workspace, pConfig);
        de_select (&workspace, pOptimizationTarget, pUserData);
        iter_count++;
    } while (!de_check_stop_condition(iter_count, pConfig, &workspace));
    
    pVector_t pBest = de_get_best (&workspace.main_population, pOptimizationTarget->f, pUserData);

    vector_t retVec = { .num_dimensions = pBest->num_dimensions };
    de_vector_allocate_coordinates (&retVec);
    for (uint32_t i = 0; i < retVec.num_dimensions; i++) {
        retVec.coordinates[i] = pBest->coordinates[i];
    }

    de_workspace_deinit (&workspace);

    return retVec;
    
}

/*vector_t de_minimum_stub(de_optimization_target_t* pOptimizationTarget, de_config_t* pConfig) {
    vector_t v = { .num_dimensions = pOptimizationTarget->num_dimensions };
    de_vector_allocate_coordinates (&v);
    //memset (v.coordinates, 1, v.num_dimensions * sizeof(double));
    for (uint32_t i = 0; i < v.num_dimensions; i++) {
        //v.coordinates[i] = v.num_dimensions - i - 0.35;
        v.coordinates[i] = 1;
    }
    double val = pOptimizationTarget->f(v);
    for (uint32_t i = 0; i < v.num_dimensions; i++) {
        v.coordinates[i] = val;
    }
    return v;
}*/