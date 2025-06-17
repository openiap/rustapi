{ pkgs ? import <nixpkgs> {} }:

let
  env = import ./env.nix { inherit pkgs; };
in
pkgs.mkShell {
  name = "java-env";
  packages = env.packages;
  shellHook = env.shellHook;
}
