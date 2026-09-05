set unstable
set lists
set no-cd

# self

self := 'just --justfile' ++ justfile()

# literal value

lv(s) := replace(replace_regex(replace_regex(join_list(s),
    '^'+ "'" +'?\n', ''),
    '\n$', ''),
    '\', '\\')

# shelly

__shelly(cmd) := if cmd =~ '^>.*' {
    shell(replace_regex(cmd, '^>[[:blank:]]*', ''))
} else if cmd =~ '^@.*' {
    replace_regex(cmd, '^@[[:blank:]]*', '')
} else {
    '( echo ' + quote('> ' + BOLD + cmd + NORMAL) + '; ' + cmd + '; )'
}
shelly(cmd) := __shelly(join_list(cmd))

# variables

__set(signature, name, value) := shelly('> swiss-vars' ++ quote(signature) ++ quote(name) ++ 'set' ++ quote(value))
__nxt(signature, name, index) := if index != '' { __get(signature, name, index) }
__get(signature, name, index) := (if index {
    shelly('> swiss-vars' ++ quote(signature) ++ quote(name) ++ 'get' ++ quote(index))
}) ++ __nxt(signature, name, shelly('> swiss-vars' ++ quote(signature) ++ quote(name) ++ 'next' ++ quote(index)))
