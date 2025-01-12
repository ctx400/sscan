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

-- Tries to get the user's home directory.
-- Returns nil if a home directory cannot be found.
function sscani.try_get_home()
    local home = os.getenv('HOME')
    if home ~= nil then return home end

    home = os.getenv('USERPROFILE')
    return home
end

-- Checks for an rcfile at several locations and executes it if found.
function sscani.check_for_rcfile()
    local home = sscani.try_get_home()
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
            test:close()
            dofile(path)
            return
        end
    end
end

-- Generates a default rcfile at $HOME/.sscani.rc.lua
function sscani.mkrcfile()
    local home = sscani.try_get_home()
    if home == nil then
        io.write('Could not generate rcfile: failed to find $HOME or $USERPROFILE.\n')
        io.flush()
        return
    end

    local rcfile = io.open(home .. '/.sscani.rc.lua', 'w')
    if rcfile == nil then
        io.write('Could not generate rcfile: access denied for ~/.sscani.rc.lua.\n')
        io.flush()
        return
    end

    rcfile:write(sscani.rc_default)
    rcfile:close()

    io.write('Wrote default rcfile to ~/.sscani.rc.lua\n')
    io.write("See help('rcfile') for help on configuration.\n\n")
    io.flush()
end


-- @@@@@@@@@@@@@@@
-- Startup Routine
-- @@@@@@@@@@@@@@@

-- Run user rcfile discovery
sscani.check_for_rcfile()

-- Print the splash message.
if sscani.config.show_splash then sscani.splash() end
