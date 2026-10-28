# cd fix
set no-cd

# enable list support
set unstable
set lists

# self
self := 'just' ++ '--justfile' ++ justfile()

# boolean aliases
true := 'true'
false := []

# fold list
fold(list) := if list && list != '' { list }

# literal value
lv(s) := replace(replace_regex(replace_regex(join_list(s),
    '^'+ "'" +'?\n', ''),
    '\n$', ''),
    '\', '\\')

# shelly
__swiss_shelly(cmd) := if cmd =~ '^>.*' {
    shell(replace_regex(cmd, '^>[[:blank:]]*', ''))
} else if cmd =~ '^@.*' {
    replace_regex(cmd, '^@[[:blank:]]*', '')
} else {
    '( echo ' + quote('> ' + BOLD + cmd + NORMAL) + '; ' + cmd + '; )'
}
shelly(cmd) := __swiss_shelly(join_list(cmd))

# variables
__swiss_vars_cmd(persist, signature, name, query, args) := shelly('>' ++ 'swiss-vars' ++ quote((if bool(persist) { '--persist' }) ++ signature ++ query ++ name ++ args))
__swiss_vars_set(persist, signature, name, value) := __swiss_vars_cmd(persist, signature, name, 'set', value)
__swiss_vars_nxt(persist, signature, name, index) := if fold(index) { __swiss_vars_get(persist, signature, name, index) }
__swiss_vars_get(persist, signature, name, index) := (if index {
    __swiss_vars_cmd(persist, signature, name, 'get', index)
}) ++ __swiss_vars_nxt(persist, signature, name, __swiss_vars_cmd(persist, signature, name, 'next', index))
