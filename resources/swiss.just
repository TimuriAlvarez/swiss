# cd fix
set no-cd

# enable list support
set unstable
set lists

# newline
[private]
NEWLINE := "\n"

# self
[private]
self := 'just' ++ '--justfile' ++ justfile()

# boolean aliases
[private]
true := 'true'
[private]
false := []

# fold list
fold(value) := if value && value != '' { value }

# split string into lines
lines(s) := split(s, NEWLINE)

# literal value
lv(value, replacement='$lv') := replace_regex(replace(replace_regex(replace_regex(join_list(value),
    '^'+ "'" +'?\n', ''),
    '\n$', ''),
    '\', '\\'),
    '(?<lv>.*)', replacement)

# shelly
__swiss_shelly(s) := if s =~ '^>.*' {
    shell(replace_regex(s, '^>[[:blank:]]*', ''))
} else if s =~ '^@.*' {
    replace_regex(s, '^@[[:blank:]]*', '')
} else {
    '( echo ' + quote('> ' + BOLD + s + NORMAL) + '; ' + s + '; )'
}
shelly(value) := __swiss_shelly(join_list(value))

# variables
__swiss_vars_cmd(persist, signature, name, query, args) := shelly('>' ++ 'swiss-vars' ++ quote((if bool(persist) { '--persist' }) ++ signature ++ query ++ name ++ args))
__swiss_vars_set(persist, signature, name, value) := __swiss_vars_cmd(persist, signature, name, 'set', value)
__swiss_vars_nxt(persist, signature, name, index) := if fold(index) { __swiss_vars_get(persist, signature, name, index) }
__swiss_vars_get(persist, signature, name, index) := (if index {
    __swiss_vars_cmd(persist, signature, name, 'get', index)
}) ++ __swiss_vars_nxt(persist, signature, name, __swiss_vars_cmd(persist, signature, name, 'next', index))
