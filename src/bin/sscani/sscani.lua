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

-- Configures the default splash message displayed on startup.
-- For more advanced control, override function sscani.splash().
sscani.config.splash = string.format([[

@@@ sscani v%s - Interactive REPL for sscan
@@@ Authors: %s (%s)
@@@ License: %s
@@@
@@@ Enter any valid Lua, terminated by ';'.
@@@ Use help(); for help, exit(); to exit.

]],about.version, about.authors, about.repository, about.license_spdx)

-- Configures the default prompt string.
-- For more advanced control, override function sscani.prompt()
sscani.config.prompt = 'sscan> '

-- Configures the default continuation string.
-- For more advanced control, override function sscani.prompt_continue()
sscani.config.prompt_continue = '   ... '

-- Function to use for the prompt.
-- Users can override this prompt if desired.
function sscani.prompt()
    io.write(sscani.config.prompt)
    io.flush()
end

-- Function to print continuation lines.
-- Users can override this prompt if desired.
function sscani.prompt_continue()
    io.write(sscani.config.prompt_continue)
    io.flush()
end

-- Prints a splash message with version info and basic help.
-- Users can override this function if desired.
function sscani.splash()
    io.write(sscani.config.splash)
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
