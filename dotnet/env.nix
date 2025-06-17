{ pkgs }:

{
  packages = with pkgs.dotnetCorePackages; [
    (combinePackages [
      sdk_6_0
      sdk_8_0
      sdk_9_0
    ])
  ];
  shellHook = ''
    export DOTNET_ROOT=${pkgs.dotnet-sdk}
  '';
}
