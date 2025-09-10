{
  craneLib,
  pkgs,
  inputs,
  commonArgs,
  cargoArtifacts,
  ...
}:
let

  inherit (pkgs) callPackage;
in
rec {
  kani = callPackage ./kani {
    inherit (inputs) rust-overlay;
  };
  moirai = callPackage ./moirai {
    inherit
      craneLib
      pkgs
      commonArgs
      cargoArtifacts
      ;
  };
  default = moirai;
}
