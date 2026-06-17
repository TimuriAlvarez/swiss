# wrong working directory fix
set no-cd

# user-defined functions and lists are currently unstable
set unstable
set lists

# a performant callback function
self := f'just --justfile {{ justfile() }}'
_self(recipe) := self + ' ' + recipe

# no operation instruction
_nop *args:
nop := _self('_nop')
