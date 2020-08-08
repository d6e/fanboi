{ project ? import ./nix {} }:
let
  pkgs = project.pkgs;
  ats_src = (import ./nix/sources.nix {}).ats;
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
  ats = pkgs.lua53Packages.buildLuarocksPackage rec {
    pname = "ats";
    version = "0.2.0";
    patches = [
      ./ats.patch # fixes the config path
    ];
    knownRockspec = ./ats-master-0.rockspec;
    src = ats_src;
    propagatedBuildInputs = with pkgs; [ lua_library ];
    extraVariables = ''
      LUA_LIBDIR="${pkgs.lua}/lib";
      ATS_CONF="${ats_src}/etc/ats.conf";
    '';
    meta = with pkgs.stdenv.lib; {
      description =
        "Active Fan Thermal Service tool, to Control Processor Temperature on RockPro64 Single Board Computer.";
      hydraPlatforms = stdenv.lib.platforms.linux;
      maintainers = { email = "nixpkgs@d6e.io"; github = "d6e"; githubId = 2476055; name = "Danielle"; };
      homepage = "https://github.com/tuxd3v/ats";
    };
  };
  ci = (import ./nix {}).ci;
}
