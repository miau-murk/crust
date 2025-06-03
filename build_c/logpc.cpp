#include <iostream>
#include <cmath>
#include <bits/stdc++.h>
#include "some_func.h"

// Multivariate normal distribution log probability and gradient
extern "C" {
    double logpc(const double *position, double *grad, size_t dim) {
        if (!position || !grad) return NAN;
        
        double logp = 0.0;
        for (size_t i = 0; i < dim; ++i) {
            const double mu = 0.0;       // Mean for each dimension
            const double sigma = 1.0;    // Std dev for each dimension
            const double diff = position[i] - mu;
            logp += -0.5 * diff * somefunc(diff, sigma);
            grad[i] = -somefunc(diff, sigma);
        }
        
        return logp;
    }
}
