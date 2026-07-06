set unstable
set lists

# self

self := 'just --justfile' ++ justfile()

# shelly

__shelly(cmd) := if cmd =~ '^\s*$' {
    cmd
} else if cmd =~ '^>.*' {
    shell(replace_regex(cmd, '^>\s*', ''))
} else if cmd =~ '^@.*' {
    replace_regex(cmd, '^@\s*', '')
} else {
    '( echo ' + quote(BLACK + '>' + NORMAL + ' ' + BOLD + cmd + NORMAL) + '; ' + cmd + '; )'
}
shelly(cmd) := __shelly(join_list(cmd))

# variables

var(name) := shelly(('>SWISS_SIGNATURE=' + quote(file_stem(justfile()))) ++ 'swiss-var get' ++ quote(name))
let(name, value) := shelly(('>SWISS_SIGNATURE=' + quote(file_stem(justfile()))) ++ 'swiss-var set' ++ quote(name) ++ quote(value))
