{ pkgs }:

{
  packages = [
    pkgs.python311
    pkgs.python311Packages.pip
    pkgs.python311Packages.setuptools
  ];
  shellHook = "";
}
