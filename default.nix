let
  pkgs = import <nixpkgs> {};
  cargo = pkgs.callPackage ./Cargo.nix {};
in
cargo.rootCrate.build
