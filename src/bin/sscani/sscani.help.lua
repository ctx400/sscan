-- help.lua - The sscani help subsystem.
--
-- This helper library provides the help() function, which can either
-- display generic help, or detailed information if a topic is selected.

-- This table stores all help information.
sscani_help = {}

-- Individual help topics are stored here.
sscani_help.topics = {}

-- Displayed if no topic is provided.
sscani_help.generic = string.format([[

sscani is an interactive %s REPL for sscan.

The REPL accepts single- or multi-line chunks of Lua code, with the
final line terminated by a semicolon. The REPL then evaluates the
chunk. If the chunk returns a value, sscani will attempt to display it.

GLOBAL FUNCTIONS
================
exit()          - Quits sscani. Alias to os.exit(0).
help(['topic']) - Prints either generic help, or help on 'topic'.
license()       - Prints open-source license information.
version()       - Prints sscan version information.

For information on a specific topic, use help('topic');
To list all available topics, use help('topics');

]], _VERSION)

-- Help for the `about` table.
sscani_help.topics.about = string.format([[

TABLE `about`
============

The `about.*` table contains basic version and license information about
the sscan project. All of these keys should be of a printable type.

For example, to access the current version of sscan, use:

  sscan> about.version

]])

-- Additional context for the help('topics') command.
sscani_help.about_topics = [[
The following topics are available. Use help('topic') to select one:

]]

-- Sort all the topics.
table.sort(sscani_help.topics)

-- Returns a comma-delimited string of help topics.
function sscani_help.list_topics()
    local topics = {}
    for key, _ in pairs(sscani_help.topics)
    do
        table.insert(topics, key)
    end
    return table.concat(topics, ',')
end

-- Prints either generic help or a topic.
function help(topic)
    -- Print generic help if no topic was provided.
    if topic == nil then
        io.write(sscani_help.generic)
        io.flush()
        return
    end

    -- Return an error if 'topic' is not a string
    if type(topic) ~= 'string' then
        io.write('Error: if `topic` is provided, it must be a string.\n\n')
        io.flush()
        return
    end

    -- Print a list of topics if requested.
    if topic == 'topics' then
        io.write(sscani_help.about_topics)
        io.write(sscani_help.list_topics() .. '\n\n')
        io.flush()
        return
    end

    -- Otherwise, print help on the specified topic
    local help_topic = sscani_help.topics[topic]
    if topic == nil then
        io.write("Couldn't find that topic. List all with help('topics').\n\n")
        io.flush()
        return
    end

    io.write(help_topic)
    io.flush()
end
