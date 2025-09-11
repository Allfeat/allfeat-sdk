# ATS Certificate Parser

A WebAssembly module for parsing ATS certificates in JSON format, compatible with TypeScript/JavaScript applications including Next.js, Node.js, and browser environments.

## Prerequisites

Install `wasm-pack` to build the WASM module:

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Building for Different Environments

### For Next.js / Modern Web Applications (Bundler)

```bash
wasm-pack build --target bundler --out-dir pkg
```

This generates:
- `pkg/ats_cert_parser_bg.wasm` - The WASM binary
- `pkg/ats_cert_parser.js` - JavaScript bindings
- `pkg/ats_cert_parser.d.ts` - TypeScript definitions
- `pkg/package.json` - Package configuration

### For Node.js Applications

```bash
wasm-pack build --target nodejs --out-dir pkg-node
```

### For Browser (ES Modules)

```bash
wasm-pack build --target web --out-dir pkg-web
```

## Integration in TypeScript/JavaScript Applications

### Next.js Integration

1. Build the WASM module:
```bash
wasm-pack build --target bundler --out-dir pkg
```

2. Copy the `pkg` folder to your Next.js project (e.g., `lib/ats-cert-parser`)

3. Create a wrapper module to handle async initialization:

```typescript
// lib/ats-cert-parser/index.ts
import init, { 
  parseAtsCertificate, 
  parseAtsCertificateToJs,
  AtsCertificate,
  Creator 
} from './pkg/ats_cert_parser';

let initialized = false;

export async function initializeWasm() {
  if (!initialized) {
    await init();
    initialized = true;
  }
}

export { 
  parseAtsCertificate, 
  parseAtsCertificateToJs,
  AtsCertificate,
  Creator 
};
```

4. Use in your Next.js components:

```typescript
// app/components/CertificateParser.tsx
import { useEffect, useState } from 'react';
import { initializeWasm, parseAtsCertificateToJs } from '@/lib/ats-cert-parser';

export function CertificateParser() {
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    initializeWasm().then(() => setIsReady(true));
  }, []);

  const parseCertificate = (jsonStr: string) => {
    if (!isReady) return;
    
    try {
      const result = parseAtsCertificateToJs(jsonStr);
      console.log('Parsed certificate:', result);
      return result;
    } catch (error) {
      console.error('Parse error:', error);
    }
  };

  // Your component logic here
}
```

### Node.js Integration

1. Build for Node.js:
```bash
wasm-pack build --target nodejs --out-dir pkg-node
```

2. Use in your Node.js application:

```javascript
// index.js
const { 
  parseAtsCertificate, 
  parseAtsCertificateToJs,
  AtsCertificate,
  Creator 
} = require('./pkg-node/ats_cert_parser');

const jsonStr = `{
  "id_allfeat": "285328923",
  "version_number": "v1.0",
  "title": "My Certificate",
  "asset_filename": "asset.mp3",
  "creators": [
    {
      "fullname": "John Doe",
      "email": "john@example.com",
      "roles": ["Author", "Composer"]
    }
  ]
}`;

// Parse to JavaScript object
const certificate = parseAtsCertificateToJs(jsonStr);
console.log(certificate);

// Or work with the typed class
const cert = parseAtsCertificate(jsonStr);
console.log(`ID: ${cert.idAllfeat}`);
console.log(`Title: ${cert.title}`);
console.log(`Creators count: ${cert.getCreatorsCount()}`);
```

## API Reference

All functions and properties use camelCase naming in JavaScript:

### Functions

- `parseAtsCertificate(jsonStr: string): AtsCertificate` - Parse JSON string to AtsCertificate instance
- `parseAtsCertificateToJs(jsonStr: string): object` - Parse JSON string directly to JavaScript object

### AtsCertificate Class

Properties (getters/setters):
- `idAllfeat: string`
- `versionNumber: string`
- `title: string`
- `assetFilename: string`

Methods:
- `constructor(idAllfeat: string, versionNumber: string, title: string, assetFilename: string)`
- `addCreator(creator: Creator): void`
- `getCreatorsCount(): number`
- `toJson(): object`
- `fromJson(value: object): AtsCertificate`

### Creator Class

Properties (getters/setters):
- `fullname: string`
- `email: string`
- `roles: string[]`

Methods:
- `constructor(fullname: string, email: string, roles: string[])`

## Package.json Scripts

Add these scripts to your `package.json` for convenience:

```json
{
  "scripts": {
    "build:wasm": "wasm-pack build --target bundler --out-dir pkg",
    "build:wasm:node": "wasm-pack build --target nodejs --out-dir pkg-node",
    "build:wasm:web": "wasm-pack build --target web --out-dir pkg-web"
  }
}
```

## Testing

Run Rust tests:
```bash
cargo test
```

Run WASM tests:
```bash
wasm-pack test --headless --firefox
```

## Troubleshooting

### Next.js specific issues

If you encounter issues with Next.js, you may need to update your `next.config.js`:

```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack: (config) => {
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: true,
    };
    return config;
  },
};

module.exports = nextConfig;
```

### TypeScript types not found

Ensure the generated `.d.ts` file is included in your `tsconfig.json`:

```json
{
  "compilerOptions": {
    "paths": {
      "@/lib/ats-cert-parser/*": ["./lib/ats-cert-parser/*"]
    }
  }
}