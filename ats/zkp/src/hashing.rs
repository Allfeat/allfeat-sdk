//! Hashing utilities for titles, creators, and audio files into BN254 field elements (`Fr`).
//!
//! This module defines deterministic ways to hash musical metadata into field elements
//! for use inside zero-knowledge proof circuits or commitments.
//!
//! Hashing is done with **SHA-256** reduced modulo BN254 field order (Fr).
//!
//! # Provided functionality
//!
//! - [`hash_title`] — hash a song title (UTF-8).
//! - [`hash_creators`] — hash a list of creators with normalized fields.
//! - [`hash_audio`] (requires `std`) — hash an audio file in streaming mode.
//!
//! # Normalization rules
//!
//! - **Title**: raw UTF-8 bytes, no normalization.
//! - **Creator**:
//!   - `full_name`: trimmed.
//!   - `email`: trimmed and lowercased.
//!   - `roles`: rendered in fixed `AT/CP/AR/AD` order, only those set to true.
//!   - `ipi` and `isni`: trimmed if present.
//! - **Concatenation**: `<FullName><Email><Roles><IPI?><ISNI?>` per creator,
//!   no separators between creators.
//!
//! # Spec references
//!
//! These hashes correspond to the components used in the commitment scheme:
//!
//! ```text
//! hash_commitment = Poseidon(hash_file, hash_title, hash_creators, secret)
//! ```

use ark_bn254::Fr;
use ark_ff::PrimeField;
use sha2::{Digest, Sha256};

/// Convert a SHA-256 digest (32 bytes) into an `Fr` field element (BN254).
///
/// - Uses **big-endian** order.
/// - Reduces modulo BN254 field order via `Fr::from_be_bytes_mod_order`.
fn fr_from_sha256(digest32: [u8; 32]) -> Fr {
    Fr::from_be_bytes_mod_order(&digest32)
}

/// Hash a song title (UTF-8 string) into `Fr` using SHA-256.
///
/// - Takes the raw UTF-8 bytes of the title.
/// - Returns `Fr(SHA256(title))` reduced mod BN254.
///
/// Deterministic: same title always yields the same `Fr`.
pub fn hash_title_fr(title: &str) -> Fr {
    let mut hasher = Sha256::new();
    hasher.update(title.as_bytes());
    let digest = hasher.finalize();
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&digest);
    fr_from_sha256(arr)
}

/// Creator role flags, rendered in the fixed order `AT/CP/AR/AD`.
///
/// Each flag is boolean; multiple can be set to true.
/// Used to build the canonical "roles string" for creator hashing.
#[derive(Debug, Clone, Copy, Default)]
pub struct Roles {
    pub author: bool,   // AT
    pub composer: bool, // CP
    pub arranger: bool, // AR
    pub adapter: bool,  // AD
}

impl Roles {
    /// Render roles into a slash-separated abbreviation string.
    ///
    /// Always uses the order `AT/CP/AR/AD`.
    /// Only roles set to `true` are included.
    ///
    /// Examples:
    /// - `author=true, composer=true` → `"AT/CP"`
    /// - `arranger=true, adapter=true` → `"AR/AD"`
    /// - none → `""`
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

/// A single creator entry for hashing.
///
/// Fields:
/// - `full_name` (required)
/// - `email` (required, normalized to lowercase)
/// - `roles` (at least one `true` recommended)
/// - `ipi` (optional, numeric up to 11 digits)
/// - `isni` (optional, 16 digits or 15 + 'X')
#[derive(Debug, Clone)]
pub struct Creator {
    pub full_name: String,
    pub email: String,
    pub roles: Roles,
    pub ipi: Option<String>,
    pub isni: Option<String>,
}

/// Hash a list of creators into `Fr` using SHA-256.
///
/// Concatenates all creators’ fields in order without separators:
///
/// ```text
/// <FullName><Email><Roles><IPI?><ISNI?>
/// ```
///
/// where:
/// - `FullName` and `IPI`/`ISNI` are trimmed.
/// - `Email` is trimmed and lowercased.
/// - `Roles` is the string from [`Roles::to_abbrev`].
///
/// The list is order-sensitive: swapping creators produces a different hash.
///
/// Returns `Fr(SHA256(concatenated_bytes))`.
pub fn hash_creators_fr(creators: &[Creator]) -> Fr {
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

        if let Some(ipi) = c.ipi.as_deref() {
            buf.push_str(ipi.trim());
        }
        if let Some(isni) = c.isni.as_deref() {
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

    /// Hash an audio file into `Fr` using SHA-256 (streaming).
    ///
    /// - Reads the file in 64 KiB chunks (does not load whole file into memory).
    /// - Returns `Fr(SHA256(file_bytes))`.
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
}

#[cfg(feature = "std")]
pub use file_hash_std::hash_audio;

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::Zero;

    // --- Helpers for expected digests in tests ---
    fn fr_from_bytes_sha256(bytes: &[u8]) -> Fr {
        let mut h = Sha256::new();
        h.update(bytes);
        let digest = h.finalize();
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&digest);
        Fr::from_be_bytes_mod_order(&arr)
    }

    // ----------------------- hash_title -----------------------

    #[test]
    fn title_hash_is_deterministic_and_differs_on_input() {
        let h1 = hash_title_fr("Hello World");
        let h2 = hash_title_fr("Hello World");
        let h3 = hash_title_fr("Hello  World"); // different bytes
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
        assert!(!h1.is_zero());
    }

    #[test]
    fn title_hash_matches_manual_sha256() {
        let title = "Song · Title · 2025";
        let expected = fr_from_bytes_sha256(title.as_bytes());
        assert_eq!(hash_title_fr(title), expected);
    }

    // ----------------------- Roles::to_abbrev -----------------------

    #[test]
    fn roles_to_abbrev_fixed_order() {
        let r = Roles {
            author: true,
            composer: true,
            arranger: true,
            adapter: true,
        };
        assert_eq!(r.to_abbrev(), "AT/CP/AR/AD");

        let r = Roles {
            author: false,
            composer: true,
            arranger: false,
            adapter: true,
        };
        assert_eq!(r.to_abbrev(), "CP/AD");

        let r = Roles {
            author: true,
            composer: false,
            arranger: true,
            adapter: false,
        };
        assert_eq!(r.to_abbrev(), "AT/AR");

        let r = Roles::default();
        assert_eq!(r.to_abbrev(), "");
    }

    // ----------------------- hash_creators -----------------------

    #[test]
    fn creators_hash_is_deterministic_and_order_sensitive() {
        let c1 = Creator {
            full_name: "Alice Smith".to_string(),
            email: "alice@example.org".to_string(),
            roles: Roles {
                author: true,
                composer: false,
                arranger: true,
                adapter: false,
            },
            ipi: Some("12345678901".to_string()),
            isni: Some("0000000121032683".to_string()),
        };
        let c2 = Creator {
            full_name: "Bob-J.".to_string(),
            email: "bob@example.org".to_string(),
            roles: Roles {
                author: false,
                composer: true,
                arranger: false,
                adapter: true,
            },
            ipi: None,
            isni: Some("000000012146438X".to_string()),
        };

        let h1 = hash_creators_fr(&[c1.clone(), c2.clone()]);
        let h2 = hash_creators_fr(&[c1.clone(), c2.clone()]);
        assert_eq!(h1, h2, "same list => same hash");

        let h_swapped = hash_creators_fr(&[c2, c1]);
        assert_ne!(h1, h_swapped, "order must affect the hash");
    }

    #[test]
    fn creators_hash_normalizes_email_and_trims_fields() {
        // Same logical creator, but with casing/whitespace changes
        let a = Creator {
            full_name: "  Alice Smith  ".to_string(),
            email: "  ALICE@Example.ORG ".to_string(),
            roles: Roles {
                author: true,
                composer: false,
                arranger: false,
                adapter: false,
            },
            ipi: Some(" 00123456789 ".to_string()),
            isni: Some(" 0000000121032683 ".to_string()),
        };
        let b = Creator {
            full_name: "Alice Smith".to_string(),
            email: "alice@example.org".to_string(),
            roles: Roles {
                author: true,
                composer: false,
                arranger: false,
                adapter: false,
            },
            ipi: Some("00123456789".to_string()),
            isni: Some("0000000121032683".to_string()),
        };
        let h_a = hash_creators_fr(&[a]);
        let h_b = hash_creators_fr(&[b]);
        assert_eq!(h_a, h_b, "email must be lowercased; fields trimmed");
    }

    #[test]
    fn creators_hash_matches_manual_concatenation() {
        // Build the manual buffer as per spec:
        // <FullName><Email(lowercased, trimmed)><ROLES(AT/CP/AR/AD)><IPI?><ISNI?>
        let c = [
            Creator {
                full_name: "Alice Smith".to_string(),
                email: "ALICE@EXAMPLE.ORG".to_string(),
                roles: Roles {
                    author: true,
                    composer: true,
                    arranger: false,
                    adapter: false,
                },
                ipi: Some("123".to_string()),
                isni: Some("0000000121032683".to_string()),
            },
            Creator {
                full_name: "Bob".to_string(),
                email: "bob@example.org".to_string(),
                roles: Roles {
                    author: false,
                    composer: false,
                    arranger: true,
                    adapter: true,
                },
                ipi: None,
                isni: None,
            },
        ];

        // Manual buffer
        let mut buf = String::new();
        {
            // c[0]
            buf.push_str("Alice Smith");
            buf.push_str("alice@example.org"); // lowercased
            buf.push_str("AT/CP");
            buf.push_str("123");
            buf.push_str("0000000121032683");
            // c[1]
            buf.push_str("Bob");
            buf.push_str("bob@example.org");
            buf.push_str("AR/AD");
        }

        let expected = fr_from_bytes_sha256(buf.as_bytes());
        assert_eq!(hash_creators_fr(&c), expected);
    }

    #[test]
    fn creators_hash_empty_list_is_sha256_of_empty_string() {
        let expected = fr_from_bytes_sha256(b"");
        assert_eq!(hash_creators_fr(&[]), expected);
    }

    // ----------------------- hash_audio (std only) -----------------------

    #[cfg(feature = "std")]
    #[test]
    fn audio_hash_streaming_is_deterministic_and_changes_with_content() {
        use std::{fs::File, io::Write};

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("audio.raw");

        // Write >64KiB so we exercise chunked reading
        {
            let mut f = File::create(&path).unwrap();
            let data = vec![0xABu8; 150 * 1024]; // 150 KiB
            f.write_all(&data).unwrap();
        }

        let h1 = hash_audio(&path).unwrap();
        let h2 = hash_audio(&path).unwrap();
        assert_eq!(h1, h2, "same file => same hash");

        // Change file, hash changes
        {
            let mut f = File::options().append(true).open(&path).unwrap();
            f.write_all(&[0xCD, 0xEF]).unwrap();
        }
        let h3 = hash_audio(&path).unwrap();
        assert_ne!(h1, h3, "modified file => different hash");
    }

    #[cfg(feature = "std")]
    #[test]
    fn audio_hash_matches_manual_sha256() {
        use std::{fs::File, io::Write};

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("clip.pcm");

        // Small deterministic content
        {
            let mut f = File::create(&path).unwrap();
            f.write_all(b"\x01\x02\x03\x04hello-audio").unwrap();
        }

        // Manual expected Fr(SHA256(file_bytes))
        let expected = {
            let bytes = std::fs::read(&path).unwrap();
            fr_from_bytes_sha256(&bytes)
        };

        let got = hash_audio(&path).unwrap();
        assert_eq!(got, expected);
    }
}
