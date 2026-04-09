#!/usr/bin/env bash
# Quick launcher for native (Blitz) mode with NixOS library paths
set -e

# Pkg-config paths for building
export PKG_CONFIG_PATH="/nix/store/g3xkwyfnh4lifwyx6kmhnpz9bw2540hm-libxcb-1.17.0-dev/lib/pkgconfig:/nix/store/5s110bxdr4jvxi3wjd2dwwjv9inn8ddp-libxkbcommon-1.11.0-dev/lib/pkgconfig:/nix/store/94j5bdf27k7f95hdw1ppqwggwxjw76jk-expat-2.7.3-dev/lib/pkgconfig:/nix/store/ss8rcralsb3b192371ma7kn9wvl3kdx4-freetype-2.13.3-dev/lib/pkgconfig:/nix/store/pp2fbp19cabbdb2dhslf3qnxbsyyvnxm-fontconfig-2.17.1-dev/lib/pkgconfig:/nix/store/ayhcy9p6rshhda432gc1j3nqbhfklpi2-harfbuzz-12.2.0-dev/lib/pkgconfig:/nix/store/ydrckgnllgg8nmhdwni81h7xhcpnrlhd-openssl-3.6.0-dev/lib/pkgconfig:$PKG_CONFIG_PATH"

# Runtime library paths for Blitz (Vulkan, Wayland, fontconfig, xkbcommon)
export LD_LIBRARY_PATH="/nix/store/h9grlk1b07l80k6rnmyliawg25bdpsv5-wayland-1.24.0/lib:/nix/store/b1bldnpjpys7np3361plhp2wxcaw9iwr-vulkan-loader-1.4.328.0/lib:/nix/store/bhmkfqa9b9sakim9zan85ylsgam03b0x-libxkbcommon-1.11.0/lib:/nix/store/5l5wsn4qxkw90h6sxxz43gbnar7w5x8s-fontconfig-2.17.1-lib/lib:$LD_LIBRARY_PATH"

# Display vars for Wayland/X11
export WAYLAND_DISPLAY="${WAYLAND_DISPLAY:-wayland-0}"
export XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/run/user/$(id -u)}"
export DISPLAY="${DISPLAY:-:0}"

cargo run --features native --no-default-features "$@"
