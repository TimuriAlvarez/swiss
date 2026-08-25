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
    shell(trim(replace_regex(cmd, '^>', '')))
} else if cmd =~ '^@.*' {
    trim(replace_regex(cmd, '^@', ''))
} else {
    '( echo ' + quote(BLACK + '>' + NORMAL + ' ' + BOLD + cmd + NORMAL) + '; ' + cmd + '; )'
}
shelly(cmd) := __shelly(trim(join_list(cmd)))

# variables

var(name) := shelly(('>SWISS_SIGNATURE=' + quote(file_stem(justfile()))) ++ 'swiss-var get' ++ quote(lv(name)))
let(name, value) := shelly(('>SWISS_SIGNATURE=' + quote(file_stem(justfile()))) ++ 'swiss-var set' ++ quote(lv(name)) ++ quote(lv(value)))
