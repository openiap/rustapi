{ pkgs }:

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
  # export LD_LIBRARY_PATH=${pkgs.zlib}/lib:${pkgs.libglvnd}/lib:${pkgs.glib}/lib:${pkgs.glib}/lib/glib-2.0:$LD_LIBRARY_PATH
  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.zlib}/lib:$LD_LIBRARY_PATH
    export LD_LIBRARY_PATH=${pkgs.libglvnd}/lib:$LD_LIBRARY_PATH

    # glib libraries (libgthread-2.0.so.0, libglib-2.0.so.0, libgobject-2.0.so.0)
    export LD_LIBRARY_PATH=${pkgs.glib.out}/lib:$LD_LIBRARY_PATH
    export LD_LIBRARY_PATH=${pkgs.glib.out}/lib/glib-2.0:$LD_LIBRARY_PATH
    export LD_LIBRARY_PATH=${pkgs.glib.out}/lib/gio/modules:$LD_LIBRARY_PATH
  '';
}
