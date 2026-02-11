{
  description = "Flake containing comsolith nix derivation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    nix-filter.url = "github:numtide/nix-filter";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      nix-filter,
      crane,
      fenix,
    }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        craneLib = (crane.mkLib pkgs).overrideToolchain fenix.packages.${system}.stable.toolchain;

        pkgDef = {
          pname = "cosmolith";
          version = "0.1.0";
          src = nix-filter.lib.filter {
            root = ./.;
            exclude = [
              ./.gitignore
              ./flake.nix
              ./flake.lock
            ];
          };
          nativeBuildInputs = with pkgs; [
            pkg-config
            autoPatchelfHook
          ];
          buildInputs = with pkgs; [
            udev
            libinput
            libxkbcommon
            glib # For gobject
            libglvnd # For libEGL
            libpulseaudio
            dbus.dev
          ];
          runtimeDependencies = with pkgs; [
            wayland
            libglvnd
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly pkgDef;
        cosmolith = craneLib.buildPackage (
          pkgDef
          // {
            inherit cargoArtifacts;
          }
        );
      in
      {
        checks = {
          inherit cosmolith;
        };

        packages.default = cosmolith;

        apps.default = flake-utils.lib.mkApp {
          drv = cosmolith;
        };

        devShells.default = pkgs.mkShell rec {
          inputsFrom = builtins.attrValues self.checks.${system};
          LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath (
            builtins.concatMap (d: d.runtimeDependencies) inputsFrom
          );
        };
      }
    );

  nixConfig = {
    # Cache for the Rust toolchain in fenix
    extra-substituters = [ "https://nix-community.cachix.org" ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };
}
