const {
  build_bundle,
  calculate_commitment,
  prove,
  verify,
} = require('../pkg-node/allfeat_ats_zkp_wasm.js');
const { PK } = require('./pk.js');
const { VK } = require('./vk.js');

const fs = require('node:fs');

(async () => {
    // Example data
    const title = 'Song Title';
    const audioBytes = new Uint8Array(fs.readFileSync('./sample-audio.mp3'));
    const creators = [{ fullName: 'Alice', email: 'alice@example.com', roles: ['AT'] }];
    const timestamp = BigInt(Math.floor(Date.now() / 1000));

    // Build the bundle of data to be proven
    const { bundle } = build_bundle(title, audioBytes, creators, timestamp);
    const secret = bundle.secret;
    const publics = [
        bundle.hash_title,
        bundle.hash_audio,
        bundle.hash_creators,
        bundle.commitment,
        bundle.timestamp,
        bundle.nullifier,
    ];
    console.log('secret (DO NOT REVEAL):', secret);

    // Example: Recompute the commitment using an existing secret
    // This is useful when you want to verify or recompute the commitment later
    const recomputedCommitment = calculate_commitment(title, audioBytes, creators, secret);
    console.log('recomputed commitment:', recomputedCommitment);
    console.log('commitments match:', recomputedCommitment === bundle.commitment);

    // Generate the proof
    const { proof, publics: publicsProof } = prove(PK, secret, publics);
    console.log('proof:', proof);
    console.log('publics:', publicsProof);

    // Verify the proof
    const ok = verify(VK, proof, publicsProof);
    console.log('verify:', ok);
})();
