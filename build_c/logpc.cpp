#include <iostream>
#include <cmath>

// This function calculates the log probability of a standard normal distribution
// and its gradient. It accepts a pointer to the position array (which for simplicity,
// we assume is 1-dimensional) and a pointer to the gradient array.
extern "C" {
    double logpc(const double *position, double *grad) {
        const double mu = 0.0;
        const double sigma = 1.0;
        if (!position || !grad) return NAN; 
        const double diff = position[0] - mu;
        double logp = -0.5 * diff * diff / (sigma * sigma);
        grad[0] = -diff / (sigma * sigma);
        return logp;
    }
}

