v1 := []
v2 := ''
v3 := '*'
v4 := ['a', 'b', 'c']
v5 := justfile()

local a1 := v1 ++ v1
local a2 := v2 ++ v2
local a3 := v3 ++ v3
local a4 := v4 ++ v4
local a5 := v5 ++ v5

[private]
test-local:
    {{ assert(var(a1) == v1 ++ v1) }}
    {{ assert(var(a2) == v2 ++ v2) }}
    {{ assert(var(a3) == v3 ++ v3) }}
    {{ assert(var(a4) == v4 ++ v4) }}
    {{ assert(var(a5) == v5 ++ v5) }}

[private]
@test-1:
    {{ self }} test-local
    a1 := v1 ++ v1
    a2 := v2 ++ v2
    a3 := v3 ++ v3
    a4 := v4 ++ v4
    a5 := v5 ++ v5
    {{ self }} test-local
    a1 := v1
    a2 := v2
    a3 := v3
    a4 := v4
    a5 := v5
    a1 ++:= v1
    a2 ++:= v2
    a3 ++:= v3
    a4 ++:= v4
    a5 ++:= v5
    {{ self }} test-local

v6 := '--hello--'
local a6 := v6

[private]
test-2:
    a6 := v6
    {{ assert(var(a6) == v6) }}
    echo {{ quote(var(a6)) }}

[default]
test: test-1 test-2
