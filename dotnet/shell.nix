{ pkgs ? import <nixpkgs> {
    config = {
      permittedInsecurePackages = [
        "dotnet-sdk-6.0.428"
      ];
    };
  }
}:

let
  env = import ./env.nix { inherit pkgs; };
in
pkgs.mkShell {
  name = "dotnet-env";
  packages = env.packages;
  shellHook = env.shellHook;
}
