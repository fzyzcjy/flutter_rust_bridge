{
  pkgs ? import <nixpkgs> { },
  ...
}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    flutter
  ];

  buildInputs = with pkgs; [
    glib
    libepoxy
    pango
    ninja
    at-spi2-core
    gtk3
    rustup
    xvfb-run
  ];
}
