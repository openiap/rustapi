{ pkgs }:

{
  packages = [
    pkgs.openjdk21
    pkgs.maven
    pkgs.gnupg
    # pkgs.sdkman # Uncomment if you want sdkman
  ];
  shellHook = "";
}
