# ATS Certificate Generator

A WebAssembly module for generating ATS certificates in HTML and PDF formats, compatible with TypeScript/JavaScript applications including Next.js and browser environments.

## Prerequisites

Install `wasm-pack` to build the WASM module:

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Building

### For Browser/Next.js Applications (ES Modules)

```bash
wasm-pack build --target web --out-dir pkg
```

This generates:

-   `pkg/ats_cert_generator_bg.wasm` - The WASM binary
-   `pkg/ats_cert_generator.js` - JavaScript bindings
-   `pkg/ats_cert_generator.d.ts` - TypeScript definitions
-   `pkg/package.json` - Package configuration

## Integration in TypeScript/JavaScript Applications

### Next.js Integration

1. Build the WASM module:

```bash
wasm-pack build --target bundler --out-dir pkg
```

2. Copy the `pkg` folder to your Next.js project (e.g., `src/lib/ats-cert-generator`)

3. Configure Next.js for WASM support in `next.config.ts`:

```typescript
// next.config.ts
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
    webpack: (config, { isServer }) => {
        config.experiments = {
            ...config.experiments,
            asyncWebAssembly: true,
        };

        config.module.rules.push({
            test: /\.wasm$/,
            type: "webassembly/async",
        });

        return config;
    },
};

export default nextConfig;
```

4. Use in your Next.js components:

```typescript
// components/CertificateGenerator.tsx
import { useState } from "react";

export function CertificateGenerator() {
    const [isGenerating, setIsGenerating] = useState(false);

    const generateCertificate = async () => {
        if (typeof window === "undefined") return;

        try {
            setIsGenerating(true);

            // Dynamic import for client-side only
            const init = (
                await import("@/lib/ats-cert-generator/ats_cert_generator.js")
            ).default;
            const {
                generate_pdf_from_js_object,
                create_certificate_from_js_object,
            } = await import("@/lib/ats-cert-generator/ats_cert_generator.js");

            // Initialize WASM module
            await init();

            const certificateData = {
                title: "My Certificate",
                asset_filename: "asset.mp3",
                creators: [
                    {
                        fullname: "John Doe",
                        email: "john@example.com",
                        roles: ["Author", "Composer"],
                        ipi: "123456789",
                        isni: "0000000123456789",
                    },
                ],
                hash: "0x1234567890abcdef",
                timestamp: new Date()
                    .toISOString()
                    .replace("T", " ")
                    .replace(/\.\d{3}Z$/, " UTC"),
                current_page: 1,
                total_pages: 1,
            };

            // Generate HTML
            const html = create_certificate_from_js_object(certificateData);
            console.log("Generated HTML certificate");

            // Generate PDF
            const pdfBytes = generate_pdf_from_js_object(certificateData);
            const pdfBlob = new Blob([pdfBytes], { type: "application/pdf" });

            // Download PDF
            const url = URL.createObjectURL(pdfBlob);
            const link = document.createElement("a");
            link.href = url;
            link.download = "certificate.pdf";
            link.click();
            URL.revokeObjectURL(url);
        } catch (error) {
            console.error("Generation failed:", error);
        } finally {
            setIsGenerating(false);
        }
    };

    return (
        <button onClick={generateCertificate} disabled={isGenerating}>
            {isGenerating ? "Generating..." : "Generate Certificate"}
        </button>
    );
}
```

## API Reference

### Functions

-   `create_certificate_from_js_object(data: object): string` - Generate HTML certificate from JavaScript object
-   `generate_pdf_from_js_object(data: object): Uint8Array` - Generate PDF certificate from JavaScript object

### Certificate Data Structure

```typescript
interface CertificateData {
    title: string;
    asset_filename: string;
    creators: Creator[];
    hash?: string;
    timestamp: string; // Format: "YYYY-MM-DD HH:MM:SS UTC"
    current_page: number;
    total_pages: number;
}

interface Creator {
    fullname: string;
    email: string;
    roles: string[];
    ipi?: string;
    isni?: string;
}
```

## Features

-   **HTML Generation**: Creates styled HTML certificates using Handlebars templates
-   **PDF Generation**: Native PDF creation in WASM for consistent output
-   **Branding**: Includes Allfeat branding with proper colors and styling
-   **Accessibility**: Generated HTML includes proper ARIA labels and semantic structure
-   **Performance**: Optimized WASM binary for fast generation

## Package.json Scripts

Add these scripts to your `package.json` for convenience:

```json
{
    "scripts": {
        "build:wasm": "wasm-pack build --target web --out-dir pkg",
        "test:wasm": "wasm-pack test --headless --firefox"
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

### Build Errors with wasm-opt

If you encounter wasm-opt optimization errors, add this to `Cargo.toml`:

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

### Next.js Build Issues

If Next.js fails to build with WASM imports:

1. Ensure dynamic imports are used (not static)
2. Wrap WASM usage in `typeof window !== 'undefined'` checks
3. Consider excluding the lib folder from linting:

```javascript
// .eslintignore
src/lib/**

// .prettierignore
src/lib/**
```

## License

See the main repository LICENSE file.
