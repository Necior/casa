{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    clippy
    poetry
    rustc
    rustfmt
  ];
}
