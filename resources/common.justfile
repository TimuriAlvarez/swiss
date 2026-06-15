# wrong working directory fix
set no-cd

# user-defined functions are currently unstable
set unstable

# a performant callback function
self := f'just --justfile {{ justfile() }}'
_self(recipe) := self + ' ' + recipe

# no operation instruction
_nop *args:
nop := _self('_nop')
