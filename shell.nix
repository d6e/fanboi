{ project ? import ./nix {}
}:
let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  ats_rs = (import ./default.nix {}).ats_rs;
in
project.pkgs.mkShell {
  buildInputs = builtins.attrValues project.devTools ++ [ project.pkgs.cargo-edit rust ats_rs ];
  shellHook = ''
    ${project.ci.pre-commit-check.shellHook}
  '';
}
