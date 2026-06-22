literal-trim(s) := replace_regex(replace_regex(s, '^\n', ''), '\n$', '')
literal-backslash(s) := replace(s, '\', '\\\\')
literal-quotes(s) := replace(replace(replace(s, '(")', "'"), '"', '\"'), '`', '\`')
literal-shell(s) := replace(s, '$', '\$')
literal(prefix, s, suffix) := '"' + prefix + literal-shell(literal-quotes(literal-backslash(literal-trim(s)))) + suffix + '"'
