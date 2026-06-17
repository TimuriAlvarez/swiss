# if-op
_if-op a op b then else='_nop' *args:
    @if [ '{{ a }}' {{ op }} '{{ b }}' ]; then {{ self }} {{ then }} {{ quote(args) }}; else {{ self }} {{ else }} {{ quote(args) }}; fi
if-op := _self('_if-op')

# if equal
_if-eq a b then else='_nop' *args: (_if-op a '==' b then else args)
if-eq := _self('_if-eq')

# if not equal
_if-ne a b then else='_nop' *args: (_if-op a '!=' b then else args)
if-ne := _self('_if-ne')

# if equals to empty
_if-ez value then else='_nop' *args: (_if-eq value '' then else args)
if-ez := _self('_if-ez')

# if not equals to empty
_if-nz value then else='_nop' *args: (_if-ne value '' then else args)
if-nz := _self('_if-nz')
