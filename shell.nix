{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
    rust-analyzer
    rustfmt
    clippy
  ];

  buildInputs = with pkgs; [
    alsa-lib
  ];
}
