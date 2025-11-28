{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    unixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };
  outputs = { self, nixpkgs, unixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages."${system}";
      upkgs = unixpkgs.legacyPackages."${system}";
    in
    {
      devShells."${system}".default = pkgs.mkShell {
        packages = [
          upkgs.cargo
          upkgs.rustc
          upkgs.rust-analyzer
          upkgs.rustfmt
          upkgs.clippy
        ];
      };
      formatter."${system}" = pkgs.nixpkgs-fmt;
    };
}

