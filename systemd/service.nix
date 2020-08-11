{ config, lib, pkgs, ... }:
let
  fanboi = (pkgs.callPackage ../default.nix {}).fanboi;
  cfg = config.services.fanboi;
in
{
  options.services.fanboi.enable = lib.mkEnableOption "Fanboi fan PID controller";
  options.services.fanboi.extraArgs = lib.mkOption {
    type = lib.types.listOf lib.types.str;
    default = [ "" ];
    example = [ "--debug" ];
  };

  config = lib.mkIf cfg.enable {
    systemd.services.fanboi = {
      description = "A linux PID controller for fans.";
      wantedBy = [ "multi-user.target" ];
      startLimitIntervalSec = 0;
      serviceConfig = {
        Type = "simple";
        ExecStart = "${fanboi}/bin/fanboi ${lib.concatStringsSep " " cfg.extraArgs}";
        Restart = "always";
        RestartSec = 8;
        User = "root";
      };
    };
  };
}
