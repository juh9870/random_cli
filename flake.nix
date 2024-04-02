{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    cargo2nix = {
      url = "github:cargo2nix/cargo2nix/release-0.11.0";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs = inputs:
    with inputs;

    # Build the output set for each default system and map system sets into
    # attributes, resulting in paths such as:
    # nix build .#packages.x86_64-linux.<name>
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:

      # let-in expressions, very similar to Rust's let bindings.  These names
      # are used to express the output but not themselves paths in the output.
      let

        # create nixpkgs that contains rustBuilder from cargo2nix overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };
        unstable = import nixpkgs-unstable { inherit system; };

        # create the workspace & dependencies package set
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          packageFun = import ./Cargo.nix;
          rustVersion = "1.75.0";
          rustProfile = "default";
          extraRustComponents = [ "clippy" "rust-src" "rustfmt" "rust-docs" ];
        };

        # The workspace defines a development shell with all of the dependencies
        # and environment settings necessary for a regular `cargo build`
        workspaceShell = rustPkgs.workspaceShell {
          # This adds cargo2nix to the project shell via the cargo2nix flake
          packages = [
            cargo2nix.packages."${system}".cargo2nix
            pkgs.rust-analyzer-unwrapped
            unstable.cargo-bloat
            unstable.cargo-unused-features
            unstable.cargo-watch
            unstable.cargo-sort
            unstable.cargo-machete
            unstable.cargo-depgraph
            unstable.cargo-limit
            unstable.cargo-dist
            unstable.cargo-release
          ];
        };

      in rec {
        # this is the output (recursive) set (expressed for each system)

        devShells = {
          default = workspaceShell; # nix develop
        };

        # the packages in `nix build .#packages.<system>.<name>`
        packages = {
          # nix build .#json_verify
          # nix build .#packages.x86_64-linux.json_verify
          json_verify = (rustPkgs.workspace.json_verify { }).bin;
          # nix build .#nocommit
          # nix build .#packages.x86_64-linux.nocommit
          nocommit = (rustPkgs.workspace.nocommit { }).bin;
          # nix build .#spm
          # nix build .#packages.x86_64-linux.spm
          spm = (rustPkgs.workspace.spm { }).bin;
          # nix build
          default = packages.json_verify;
        };

        # nix run github:positron-solutions/unixsocks
        # apps = rec {
        #   unixsocks = {
        #     type = "app";
        #     program = "${packages.default}/bin/unixsocks";
        #   };
        #   default = unixsocks;
        # };
      });
}
