{ pkgs ? import ./pkgs.nix {}, ci ? false }:

with pkgs;
mkShell {
  nativeBuildInputs = [
    gitAndTools.gh
    # Rust
    rustc
    cargo
    gcc
    rustfmt
    clippy
    # Fly
    flyctl
  ];
  # Don't set rpath for native addons
  NIX_DONT_SET_RPATH = true;
  NIX_NO_SELF_RPATH = true;
  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  shellHook = ''

  '';
}
