set unstable
set lists

# self

self := 'just --justfile' ++ justfile()

# literal

literal(s) := replace(replace_regex(replace_regex(join_list(s),
    '^?'+ "'" +'\n', ''),
    '\n$', ''),
    '\', '\\')
qlv(s) := quote(literal(s))

# shelly

__shelly(cmd) := if cmd =~ '^>.*' {
    shell(trim(replace_regex(cmd, '^>', '')))
} else if cmd =~ '^@.*' {
    trim(replace_regex(cmd, '^@', ''))
} else {
    '( echo ' + quote(BLACK + '>' + NORMAL + ' ' + BOLD + cmd + NORMAL) + '; ' + cmd + '; )'
}
shelly(cmd) := __shelly(trim(join_list(cmd)))

# variables

var(name) := shelly(('>SWISS_SIGNATURE=' + quote(file_stem(justfile()))) ++ 'swiss-var get' ++ qlv(name))
let(name, value) := shelly(('>SWISS_SIGNATURE=' + quote(file_stem(justfile()))) ++ 'swiss-var set' ++ qlv(name) ++ qlv(value))
