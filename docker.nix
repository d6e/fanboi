let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs {};
  fanboi = (import ./default.nix { inherit sources pkgs; }).fanboi;
  name = "d6e/fanboi";
  tag = "latest";

in
pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ fanboi ];

  config = {
    Cmd = [ "/bin/fanboi" ];
    WorkingDir = "/";
  };
}
