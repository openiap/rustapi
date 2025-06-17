{ pkgs }:

{
  packages = [
    pkgs.nodejs_20
    pkgs.nodePackages.npm
  ];
  shellHook = "";
}
