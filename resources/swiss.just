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
    shell(trim(replace_regex(cmd, '^>\s+', '')))
} else if cmd =~ '^@.*' {
    trim(replace_regex(cmd, '^@\s+', ''))
} else {
    '( echo ' + quote(BLACK + '>' + NORMAL + ' ' + BOLD + cmd + NORMAL) + '; ' + cmd + '; )'
}
shelly(cmd) := __shelly(join_list(cmd))
