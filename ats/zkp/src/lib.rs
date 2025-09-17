use ark_bn254::{Bn254, Fr};

pub mod api;
pub mod circuit;
pub mod utils;

// Exposed types
pub type Curve = Bn254;
pub type F = Fr;
