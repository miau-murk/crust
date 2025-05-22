#include <stdio.h>
#include "nuts_impl.h"

int main() {
    size_t num_samples = 10;
    size_t dim = 2;
    double initial_positions[] = {1.0, -1.0};
    
    double* samples = generate_samples(num_samples, initial_positions, dim);
    
    printf("Generated %zu samples (each with %zu dimensions):\n", num_samples, dim);
    for (size_t i = 0; i < num_samples; ++i) {
        printf("Sample %zu: ", i+1);
        for (size_t j = 0; j < dim; ++j) {
            printf("%f ", samples[i*dim + j]);
        }
        printf("\n");
    }
    
    free_samples(samples, num_samples * dim);
    return 0;
}