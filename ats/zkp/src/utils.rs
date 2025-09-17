use ark_bn254::Fr;
use ark_crypto_primitives::sponge::{
    CryptographicSponge,
    poseidon::{PoseidonConfig, PoseidonSponge},
};
use ark_ff::PrimeField;

pub fn fr_from_hex_be(h: &str) -> Fr {
    let s = h.trim_start_matches("0x");
    let bytes = hex::decode(s).expect("invalid hex");
    let mut be = [0u8; 32];
    let off = 32 - bytes.len();
    be[off..].copy_from_slice(&bytes);
    Fr::from_be_bytes_mod_order(&be)
}

pub fn fr_u64(x: u64) -> Fr {
    Fr::from(x)
}

pub fn poseidon_h4_offchain(a: Fr, b: Fr, c: Fr, d: Fr, cfg: &PoseidonConfig<Fr>) -> Fr {
    let mut sp = PoseidonSponge::<Fr>::new(cfg);
    sp.absorb(&vec![a, b, c, d]);
    sp.squeeze_field_elements(1)[0]
}

pub fn poseidon_h2_offchain(x: Fr, y: Fr, cfg: &PoseidonConfig<Fr>) -> Fr {
    let mut sp = PoseidonSponge::<Fr>::new(cfg);
    sp.absorb(&vec![x, y]);
    sp.squeeze_field_elements(1)[0]
}
