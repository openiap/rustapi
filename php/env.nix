{ pkgs }:

let
  myPhp = pkgs.php.buildEnv {
    extensions = { enabled, all }: enabled ++ [
      all.imagick
      all.opcache
      all.xdebug
      all.ffi
    ];
    extraConfig = ''
      memory_limit = 256M
      xdebug.mode = debug
      xdebug.start_with_request = yes
      xdebug.client_host = 127.0.0.1
      xdebug.client_port = 9003
      ffi.enable = 1
    '';
  };
in {
  packages = [ myPhp pkgs.phpPackages.composer ];
  shellHook = ''
    export PATH=${myPhp}/bin:$PATH
    export PHP_INI_SCAN_DIR=${myPhp}/etc/php:${myPhp}/lib
  '';
}
