#include <stdint.h>
#include <stdlib.h>

typedef struct {
    double* coordinates;
    uint32_t num_dimensions;   
} vector_t;

typedef vector_t* pVector_t;
typedef vector_t* vectorBuf_t;
typedef vector_t** pVectorBuf_t;

void de_vector_allocate_coordinates (pVector_t pVec) { pVec->coordinates = malloc(pVec->num_dimensions * sizeof(double)); }
void de_vector_free_coordinates (pVector_t pVec) { free(pVec->coordinates); }

typedef double (*RdR_Function)(vector_t, void*);

typedef struct {
    RdR_Function f;
    uint32_t num_dimensions;
    double left_bound;
    double right_bound;
} de_optimization_target_t;

typedef de_optimization_target_t* pDeOptimizationTarget_t;

typedef enum {
    STOP_AFTER_ITERS,
    STOP_WHEN_SATISFIED
} de_stop_type_t;

typedef struct {
    de_stop_type_t type;
    union {
        uint64_t iters;
        double accuracy; 
    } limitation;
} de_stop_condition_t;

typedef struct {
    uint32_t population_size;
    double crossover_probability; // CR (pc) [0; 1]
    double amplification_factor; // F [0; 2]
    double lambda; // DE/rand_best/1/bin specific x_best weight [0; 1]
    de_stop_condition_t stop_condition;
} de_config_t;

typedef de_config_t* pDeConfig_t;

/**
 * Differential evolution with DE/rand_best/1/bin mutation
 * @attention The coordinates buffer of the returned vector need to be freed, please use `vector_free_coordinates(pVector_t)`
*/
vector_t de_minimum(de_optimization_target_t* pOptimizationTarget, de_config_t* pConfig, void* pUserData);

/**
 * If you need a stub, you can use this one
*/
//vector_t de_minimum_stub(de_optimization_target_t* pOptimizationTarget, de_config_t* pConfig);