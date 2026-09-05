a1 := []
a2 := ''
a3 := '--hello--'
a4 := ['a', 'b', 'c']
a5 := justfile()

[private]
local-test:
    {{ assert(var(v1) == a1 ++ a1) }}
    {{ assert(var(v2) == a2 ++ a2) }}
    {{ assert(var(v3) == a3 ++ a3) }}
    {{ assert(var(v4) == a4 ++ a4) }}
    {{ assert(var(v5) == a5 ++ a5) }}

[private]
local-1: && local-test
    v1 := a1 ++ a1
    v2 := a2 ++ a2
    v3 := a3 ++ a3
    v4 := a4 ++ a4
    v5 := a5 ++ a5

[private]
local-2: && local-test
    v1 := a1
    v2 := a2
    v3 := a3
    v4 := a4
    v5 := a5
    v1 ++:= a1
    v2 ++:= a2
    v3 ++:= a3
    v4 ++:= a4
    v5 ++:= a5

[private]
global-1:
    sandbox::v1 := a1
    {{ assert(var(sandbox::v1) == a1) }}
    sandbox::v1 := a2 ++ a2
    {{ assert(var(sandbox::v1) == a2 ++ a2) }}
    sandbox::v1 := a3
    sandbox::v1 ++:= a3
    {{ assert(var(sandbox::v1) == a3 ++ a3) }}

[private]
global-2:
    {{ assert(var(sandbox::v2) == [] || var(sandbox::v2) == '*') }}
    sandbox::v2 := if !var(sandbox::v2) { '*' }

[private]
global-3:
    {{ assert(var(sandbox::v3) == []) }}

[default]
test: local-1 local-2 global-1 global-2 global-3
