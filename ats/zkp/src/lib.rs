use ark_bn254::{Bn254, Fr};

pub mod circuit;
pub mod hashing;
pub mod utils;
pub mod zkp;

// Exposed types
pub type Curve = Bn254;
pub type F = Fr;

pub use circuit::*;
pub use hashing::*;
pub use utils::*;
pub use zkp::*;
