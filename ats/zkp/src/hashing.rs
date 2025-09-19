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
        let h1 = hash_title("Hello World");
        let h2 = hash_title("Hello World");
        let h3 = hash_title("Hello  World"); // different bytes
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
        assert!(!h1.is_zero());
    }

    #[test]
    fn title_hash_matches_manual_sha256() {
        let title = "Song · Title · 2025";
        let expected = fr_from_bytes_sha256(title.as_bytes());
        assert_eq!(hash_title(title), expected);
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
            full_name: "Alice Smith",
            email: "alice@example.org",
            roles: Roles {
                author: true,
                composer: false,
                arranger: true,
                adapter: false,
            },
            ipi: Some("12345678901"),
            isni: Some("0000000121032683"),
        };
        let c2 = Creator {
            full_name: "Bob-J.",
            email: "bob@example.org",
            roles: Roles {
                author: false,
                composer: true,
                arranger: false,
                adapter: true,
            },
            ipi: None,
            isni: Some("000000012146438X"),
        };

        let h1 = hash_creators(&[c1.clone(), c2.clone()]);
        let h2 = hash_creators(&[c1.clone(), c2.clone()]);
        assert_eq!(h1, h2, "same list => same hash");

        let h_swapped = hash_creators(&[c2, c1]);
        assert_ne!(h1, h_swapped, "order must affect the hash");
    }

    #[test]
    fn creators_hash_normalizes_email_and_trims_fields() {
        // Same logical creator, but with casing/whitespace changes
        let a = Creator {
            full_name: "  Alice Smith  ",
            email: "  ALICE@Example.ORG ",
            roles: Roles {
                author: true,
                composer: false,
                arranger: false,
                adapter: false,
            },
            ipi: Some(" 00123456789 "),
            isni: Some(" 0000000121032683 "),
        };
        let b = Creator {
            full_name: "Alice Smith",
            email: "alice@example.org",
            roles: Roles {
                author: true,
                composer: false,
                arranger: false,
                adapter: false,
            },
            ipi: Some("00123456789"),
            isni: Some("0000000121032683"),
        };
        let h_a = hash_creators(&[a]);
        let h_b = hash_creators(&[b]);
        assert_eq!(h_a, h_b, "email must be lowercased; fields trimmed");
    }

    #[test]
    fn creators_hash_matches_manual_concatenation() {
        // Build the manual buffer as per spec:
        // <FullName><Email(lowercased, trimmed)><ROLES(AT/CP/AR/AD)><IPI?><ISNI?>
        let c = [
            Creator {
                full_name: "Alice Smith",
                email: "ALICE@EXAMPLE.ORG",
                roles: Roles {
                    author: true,
                    composer: true,
                    arranger: false,
                    adapter: false,
                },
                ipi: Some("123"),
                isni: Some("0000000121032683"),
            },
            Creator {
                full_name: "Bob",
                email: "bob@example.org",
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
        assert_eq!(hash_creators(&c), expected);
    }

    #[test]
    fn creators_hash_empty_list_is_sha256_of_empty_string() {
        let expected = fr_from_bytes_sha256(b"");
        assert_eq!(hash_creators(&[]), expected);
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
