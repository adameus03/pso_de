#include <stdint.h>
#include <stdlib.h>

typedef struct {
    double* coordinates;
    uint32_t num_dimensions;   
} vector_t;

typedef vector_t* pVector_t;
typedef vector_t* vectorBuf_t;
typedef vector_t** pVectorBuf_t;

void vector_allocate_coordinates (pVector_t pVec) { pVec->coordinates = malloc(pVec->num_dimensions * sizeof(double)); }
void vector_free_coordinates (pVector_t pVec) { free(pVec->coordinates); }

typedef double (*RdR_Function)(vector_t);

typedef struct {
    RdR_Function f;
    uint32_t num_dimensions;
    double left_bound;
    double right_bound;
} de_optimization_target_t;

typedef struct {
    uint32_t population_size;
} de_config_t;

/**
 * @attention The coordinates buffer of the returned vector need to be freed, please use `vector_free_coordinates(pVector_t)`
 * @attention This function is a stub
*/
vector_t de_minimum(de_optimization_target_t* pOptimizationTarget, de_config_t* pConfig);