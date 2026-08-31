v1 := []
v2 := ''
v3 := '*'
v4 := ['a', 'b', 'c']
v5 := justfile()

local a1 := v1
local a2 := v2
local a3 := v3
local a4 := v4
local a5 := v5

[private]
test-1:
    {{ assert(var(a1) == v1) }}
    {{ assert(var(a2) == v2) }}
    {{ assert(var(a3) == v3) }}
    {{ assert(var(a4) == v4) }}
    {{ assert(var(a5) == v5) }}
    a1 := (v1 ++ v1)
    a2 := (v2 ++ v2)
    a3 := (v3 ++ v3)
    a4 := (v4 ++ v4)
    a5 := (v5 ++ v5)
    {{ assert(var(a1) == v1 ++ v1) }}
    {{ assert(var(a2) == v2 ++ v2) }}
    {{ assert(var(a3) == v3 ++ v3) }}
    {{ assert(var(a4) == v4 ++ v4) }}
    {{ assert(var(a5) == v5 ++ v5) }}

[default]
_: test-1
