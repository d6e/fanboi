let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs {};
  ats_rs = (import ./default.nix { inherit sources pkgs; }).ats_rs;

  name = "d6e/ats_rs";
  tag = "latest";

in
pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ ats_rs ];

  config = {
    Cmd = [ "/bin/ats_rs" ];
    WorkingDir = "/";
  };
}
