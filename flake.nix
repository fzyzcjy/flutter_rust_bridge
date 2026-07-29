{
  description = "Flutter/Dart <-> Rust binding generator, feature-rich, but seamless and simple.";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-26.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            flutter
            glib
            libepoxy
            pango
            pkg-config
            ninja
            at-spi2-core
            gtk3
            rustup
          ];
        };
      }
    );
}
