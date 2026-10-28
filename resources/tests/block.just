[private]
test-1:
    r#{{
    #}}

[private]
test-2:
    r#{{
    assert(true)
    #}}

[private]
test-3:
    r#{{ REGION BEGIN
    assert(true)
    assert(true)
    assert(true)
    #}} REGION END

[private]
test-4:
    {{ assert(true) }}
    r#{{
    assert(true)
    assert(true)
    assert(true)
    #}}
    {{ assert(true) }}
    r#{{
    assert(true)
    assert(true)
    # comment
    assert(true)
    #}}
    {{ assert(true) }}

[private]
test-5:
    @r#{{
    #}}
    {{ assert(true) }}
    @ r#{{
    # comment
    assert(true)
    #}}

[private]
test-6:
    r#{{#}}

[private]
test-multiline *value:
    {{ if bool(value) {
        'echo true'
    } else {
        'echo false'
    } }}
    @r#{{ REGION multiline BEGIN
    if bool(value) {
        'echo true'
    } else {
        'echo false'
    }
    #}} REGION multiline END

[private]
test-7: (test-multiline true) (test-multiline false)

[default]
test: test-1 test-2 test-3 test-4 test-5 test-6 test-7
