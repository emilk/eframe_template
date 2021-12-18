{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    utils.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, fenix, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        toolchain = with fenix.packages.${system};
          combine [
            stable.rustc
            stable.cargo
            stable.clippy
            targets."wasm32-unknown-unknown".stable.rust-std
          ];
        naersk-lib = (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        });
      in
      rec {
        # nix build
        packages.eframe_template = naersk-lib.buildPackage {
          root = ./.;
          doCheck = true;
          nativeBuildInputs = with pkgs; [ makeWrapper pkg-config ];
          buildInputs = with pkgs; [
            libxkbcommon
            openssl
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            xorg.libxcb
          ];
          overrideMain = (_: {
            postFixup = ''
              wrapProgram $out/bin/eframe_template \
                --prefix LD_LIBRARY_PATH ":" ${pkgs.libGL}/lib 
            '';
          });
        };
        defaultPackage = packages.eframe_template;

        # nix run
        apps.eframe_template = utils.lib.mkApp { drv = packages.eframe_template; };
        defautltApp = apps.eframe_template;

        # nix develop
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; packages.eframe_template.nativeBuildInputs ++ [
            toolchain
            httplz
            wasm-bindgen-cli
          ];
          buildInputs = packages.eframe_template.buildInputs;
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.libGL}/lib"
            export IS_NIX_BUILD=true
          '';
        };
      });
}
