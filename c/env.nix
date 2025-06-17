{ pkgs }:

{
  packages = [
    pkgs.gcc
    pkgs.gdb
    pkgs.gnumake
    pkgs.pkg-config
  ];
  shellHook = "";
}
