# wrong working directory workaround
set no-cd

# user-defined functions and lists are currently unstable
set unstable
set lists

# a performant callback function
self := f'just --justfile {{ justfile() }}'
