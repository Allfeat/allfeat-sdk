//! Error types for the ZKP crate.
//!
//! This module defines a comprehensive error type [`ZkpError`] that captures
//! all failure modes in proof generation, verification, and input processing.

/// Comprehensive error type for ZKP operations.
///
/// Distinguishes between different failure modes to enable proper error handling
/// and debugging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkpError {
    /// Invalid hex string provided (malformed or wrong length).
    ///
    /// Contains the field name and description of the issue.
    InvalidHex,

    /// Wrong number of public inputs provided.
    ///
    /// Expected 6 inputs in order: hash_title, hash_audio, hash_creators,
    /// commitment, timestamp, nullifier.
    WrongPublicInputCount,

    /// Failed to generate proof.
    ///
    /// This typically indicates a constraint system error or RNG failure.
    ProofGenerationFailed,

    /// Failed to verify proof.
    ///
    /// This is different from verification returning `false` - it indicates
    /// an error in the verification process itself.
    VerificationError,

    /// Failed to serialize or deserialize cryptographic objects (keys, proofs).
    SerializationFailed,

    /// Failed to deserialize cryptographic objects (keys, proofs).
    DeserializationFailed,

    /// Input data is too large (e.g., hex string exceeds field size).
    InputTooLarge,
}

impl core::fmt::Display for ZkpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ZkpError::InvalidHex => {
                write!(f, "Invalid hex string")
            }
            ZkpError::WrongPublicInputCount => {
                write!(f, "Wrong number of public inputs")
            }
            ZkpError::ProofGenerationFailed => {
                write!(f, "Proof generation failed")
            }
            ZkpError::VerificationError => {
                write!(f, "Verification error")
            }
            ZkpError::SerializationFailed => {
                write!(f, "Serialization failed")
            }
            ZkpError::DeserializationFailed => {
                write!(f, "Deserialization failed")
            }
            ZkpError::InputTooLarge => {
                write!(f, "Input too large")
            }
        }
    }
}

/// Specialized `Result` type for ZKP operations.
pub type Result<T> = core::result::Result<T, ZkpError>;
