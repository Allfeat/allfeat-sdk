{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    polkadot.url = "github:andresilva/polkadot.nix";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      polkadot,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
          polkadot.overlays.default
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            clang
            pkg-config
            openssl
            wasm-bindgen-cli
            wasm-pack
            subxt
            binaryen
            just
            cargo-release
            psvm
            typeshare

            # IDE requirements
            prettier-d-slim
            vtsls
            vue-language-server
            yaml-language-server
            typescript-language-server
          ];
        };
      }
    );
}
