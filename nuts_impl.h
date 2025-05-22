#ifndef NUTS_IMPL_H
#define NUTS_IMPL_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

double* generate_samples(size_t num_samples, const double* initial_positions, size_t dim);
void free_samples(double* ptr, size_t len);

#ifdef __cplusplus
}
#endif

#endif