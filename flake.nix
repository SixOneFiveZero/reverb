{
  description = "Rever dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };

      # Define rust version here
      rustToolchain = pkgs.rust-bin.stable.latest.default;

    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          rustToolchain
          pkgs.pkg-config
          pkgs.rust-analyzer
          pkgs.clippy
          pkgs.alsa-lib
        ];
      };
    };
}
