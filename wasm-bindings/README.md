# Allfeat WebAssembly Bindings

Unified WebAssembly bindings for the Allfeat SDK, providing a single entry point for both client and MIDDS functionality with organized TypeScript exports.

## Features

- 🚀 **Unified Entry Point**: Single crate combining client and MIDDS functionality
- 📝 **TypeScript Support**: Well-organized TypeScript definitions and namespaces
- 🏗️ **Modular Design**: Separated client and MIDDS functionality
- 🔧 **WebAssembly Optimized**: Configured for optimal WASM bundle size

## Structure

```
allfeat-wasm-bindings/
├── Client/              # Blockchain client functionality
│   ├── AllfeatClient   # Main client class
│   ├── Utils/          # Client utilities  
│   ├── Transactions/   # Transaction functionality
│   └── Metrics/        # Metrics and monitoring
└── Midds/              # Music Industry Data Structures
    ├── MusicalWork/    # Musical work types
    ├── PartyIdentifier/# Party identifier types
    ├── Release/        # Release types
    ├── Track/          # Track types
    └── Shared/         # Shared utilities
```

## Usage

### JavaScript/TypeScript

```javascript
import init, { 
  Client, 
  Midds, 
  AllfeatClient,
  get_version 
} from 'allfeat-wasm-bindings';

await init();

// Use the client
const client = await Client.AllfeatClient.createClient();
const balance = await client.getBalanceOf("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");

// Use MIDDS
const middsId = new Midds.MiddsId(123n);
console.log(middsId.value);

// Get version info
console.log(get_version());
```

## Building

Build the WebAssembly package using wasm-pack:

```bash
# From the wasm-bindings directory
wasm-pack build --target web --out-dir pkg

# Or from the workspace root
wasm-pack build wasm-bindings --target web --out-dir wasm-bindings/pkg
```

## Development

The bindings are organized to provide:

1. **Clear separation** between client and MIDDS functionality
2. **TypeScript-friendly exports** with proper namespacing
3. **Minimal bundle size** through selective re-exports
4. **Developer-friendly API** with intuitive organization

## License

GPL-3.0-only - see the main workspace LICENSE file.