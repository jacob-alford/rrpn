{
  description = "A rust dev shell for rrpn";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , flake-utils
    , nixpkgs
    ,
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [ cargo rustc rust-analyzer git rustfmt nodejs_22 dioxus-cli llvmPackages.bintools wasm-bindgen-cli ];
      };
    });
}
