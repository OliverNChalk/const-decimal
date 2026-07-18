# Based on: https://crane.dev/examples/quick-start-workspace.html

{
  inputs = {
    crane.url = "github:ipetkov/crane";
    nixpkgs.url = "github:NixOS/nixpkgs/release-26.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      crane,
      nixpkgs,
      rust-overlay,
      self,
      treefmt-nix,
      ...
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      mkPkgs =
        system:
        import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

      mkAugur =
        system:
        let
          pkgs = mkPkgs system;

          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            pname = "const-decimal";
            version = "0.0.0";
            strictDeps = true;
            dontStrip = true;
          };

          # Build & cache deps across all packages.
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        in
        {
          checks = {
            check = craneLib.mkCargoDerivation (
              commonArgs
              // {
                inherit cargoArtifacts;
                pnameSuffix = "-check";
                buildPhaseCargoCommand = "cargo check --all-features";
                nativeBuildInputs = commonArgs.nativeBuildInputs or [ ] ++ [ rustToolchain ];
              }
            );
            test = craneLib.cargoTest (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoTestExtraArgs = "--all-features";
              }
            );
            clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-features --all-targets --tests -- --deny warnings";
              }
            );
            doc = craneLib.cargoDoc (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoDocExtraArgs = "--all-features --no-deps";
              }
            );
          };
        };

      systemOutputs = builtins.listToAttrs (
        map (system: {
          name = system;
          value = mkAugur system;
        }) supportedSystems
      );

      treefmtEval = forAllSystems (
        system:
        let
          pkgs = mkPkgs system;
          rustNightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
        in
        treefmt-nix.lib.evalModule pkgs {
          projectRootFile = "flake.nix";
          programs.nixfmt.enable = true;
          programs.deadnix.enable = true;
          programs.rustfmt = {
            enable = true;
            package = rustNightly;
          };
          programs.prettier.enable = true;
          programs.taplo.enable = true;
        }
      );
    in
    {
      formatter = forAllSystems (system: treefmtEval.${system}.config.build.wrapper);

      devShells = forAllSystems (
        system:
        let
          pkgs = mkPkgs system;
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo-machete
              omnix
              rustup
              treefmtEval.${system}.config.build.wrapper
            ];
          };
        }
      );

      checks = forAllSystems (
        system:
        systemOutputs.${system}.checks
        // {
          treefmt = treefmtEval.${system}.config.build.check self;
        }
      );
    };
}
