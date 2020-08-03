{ project ? import ./nix {} }:
let
  pkgs = project.pkgs;
  lua_library = pkgs.stdenv.mkDerivation rec {
    name = pkgs.lua5_3.name;
    version = pkgs.lua5_3.version;
    src = pkgs.lua5_3;
    builder = builtins.toFile "builder.sh" "
    source $stdenv/setup
    mkdir -p $out/include/lua5.3
    cp -r $src/* $out/
    cp -r $src/include/* $out/include/lua5.3/
  ";
  };
in
{
  ats = pkgs.stdenv.mkDerivation rec {
    name = "ats";
    version = "0.2.0";
    src = (import ./nix/sources.nix {}).ats;
    coreutils = pkgs.coreutils;
    buildInputs = [ pkgs.lua5_3 pkgs.lua5_3_compat ];
    lua = lua_library;
    builder = ./builder.sh;
    meta = with pkgs.stdenv.lib; {
      description =
        "Active Fan Thermal Service tool, to Control Processor Temperature on RockPro64 Single Board Computer.";
      maintainers = { email = "nixpkgs@d6e.io"; github = "d6e"; githubId = 2476055; name = "Danielle"; };
      homepage = "https://github.com/tuxd3v/ats";
    };
  };
  ci = (import ./nix {}).ci;
}
