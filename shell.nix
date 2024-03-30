{ pkgs ? import ./pkgs.nix {}, ci ? false }:

with pkgs;
mkShell {
  nativeBuildInputs = [
    gitAndTools.gh
    # Rust
    rustc
    rustc-wasm32
    cargo
    gcc
    llvmPackages.bintools
    rustfmt
    clippy
    wasm-pack
  ];
  # Don't set rpath for native addons
  NIX_DONT_SET_RPATH = true;
  NIX_NO_SELF_RPATH = true;
  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
  shellHook = ''

  '';
}
