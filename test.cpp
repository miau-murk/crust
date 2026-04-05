#include <stdio.h>
#include "nuts_impl.h"

int main() {
    size_t num_samples = 100; // number of samples
    size_t dim = 3; // dimension of the parameter space
    double initial_positions[] = {1.0, -1.0, 0.5}; // initial point in space
    
    // generate samples
    double* samples = generate_samples(num_samples, initial_positions, dim);

    free_samples(samples, num_samples * dim);

    return 0;
}