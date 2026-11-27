{
  description = "Math channel — Oxframe trial development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          rust = pkgs.rust-bin.stable."1.97.1".default.override {
            extensions = [ "rust-src" "rustfmt" "clippy" ];
          };
        in {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rust stdenv.cc pkg-config ffmpeg typst git
              fontconfig freetype openssl
            ];
            OXFRAME_NO_FFMPEG_DOWNLOAD = "1";
          };
        });
    };
}
