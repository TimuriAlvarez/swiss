# wrong working directory workaround
set no-cd

# user-defined functions are currently unstable
set unstable

# a performant callback function
self := f'just --justfile {{ justfile() }}'
