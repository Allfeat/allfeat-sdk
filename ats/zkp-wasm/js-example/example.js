const {
  build_bundle,
  prove,
} = require('../pkg-node/allfeat_ats_zkp_wasm.js');
const { PK } = require('./pk.js');

const fs = require('node:fs');

(async () => {
    const title = 'Song Title';
    const audioBytes = new Uint8Array(fs.readFileSync('./sample-audio.mp3'));
    const creators = [{ fullName: 'Alice', email: 'alice@example.com', roles: ['AT'] }];
    const timestamp = BigInt(Math.floor(Date.now() / 1000));

    const { bundle } = build_bundle(title, audioBytes, creators, timestamp);

    const { proof, publics: publicsProof } = prove(
        PK,
        bundle.secret,
        [
          bundle.hash_audio,
          bundle.hash_title,
          bundle.hash_creators,
          bundle.commitment,
          bundle.timestamp,
          bundle.nullifier,
        ]
    );
    console.log('proof:', proof);
    console.log('publics:', publicsProof);
})();
