-- sscani.lua - Helper library for the sscani REPL.
--
-- This helper library is specific to sscani, and provides additional
-- functionality for the interactive REPL.

-- All helper methods except exit() will go in this table.
sscani = {}

-- Immediately exits sscani. Alias for os.exit(0).
function exit()
    os.exit(0)
end

-- Prints a splash message with version info and basic help.
function sscani.splash()
    print("\n@@@ sscani v" .. about.version .. " - Interactive REPL for sscan")
    print("@@@ Authors: " .. about.authors)
    print("@@@ License: " .. about.license_spdx .. "\n@@@")
    print("@@@ Enter any valid Lua, terminated by ';'.")
    print("@@@ Use exit(); to exit.\n")
end


-- Print the splash message.
sscani.splash()
