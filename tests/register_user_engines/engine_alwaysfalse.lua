-- A test userscript scan engine that always returns false.
-- It should never show up in scan results.

function engine_alwaysfalse(payload)
    return false
end
