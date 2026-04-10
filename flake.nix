{
  description = "Dioxus Launcher — universal application launcher engine";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, flake-parts, crane, ... } @inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" "aarch64-linux" ];

      perSystem = { self', pkgs, lib, system, ... }:
        let
          rustToolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
            targets = [ "wasm32-unknown-unknown" ];
          };

          buildInputs = (with pkgs; [
            openssl openssl.dev pkg-config fontconfig freetype
          ])
          ++ lib.optionals pkgs.stdenv.isLinux (with pkgs; [
            glib gtk3 libsoup_3 webkitgtk_4_1 xdotool
            libx11 libxcursor libxrandr libxi libxcb
            libxkbcommon wayland
            libGL vulkan-loader
            gst_all_1.gstreamer gst_all_1.gst-plugins-base
            gst_all_1.gst-plugins-good gst_all_1.gst-plugins-bad
            harfbuzz expat
          ])
          ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
            apple-sdk_15
            libiconv
          ]);

          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.rustPlatform.bindgenHook
            pkgs.dioxus-cli
            pkgs.tailwindcss_4
          ];

          libPath = lib.makeLibraryPath (with pkgs;
            [ fontconfig freetype openssl ]
            ++ lib.optionals pkgs.stdenv.isLinux [
              libGL vulkan-loader gtk3 glib
              libx11 libxcb libxkbcommon wayland
              webkitgtk_4_1 libsoup_3 harfbuzz
              gst_all_1.gstreamer gst_all_1.gst-plugins-base
              gst_all_1.gst-plugins-good gst_all_1.gst-plugins-bad
            ]
          );

        in {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };

          devShells.default = pkgs.mkShell {
            packages = [
              rustToolchain
              pkgs.dioxus-cli
              pkgs.tailwindcss_4
              pkgs.cargo-watch
              pkgs.cargo-nextest
              pkgs.bacon
            ]
            ++ buildInputs
            ++ nativeBuildInputs;

            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

            LD_LIBRARY_PATH = lib.optionalString pkgs.stdenv.isLinux libPath;
            XDG_DATA_DIRS = lib.optionalString pkgs.stdenv.isLinux
              "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}";

            shellHook = ''
              echo ""
              echo "  Dioxus Launcher dev shell"
              echo "  Rust:     $(rustc --version)"
              echo "  dx:       $(dx --version 2>/dev/null || echo 'not available')"
              echo "  tailwind: $(tailwindcss --version 2>/dev/null || echo 'not available')"
              echo ""
            '';
          };
        };
    };
}
