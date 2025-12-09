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
    cargo-upgrades = pkgs.rustPlatform.buildRustPackage {
      name = "cargo-upgrades";
      src = builtins.fetchGit {
        url = "https://gitlab.com/kornelski/cargo-upgrades";
        rev = "4d18359ba87cd7ccb2fd0d9c975b2d85d5cb7e9c";
      };
      cargoHash = "sha256-bWVZAKH3F4BYcEujJ2uL+Iq7HDmFQHJ4eRB9xKujoO0=";
      doCheck = false; # Tests fail at the current revision.
      meta = {
        description = "Check for outdated dependencies in a cargo workspace";
        homepage = "https://gitlab.com/kornelski/cargo-upgrades";
      };
    };

    buildInputs = with pkgs; [
      rustPlatform.bindgenHook
      rustToolchain
      rust-bin.nightly.latest.rustfmt # Use nightly formatter which has some better features.
      cargo-nextest
      cargo-upgrades
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
