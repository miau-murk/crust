use nuts_rs::{CpuLogpFunc, CpuMath, LogpError, DiagGradNutsSettings, Chain, Settings};
use thiserror::Error;
use rand::thread_rng;

extern "C" {
    fn logpc(position: *const f64, grad: *mut f64, dim: usize) -> f64;
}

#[derive(Debug)]
struct PosteriorDensity {
    dim: usize,
}

#[derive(Debug, Error)]
enum PosteriorLogpError {}
impl LogpError for PosteriorLogpError {
    fn is_recoverable(&self) -> bool { false }
}

impl CpuLogpFunc for PosteriorDensity {
    type LogpError = PosteriorLogpError;
    type TransformParams = ();

    fn dim(&self) -> usize { self.dim }

    fn logp(&mut self, position: &[f64], grad: &mut [f64]) -> Result<f64, Self::LogpError> {
        unsafe { Ok(logpc(position.as_ptr(), grad.as_mut_ptr(), self.dim)) }
    }
}

#[no_mangle]
pub extern "C" fn generate_samples(
    num_samples: usize,
    initial_positions: *const f64,
    dim: usize
) -> *mut f64 {
    let mut settings = DiagGradNutsSettings::default();
    settings.num_tune = 500;
    settings.maxdepth = 10;

    let logp_func = PosteriorDensity { dim };
    let math = CpuMath::new(logp_func);
    let mut rng = thread_rng();
    let mut sampler = settings.new_chain(0, math, &mut rng);

    let initial_positions_slice = unsafe { std::slice::from_raw_parts(initial_positions, dim) };
    sampler.set_position(initial_positions_slice).expect("Unrecoverable error during init");
    
    let mut trace = Vec::with_capacity(num_samples * dim);
    for _ in 0..num_samples {
        let (draw, _progress) = sampler.draw().expect("Unrecoverable error during sampling");
        trace.extend_from_slice(&draw);
    }

    let boxed_slice = trace.into_boxed_slice();
    Box::into_raw(boxed_slice) as *mut f64
}

#[no_mangle]
pub extern "C" fn free_samples(ptr: *mut f64, len: usize) {
    if !ptr.is_null() {
        unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, len)); }
    }
}