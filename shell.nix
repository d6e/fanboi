{ project ? import ./nix {}
}:
let
  sources = import ./nix/sources.nix;
  rust_ = import ./nix/rust.nix { inherit sources; };
  rust = rust_.override {
    extensions = [ "rust-src" "rust-analysis" ];
  };
  pkgs = import sources.nixpkgs {};
  fanboi = (import ./default.nix {}).fanboi;
in
project.pkgs.mkShell {
  buildInputs = builtins.attrValues project.devTools ++ [
    project.pkgs.cargo-edit
    rust
    fanboi
    pkgs.openssl
    pkgs.pkg-config
    pkgs.nasm
    pkgs.rustup
    pkgs.cmake
    pkgs.zlib
  ];
  shellHook = ''
    ${project.ci.pre-commit-check.shellHook}
  '';
}
