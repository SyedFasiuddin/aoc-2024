{
  description = "aoc-2024";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  };
  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages."aarch64-darwin";
    in
    {
      packages."aarch64-darwin".default = pkgs.stdenv.mkDerivation {
        name = "aoc";
        src = ./.;

        buildInputs = with pkgs; [
          rustc cargo rustfmt rust-analyzer clippy
        ];
      };
    };
}
