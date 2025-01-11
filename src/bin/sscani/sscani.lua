-- sscani.lua - Helper library for the sscani REPL.
--
-- This helper library is specific to sscani, and provides additional
-- functionality for the interactive REPL.

-- All helper methods except exit() and help() will go in this table.
sscani = {}

-- All configuration variables go in this table.
sscani.config = {}

-- Controls whether to show the splash message or not on startup.
sscani.config.show_splash = true

-- Function to use for the prompt.
-- Users can override this prompt if desired.
function sscani.prompt()
    io.write("sscan> ")
    io.flush()
end

-- Function to print continuation lines.
-- Users can override this prompt if desired.
function sscani.prompt_continue()
    io.write("   ... ")
    io.flush()
end

-- Prints a splash message with version info and basic help.
function sscani.splash()
    io.write("\n@@@ sscani v" .. about.version .. " - Interactive REPL for sscan\n")
    io.write("@@@ Authors: " .. about.authors .. "\n")
    io.write("@@@ License: " .. about.license_spdx .. "\n@@@\n")
    io.write("@@@ Enter any valid Lua, terminated by ';'.\n")
    io.write("@@@ Use exit(); to exit.\n\n")
    io.flush()
end

-- Immediately exits sscani. Alias for os.exit(0).
function exit()
    os.exit(0)
end

-- Lists available commands and variables.
function help()
    io.write('GLOBAL FUNCTIONS\n')
    io.write('================\n')
    io.write('  exit()\n')
    io.write('  help()\n')
    io.write('  license()\n')
    io.write('  version()\n\n')

    io.write('VERSION INFORMATION\n')
    io.write('===================\n')
    io.write('Type\tKey\n')
    for key, value in pairs(about)
    do
        io.write(type(value) .. '\tabout.' .. key .. '\n')
    end
    io.write('\n')
    io.flush()
end

-- Checks for an rcfile at several locations and executes it if found.
function sscani.check_for_rcfile()
    local home = os.getenv('HOME')
    if home == nil then return end

    local test_paths = {
        home .. '/.sscanirc',
        home .. '/.sscanirc.lua',
        home .. '/.sscani.rc.lua',
        home .. '/.config/sscanirc',
        home .. '/.config/sscanirc.lua',
        home .. '/.config/sscani.rc.lua'
    }

    for _, path in ipairs(test_paths)
    do
        test = io.open(path, 'r')
        if test ~= nil then
            io.close(test)
            dofile(path)
            return
        end
    end
end

-- Run user rcfile discovery
sscani.check_for_rcfile()

-- Print the splash message.
if sscani.config.show_splash then sscani.splash() end
