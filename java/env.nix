{ pkgs }:

{
  packages = [
    pkgs.openjdk21
    pkgs.maven
    # pkgs.sdkman # Uncomment if you want sdkman
  ];
  shellHook = "";
}
