{ project ? import ./nix {}
}:
let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
in
project.pkgs.mkShell {
  buildInputs = builtins.attrValues project.devTools ++ [ rust ];
  shellHook = ''
    ${project.ci.pre-commit-check.shellHook}
  '';
}
