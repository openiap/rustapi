{ pkgs ? import <nixpkgs> {} }:

let
  myPhp = pkgs.php.buildEnv {
    extensions = { enabled, all }: enabled ++ [ all.imagick all.opcache all.xdebug all.ffi ];
    extraConfig = ''
      memory_limit = 256M
      zend_extension = /nix/store/v1cwcgahcv8f501mfrjv2js1an23ffba-php-xdebug-3.3.0alpha3/lib/php/extensions/xdebug.so
      xdebug.mode = debug
      xdebug.start_with_request = yes
      xdebug.client_host = 127.0.0.1
      xdebug.client_port = 9003
      
      extension = ffi
      ffi.enable = 1
    '';
  };
in
pkgs.mkShell {
  buildInputs = [
    myPhp
    pkgs.php.packages.composer
  ];

  # Composer setup for Guzzle
  shellHook = ''
    if [ ! -f composer.json ]; then
      echo "Initializing composer project and installing Guzzle..."
      composer init --no-interaction --name="example/project" --require="guzzlehttp/guzzle:^7.0" || true
      composer install
    fi
  '';
}
