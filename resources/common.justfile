# wrong working directory workaround
set no-cd

# user-defined functions are currently unstable
set unstable

# a performant callback function
self := f'just --justfile {{ justfile() }}'

# abort execution tree and print the exit message
[private, no-exit-message]
abort message:
    @echo '{{ RED + BOLD + "error" + NORMAL }}: {{ BOLD + message + NORMAL }}'
    @exit 1
