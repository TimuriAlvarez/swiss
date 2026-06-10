# iterate over all elements in a list
_list-iter list-contents el-name el-action:
    @ echo '{{ list-contents }}' | awk -F: '{for({{ el-name }}=1; {{ el-name }}<=NF; {{ el-name }}++) {{ el-action }}}'
list-iter := _self('_list-iter')

# print all elements from a list
_list-print list-contents list-heading list-prefix='    ':
    @ echo -n -e '{{ list-heading }}'
    @ {{ list-iter }} '{{ list-contents }}' 'el' 'print "{{ list-prefix }}"$el'
list-print := _self('_list-print')

# act if list contains a specified element
_list-if-in list-contents el-value then else='_nop' *args:
    @ {{ if-nz }} "`{{ list-iter }} '{{ list-contents }}' 'el' 'if($el==\"{{ el-value }}\") print $el'`" '{{ then }}' '{{ else }}' '{{ args }}'
list-if-in := _self('_list-if-in')

# act if list does not contain a specified element
_list-if-no list-contents el-value then else='_nop' *args:
    @ {{ if-ez }} "`{{ list-iter }} '{{ list-contents }}' 'el' 'if($el==\"{{ el-value }}\") print $el'`" '{{ then }}' '{{ else }}' '{{ args }}'
list-if-no := _self('_list-if-no')
