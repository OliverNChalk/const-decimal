{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {
      inherit system overlays;
    };
    rustToolchain = pkgs.rust-bin.stable.latest.minimal.override {
      extensions = [
        "rust-src"
        "clippy"
        "rust-analyzer"
      ];
    };

    buildInputs = with pkgs; [
      rustPlatform.bindgenHook
      rustToolchain
      rust-bin.nightly.latest.rustfmt # Use nightly formatter which has some better features.
      cargo-nextest
      taplo # Toml toolkit for formatting `Cargo.toml`.
    ];
    nix_tools = with pkgs; [
      alejandra # Formatter for nix code. `alejandra .`
      deadnix # Check for dead nix code.
      statix # Code checker.
    ];
  in {
    # Dev.
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = buildInputs ++ nix_tools;
    };
  };
}
