# wrong working directory fix
set no-cd

# user-defined functions are currently unstable
set unstable

# a performant callback function
self := f'just --justfile {{ justfile() }}'
_self(recipe) := f'{{ self }} {{ recipe }}'

# a default recipe for all books
[default]
_:
    @ {{ self }} --list

# no operation instruction
_nop *args:
nop := _self('_nop')
