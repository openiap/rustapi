{ pkgs }:

let
  glvnd = pkgs.libglvnd;
  glib = pkgs.glib;
in
{
  packages = [
    # pkgs.python311
    pkgs.python311Full
    pkgs.python311Packages.pip
    pkgs.python311Packages.setuptools
    pkgs.python311Packages.twine

    pkgs.mesa
    pkgs.mesa.drivers
    pkgs.zlib
    pkgs.gtk3
    pkgs.libGL
    pkgs.libGLU
    pkgs.xorg.libX11
    pkgs.xorg.libXext
    pkgs.xorg.libXrender
    pkgs.libxkbcommon
  ];
  shellHook = ''
    export LD_LIBRARY_PATH=${glvnd}/lib:${glib}/lib:$LD_LIBRARY_PATH
  '';
}
