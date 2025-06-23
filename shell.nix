{ pkgs ? import <nixpkgs> {} }:

let
  phpEnv = import ./php/env.nix { inherit pkgs; };
  javaEnv = import ./java/env.nix { inherit pkgs; };
  dotnetEnv = import ./dotnet/env.nix { inherit pkgs; };
  nodeEnv = import ./node/env.nix { inherit pkgs; };
  pythonEnv = import ./python/env.nix { inherit pkgs; };
  pwshEnv = import ./pwsh/env.nix { inherit pkgs; };
  cEnv = import ./c/env.nix { inherit pkgs; };
  goEnv = import ./go/env.nix { inherit pkgs; };
in pkgs.mkShell {
  name = "combined-env";
  packages =
    phpEnv.packages ++
    javaEnv.packages ++
    dotnetEnv.packages ++
    nodeEnv.packages ++
    pythonEnv.packages ++
    pwshEnv.packages ++
    cEnv.packages ++
    goEnv.packages ++
    [
      # pkgs.vscode-extensions.vadimcn.vscode-lldb.adapter
      pkgs.protobuf
      pkgs.icu
    ];
  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.icu}/lib:$LD_LIBRARY_PATH
    ${phpEnv.shellHook}
    ${javaEnv.shellHook}
    ${dotnetEnv.shellHook}
    ${nodeEnv.shellHook}
    ${pythonEnv.shellHook}
    ${pwshEnv.shellHook}
    ${cEnv.shellHook}
    ${goEnv.shellHook}
  '';
}