use ark_bn254::Fr;
use ark_ff::PrimeField;
use sha2::{Digest, Sha256};

/// Convert a SHA-256 digest (32 bytes) into Fr (BN254) using big-endian mod-order.
fn fr_from_sha256(digest32: [u8; 32]) -> Fr {
    Fr::from_be_bytes_mod_order(&digest32)
}

/// Hash a work title (UTF-8) to Fr using SHA-256.
pub fn hash_title(title: &str) -> Fr {
    let mut hasher = Sha256::new();
    hasher.update(title.as_bytes());
    let digest = hasher.finalize();
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&digest);
    fr_from_sha256(arr)
}

/// Creator role flags. Only the selected roles will be included (in fixed AT/CP/AR/AD order).
#[derive(Debug, Clone, Copy, Default)]
pub struct Roles {
    pub author: bool,   // AT
    pub composer: bool, // CP
    pub arranger: bool, // AR
    pub adapter: bool,  // AD
}

impl Roles {
    /// Render as "AT/CP/AR/AD" but only including the roles that are true, in that exact order.
    fn to_abbrev(&self) -> String {
        let mut parts = Vec::with_capacity(4);
        if self.author {
            parts.push("AT");
        }
        if self.composer {
            parts.push("CP");
        }
        if self.arranger {
            parts.push("AR");
        }
        if self.adapter {
            parts.push("AD");
        }
        parts.join("/")
    }
}

/// Single creator input for hashing.
#[derive(Debug, Clone)]
pub struct Creator<'a> {
    pub full_name: &'a str,    // required
    pub email: &'a str,        // required
    pub roles: Roles,          // at least one should be true (validated in higher layer)
    pub ipi: Option<&'a str>,  // optional (up to 11 digits per spec)
    pub isni: Option<&'a str>, // optional (16 digits, or 15+X)
}

/// Hash the creators list to Fr using SHA-256 over the concatenation of:
/// <FullName><Email><ROLES><IPI><ISNI> for each creator, in order, no extra separators,
/// where ROLES are abbreviated and ordered as "AT/CP/AR/AD".
///
/// Spec references:
/// - hash_commitment = Poseidon(hash_file, hash_title, hash_creators, secret)
/// - hash_creators concatenates creators with fields in order:
///   Full name, Email, Roles (AT/CP/AR/AD), IPI, ISNI. :contentReference[oaicite:1]{index=1}
pub fn hash_creators(creators: &[Creator<'_>]) -> Fr {
    // Build the concatenated UTF-8 buffer exactly as specified (no extra separators)
    let mut buf = String::new();
    for c in creators {
        // Trim obvious whitespace; emails normalized to lowercase for stability.
        let name = c.full_name.trim();
        let email = c.email.trim().to_ascii_lowercase();
        let roles = c.roles.to_abbrev(); // already ordered AT/CP/AR/AD

        buf.push_str(name);
        buf.push_str(&email);
        buf.push_str(&roles);

        if let Some(ipi) = c.ipi {
            buf.push_str(ipi.trim());
        }
        if let Some(isni) = c.isni {
            buf.push_str(isni.trim());
        }
    }

    let mut hasher = Sha256::new();
    hasher.update(buf.as_bytes());
    let digest = hasher.finalize();
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&digest);
    fr_from_sha256(arr)
}

#[cfg(feature = "std")]
mod file_hash_std {
    use super::fr_from_sha256;
    use ark_bn254::Fr;
    use sha2::{Digest, Sha256};
    use std::{
        fs::File,
        io::{BufReader, Read},
        path::Path,
    };

    /// Hash an audio file to Fr using SHA-256 (streaming).
    /// Reads in 64 KiB chunks; does not load the entire file into memory.
    pub fn hash_audio<P: AsRef<Path>>(path: P) -> std::io::Result<Fr> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut hasher = Sha256::new();

        let mut buf = [0u8; 64 * 1024];
        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
        }

        let digest = hasher.finalize();
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&digest);
        Ok(fr_from_sha256(arr))
    }

    pub use hash_audio;
}

#[cfg(feature = "std")]
pub use file_hash_std::hash_audio;
