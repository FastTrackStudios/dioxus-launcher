{
  description = "Dioxus Launcher development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer

            # Build tools
            pkg-config
            cmake

            # Dioxus desktop (Wry/WebKit) dependencies
            openssl
            gtk3
            webkitgtk_4_1
            glib
            cairo
            pango
            gdk-pixbuf
            libsoup_3
            atk
            harfbuzz

            # Dioxus native (Blitz/Vello) dependencies
            fontconfig
            freetype
            expat
            vulkan-loader
            vulkan-headers
            libxkbcommon
            wayland
            wayland-protocols
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            xorg.libxcb

            # Clipboard
            wl-clipboard
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
              pkgs.gtk3
              pkgs.webkitgtk_4_1
              pkgs.glib
              pkgs.cairo
              pkgs.pango
              pkgs.gdk-pixbuf
              pkgs.libsoup_3
              pkgs.harfbuzz
              pkgs.vulkan-loader
              pkgs.libxkbcommon
              pkgs.wayland
              pkgs.fontconfig
              pkgs.freetype
            ]}:$LD_LIBRARY_PATH"
          '';
        };
      }
    );
}
