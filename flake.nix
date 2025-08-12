{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs
  }: let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
    };
  in {
    devShell.${system} = pkgs.mkShell {
      buildInputs = with pkgs; [
        at-spi2-atk
        atkmm
        cairo
        cargo-tauri
        gdk-pixbuf
        glib
        gobject-introspection
        gtk3
        harfbuzz
        leptosfmt
        librsvg
        libsoup_3
        openssl
        pango
        pkg-config
        trunk
        webkitgtk_4_1
      ];
    };
  };
}
