set unstable
set lists

self := 'just --justfile' ++ justfile()

# shell made friendly
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
