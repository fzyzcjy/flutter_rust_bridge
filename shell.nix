{
  nixpkgs ? <nixpkgs>,
}:
let
  pkgs = import nixpkgs { };
in
pkgs.mkShell {
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
}
