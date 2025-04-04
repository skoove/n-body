{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    librarys = with pkgs; [
      lld
      wayland
      alsa-lib
      libudev-zero
      wayland
      libxkbcommon
      vulkan-loader
      rustfmt
      mold
      rustc
    ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo
        clippy
        clang
        rust-analyzer
        pkg-config
      ] ++ librarys;

      LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath librarys}";
    };
  };
}
