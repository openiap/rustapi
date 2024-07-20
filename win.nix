let
  # For some reason I'm not getting this backport in stable, so we're just gonna pull unstable.
  # https://github.com/NixOS/nixpkgs/pull/139697
  unstable = import (fetchTarball https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz);

  pkgs = unstable {
    crossSystem = { config = "x86_64-w64-mingw32"; };
    overlays = [
      # We're using an overlay for Rust. I tested with native NixOS Rust and I can reproduce the same issue,
      # it just likes to compile the entirety of Rust which isn't super fun.
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  };
in pkgs.mkShell { 
  nativeBuildInputs = with pkgs; [
    # Just packaged Rust from our overlay.
    (pkgs.buildPackages.rust-bin.nightly.latest.default.override {
      extensions = [ "rust-src" ];
      targets = [ "x86_64-pc-windows-gnu" "x86_64-unknown-linux-gnu" ];
    })

    # Some certs weren't working properly for me in the shell, so I just wanted to make sure.
    buildPackages.cacert

    # I prefer pure shells for testing, so this just provides standard utilities.
    buildPackages.busybox
  ];

  buildInputs = with pkgs; [
    # The library at fault— libpthread for Windows.
    windows.pthreads
  ];
}