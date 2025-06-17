{ pkgs }:

{
  packages = [
    pkgs.go
    pkgs.delve
  ];
  shellHook = ''
    export LD_LIBRARY_PATH=$PWD/lib:$LD_LIBRARY_PATH
    export CGO_LDFLAGS="-L$PWD/lib -lopeniap-linux-x64"
  '';
}