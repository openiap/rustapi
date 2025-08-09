{ pkgs }:

{
  packages = [
    pkgs.python311
    pkgs.python311Packages.pip
    pkgs.python311Packages.setuptools
    pkgs.python311Packages.twine
  ];
  shellHook = "";
}
