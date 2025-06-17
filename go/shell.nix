{ pkgs ? import <nixpkgs> {} }:

let
  env = import ./env.nix { inherit pkgs; };
in
pkgs.mkShell {
  name = "go-env";
  packages = env.packages;
  shellHook = env.shellHook;
}
