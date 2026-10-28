set swiss-vars := [v1, v2, v3]
set swiss-vars := [v4]
set swiss-vars := v5

a1 := []
a2 := ''
a3 := '--hello--'
a4 := ['a', 'b', 'c']
a5 := justfile()

[private]
@test-vars:
    {{ assert(var(v1) == a1 ++ a1) }}
    {{ assert(var(v2) == a2 ++ a2) }}
    {{ assert(var(v3) == a3 ++ a3) }}
    {{ assert(var(v4) == a4 ++ a4) }}
    {{ assert(var(v5) == a5 ++ a5) }}

[private]
@test-1: && test-vars
    {{ v1 := { a1 ++ a1 } }}
    {{ v2 := { a2 ++ a2 } }}
    {{ v3 := { a3 ++ a3 } }}
    {{ v4 := { a4 ++ a4 } }}
    {{ v5 := { a5 ++ a5 } }}

[private]
@test-2: && test-vars
    {{ v1 := { a1 } }}
    {{ v2 := { a2 } }}
    {{ v3 := { a3 } }}
    {{ v4 := { a4 } }}
    {{ v5 := { a5 } }}
    {{ v1 ++:= { a1 } }}
    {{ v2 ++:= { a2 } }}
    {{ v3 ++:= { a3 } }}
    {{ v4 ++:= { a4 } }}
    {{ v5 ++:= { a5 } }}

[default]
test: test-1 test-2
