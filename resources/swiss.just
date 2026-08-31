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
    shell(replace_regex(cmd, '^>\s*', ''))
} else if cmd =~ '^@.*' {
    replace_regex(cmd, '^@\s*', '')
} else {
    '( echo ' + quote('>' + ' ' + BOLD + cmd + NORMAL) + '; ' + cmd + '; )'
}
shelly(cmd) := __shelly(join_list(cmd))

# variables

__signature := file_stem(justfile())
__idx(name, signature, index) := shelly('> swiss-vars' ++ quote(signature) ++ quote(name) ++ 'next' ++ quote(index))
set(name, value, signature=__signature) := shelly('> swiss-vars' ++ quote(signature) ++ quote(name) ++ 'set' ++ quote(value))
var(name, signature=__signature, index=[]) := (if index {
    shelly('> swiss-vars' ++ quote(signature) ++ quote(name) ++ 'get' ++ quote(index))
}) ++ (if __idx(name, signature, index) != '' {
    var(name, signature, __idx(name, signature, index))
})
