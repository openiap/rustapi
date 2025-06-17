{ pkgs ? import <nixpkgs> {} }:

let
  env = import ./env.nix { inherit pkgs; };
in
pkgs.mkShell {
  name = "pwsh-env";
  packages = env.packages;
  shellHook = env.shellHook;
}
