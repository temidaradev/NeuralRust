{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = [
    pkgs.qt6.full
    pkgs.clang
    pkgs.cmake
    pkgs.gcc
    pkgs.pkg-config
  ];
}
