rockspec_format	= "1.0"
package		= "ats"
version		= "master-0"
description = {
	summary		= "Active Thermal Service",
	detailed		= [[
		This tool, provide support for Rockpro64, Active thermal Service( Fan Control ).
	]],
	homepage	= "https://github.com/tuxd3v/ats",
	license		= "See License..",
	maintainer	= "tuxd3v <tuxd3v@sapo.pt>"
}
source = {
	url	= "git://github.com/tuxd3v/ats",
	branch	= "master"
}
dependencies = {
	"lua >= 5.3"
}
build = {
	type = "builtin",
	install = {
        lua = {
			["ats"] = "src/ats"
		},
		bin = {
			["ats"] = "src/ats"
		},
		conf = {
			["ats.conf"] = "etc/ats.conf"
		}
    },
	modules = {
		ats = {
			sources = { "src/ats.c" },
			incdirs = { "include" },
			libraries = { "lua" },
		}
	},
	copy_directorties = { "etc" }
}
