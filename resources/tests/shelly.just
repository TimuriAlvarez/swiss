[private]
@test-1:
    r#{{
    assert(shelly('> echo && i_cannot_be_found') == [])
    #}}

[private]
@test-2:
    r#{{
    assert(shelly('> echo') == shell('echo'))
    assert(shelly('> echo && echo') == shell('echo && echo'))
    assert(shelly('> echo && echo && echo') == shell('echo && echo && echo'))
    assert(shelly('> echo hi && echo hi && echo hi') == shell('echo hi && echo hi && echo hi'))
    #}}

[default]
test: test-1 test-2
