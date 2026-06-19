# `if` without `else` requires `set lists`
# list concatenation operator `++` requires `set lists`
set lists

if-exp(expression, then) := if expression { then }
if-eq(a, b, then) := if-exp(a == b, then)
if-ne(a, b, then) := if-exp(a != b, then)
if-ez(value, then) := if-exp(value == '', then)
if-nz(value, then) := if-exp(value != '', then)
if-re(value, re, then) := if-exp(value =~ re, then)
if-nr(value, re, then) := if-exp(value !~ re, then)

quote-1(v1) := quote(v1)
quote-2(v1, v2) := quote(v1) ++ quote(v2)
quote-3(v1, v2, v3) := quote(v1) ++ quote(v2) ++ quote(v3)
quote-4(v1, v2, v3, v4) := quote(v1) ++ quote(v2) ++ quote(v3) ++ quote(v4)
quote-5(v1, v2, v3, v4, v5) := quote(v1) ++ quote(v2) ++ quote(v3) ++ quote(v4) ++ quote(v5)
