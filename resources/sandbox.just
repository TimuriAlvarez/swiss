foo-1(a1='*') := a1
foo-2(a1, a2='*') := a1 ++ a2
foo-3(a1, a2, a3='*') := a1 ++ a2 ++ a3
bar-1(a1='*', a2='*') := foo-2(a1, a2)
baz-1(a1, a2='*', a3='*') := foo-3(a1, a2, a3)

[private]
test-1:
    {{ assert(foo-1() == foo-1()) }}
    {{ assert(foo-2('a') == foo-2('a')) }}
    {{ assert(foo-3('a', 'b') == foo-3('a', 'b')) }}

[private]
test-2:
    {{ assert(foo-1('*') == foo-1()) }}
    {{ assert(foo-2('a', '*') == foo-2('a')) }}
    {{ assert(foo-3('a', 'b', '*') == foo-3('a', 'b')) }}

[private]
test-3:
    {{ assert(foo-1('''*''') == foo-1()) }}
    {{ assert(foo-2('''a''', """*""") == foo-2('a')) }}
    {{ assert(foo-3('''a''', """b""", '*') == foo-3('a', 'b')) }}

[private]
test-4:
    {{ assert(foo-2('', '*') == foo-2('')) }}
    {{ assert(foo-2(',', '*') == foo-2(',')) }}
    {{ assert(foo-2(',,', '*') == foo-2(',,')) }}
    {{ assert(foo-3('', '', '*') == foo-3('', '')) }}
    {{ assert(foo-3(',', '', '*') == foo-3(',', '')) }}
    {{ assert(foo-3(',', ',', '*') == foo-3(',', ',')) }}

[private]
test-5:

[default]
_: test-1 test-2 test-3 test-4 test-5
