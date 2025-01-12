-- Default rcfile for sscani.
--
-- Overrides some startup behaviors and configuration of scanni.
-- To regenerate this file, re-run generate_rcfile().
--
-- See help('rcfile') for help on configuration.


-- Uncomment to disable the splash text on startup.
-- sscani.config.show_splash = false


-- Uncomment to edit the splash text displayed on startup.
-- For more advanced control, override function sscani.splash().

-- sscani.config.splash = string.format([[
--
-- @@@ sscani v%s - Interactive REPL for sscan
-- @@@ Authors: %s (%s)
-- @@@ License: %s
-- @@@
-- @@@ Enter any valid Lua, terminated by ';'.
-- @@@ Use help(); for help, exit(); to exit.
--
-- ]],about.version, about.authors, about.repository, about.license_spdx)


-- Uncomment to configure the prompt text.
-- For more advanced control, override function sscani.prompt()
-- sscani.config.prompt = 'sscan> '

-- Uncomment to configure the prompt continuation text.
-- For more advanced control, override function sscani.prompt_continue()
-- sscani.config.prompt_continue = '   ... '
