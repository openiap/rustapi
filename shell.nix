{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  name = "dotnet-env";
  packages = with pkgs.dotnetCorePackages; [
    sdk_8_0
    sdk_6_0
  ];
}
