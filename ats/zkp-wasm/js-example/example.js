const {
  build_bundle,
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
    const publics = [
        bundle.hash_title,
        bundle.hash_audio,
        bundle.hash_creators,
        bundle.commitment,
        bundle.timestamp,
        bundle.nullifier,
    ];

    // Generate the proof
    const { proof, publics: publicsProof } = prove(
        PK,
        bundle.secret,
        publics
    );
    console.log('proof:', proof);
    console.log('publics:', publicsProof);

    // Verify the proof
    const ok = verify(VK, proof, publicsProof);
    console.log('verify:', ok);
})();
