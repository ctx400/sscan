-- sscan Example Userscript
-- "Scan all subfiles of a folder for a matching pattern."
--
-- This is an example sscan userscript. As an example, it only intends
-- to demonstrate the basics of how to write a userscript. It does not
-- try to be perfect or be 100% error-proof.
--
-- To print usage information, run this userscript without arguments:
--   sscan run scan_folder.lua
--
-- Usage is also defined directly below this comment.

-- Print usage information.
local usage = function()
    io.write('\nUSAGE: sscan run scan_folder.lua <pattern> <path/to/folder>\n')
    io.write('  Scan all subfiles for <pattern>, a Lua regex pattern.\n\n')
end

-- Check if the correct number of arguments was passed.
if #arg ~= 2 then usage() return 1 end
local scan_pattern = arg[1]
local scan_folder = arg[2]

-- Register a simple scan engine that matches a user-provided pattern.
user_engines:register(
    'match_pattern',
    function(p) return string.find(p, scan_pattern) ~= nil end)

-- Check that the provided folder path exists.
if not fs:test(scan_folder) and scan_folder.type == 'directory' then
    local err_msg = string.format(
        'Error: expected directory, got %s\n', scan_folder.type)
    io.write(err_msg)
    return 1
end

-- Queue all files under scan_folder/ for scanning
local entries = fs:walk(scan_folder)
for _,entry in ipairs(entries)
do
    if entry.type == 'file' then queue:add_file(entry.path) end
end

-- Print how many files were queued.
io.write(string.format('There are %d files in the scan queue.\n', #queue))

-- Run the scan
io.write(string.format('Starting scan for pattern: %s\n', scan_pattern))
local results = scanmgr:scan()

-- Print how many results were found.
io.write(string.format('Scan found %d matching files.\n', #results))

-- Print each result.
for _,result in ipairs(results)
do
    local result = string.format(
        'Engine %q matched file %q (%q)\n',
        result.engine, result.item.name, result.item.path)
    io.write(result)
end
