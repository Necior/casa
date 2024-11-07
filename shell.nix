{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    cargo-edit
    clippy
    rustc
    rustfmt
  ];
}
