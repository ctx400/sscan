-- Test if the CounterAPI and its help are accessible from Lua.
-- Otherwise this script will return an error to Rust.

-- Can we see generic help?
help()

-- Can we see a list of topics?
help:topics()

-- Can we see our counter API's help
help 'counter'

-- Can we access the counter's value?
assert(counter.value == 0)

-- Can we increment the counter?
counter:inc()
assert(counter.value == 1)

-- Can we reset the counter?
counter:reset()
assert(counter.value == 0)
