# set working directory
set working-directory := x'$PWD'

# enable list support
set unstable
set lists

# newline
[private]
NEWLINE := "\n"

# boolean aliases
[private]
true := 'true'
[private]
false := []

# self
[private]
self := 'just' ++ '--justfile' ++ justfile()

# fold list
fold(value) := if value && value != '' { value }

# split string into lines
lines(s) := split(s, NEWLINE)

# literal value
lv(value) := replace(replace_regex(replace_regex(join_list(value),
    '^'+ "'" +'?\n', ''),
    '\n$', ''),
    '\', '\\')

# require
require(name) := fold(replace_regex(which(name) || '', '^.*/(?<name>.*)$', '$name'))

# signature
[private]
__swiss_signature := file_stem(justfile())

# state
state_directory() := if env('XDG_CONFIG_HOME', []) { env('XDG_CONFIG_HOME') } else { home_directory() / '.local' / 'state' }
state_file() := state_directory() / 'swiss' / __swiss_signature / uuid()

# shelly
__swiss_shelly(s, stdout, failure) := if s =~ '^>.*' {
    shell('( ' + replace_regex(s, '^>[[:blank:]]*', '') + '; ) > ' + quote(stdout) + ' || swiss-edit ' + quote(failure) + ' ' + quote('\A\z') + ' ' + quote('')) \
    && [] || if !path_exists(failure) { trim_end_match(read(stdout), NEWLINE) }
} else if s =~ '^@.*' {
    replace_regex(s, '^@[[:blank:]]*', '')
} else {
    '( echo ' + quote('> ' + BOLD + s + NORMAL) + '; ' + s + '; )'
}
shelly(value) := __swiss_shelly(join_list(value), state_file(), state_file())

# variables
__swiss_vars_cmd(persist, signature, name, query, args) := shelly('>' ++ 'swiss-vars' ++ quote((if bool(persist) { '--persist' }) ++ signature ++ query ++ name ++ args))
__swiss_vars_set(persist, signature, name, value) := __swiss_vars_cmd(persist, signature, name, 'set', value)
__swiss_vars_nxt(persist, signature, name, index) := if fold(index) { __swiss_vars_get(persist, signature, name, index) }
__swiss_vars_get(persist, signature, name, index) := (if index {
    __swiss_vars_cmd(persist, signature, name, 'get', index)
}) ++ __swiss_vars_nxt(persist, signature, name, __swiss_vars_cmd(persist, signature, name, 'next', index))

# hook
[private]
__swiss_hook_variable := []
is_hook() := __swiss_hook_variable == []
__swiss_hook(variable, book, entry) := (shelly('> swiss-hook' ++ quote(__swiss_signature ++ variable ++ book ++ entry)) \
    || error('swiss-hook failed')) \
    && __swiss_vars_get(false, __swiss_signature, variable, [])
