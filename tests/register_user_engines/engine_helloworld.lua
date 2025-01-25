-- A test scan engine that detects "Hello World"
-- It should return true when "Hello World" appears as a substring.

function engine_helloworld(payload)
    if string.find(payload, "Hello World") ~= nil then
        return true
    end
    return false
end
