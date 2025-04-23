// use std::collections::HashMap;
use nuts_rs::{CpuLogpFunc, CpuMath, LogpError, DiagGradNutsSettings, Chain, Settings};
use thiserror::Error;
use rand::thread_rng;

extern "C" {
    // This definition must match the C function signature.
    fn logpc(position: *const f64, grad: *mut f64) -> f64;
}


#[derive(Debug)]
struct PosteriorDensity {}

#[derive(Debug, Error)]
enum PosteriorLogpError {}
impl LogpError for PosteriorLogpError {
    fn is_recoverable(&self) -> bool { false }
}

impl CpuLogpFunc for PosteriorDensity {
    type LogpError = PosteriorLogpError;
    type TransformParams = ();

    fn dim(&self) -> usize { 1 }

    fn logp(&mut self, position: &[f64], grad: &mut [f64]) -> Result<f64, Self::LogpError> {
        // Note: you'll need to handle the array length and possible errors.
        // unsafe { println!("{:?}", logpc(position.as_ptr(), grad.as_mut_ptr()));}
        unsafe { Ok(logpc(position.as_ptr(), grad.as_mut_ptr())) }
    }
}

fn main() {

    // unsafe{ println!("{:?}", logpc());}

    let mut settings = DiagGradNutsSettings::default();

    settings.num_tune = 500; // small value just for testing...
    settings.maxdepth = 10;  // small value just for testing...

    // We instanciate our posterior density function
    let logp_func = PosteriorDensity {};
    // creating function, that will
    // calculated in parallel procces
    let math = CpuMath::new(logp_func); 


    let mut rng = thread_rng();
    let mut sampler = settings.new_chain(0, math, &mut rng);

    // Set to some initial position and start drawing samples.
    sampler.set_position(&vec![0.1f64]).expect("Unrecoverable error during init"); 
    let mut trace = vec![];  // Collection of all draws
    for _ in 0..1000 {
        let draw = sampler.draw().expect("Unrecoverable error during sampling");
        trace.push(draw);
    }
    println!("{:?}", trace);
}