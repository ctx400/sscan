-- A test scan engine that always returns true.
-- It should show up under results for every scan operation.

function engine_alwaystrue(payload)
    return true
end
