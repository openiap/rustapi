{ pkgs ? import <nixpkgs> {} }:

let
  env = import ./env.nix { inherit pkgs; };
in
pkgs.mkShell {
  name = "node-env";
  packages = env.packages;
  shellHook = env.shellHook;
}
