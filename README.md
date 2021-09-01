# lalrpop-operator-precedence

An example of partially ordered operator precedence using [LALRPOP](https://github.com/lalrpop/lalrpop).

Erroneous expressions without additional parentheses:
```
a & b + c
a << b + 1
```

Well-formed expressions:
```
a * b + c == foo & a
a && (!b || c)
-a + 32 < !a | b
a || b * (a-c) || b ^ d
```

## Operator precedence

- Logical operators: `&&`, `||`
- Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Mathematical operators: `+`, `-`, `*`, `/`, `%`
- Bitwise operators: `&`, `|`, `^`, `<<`, `>>`
- Unary operators: `!`, `-`

unary operators > mathematical/bitwise operators > comparison operators > logical operators

## Precedence of the operators inside the categories

- Logical operators: no mixing without parentheses
- Comparison operators: no chaining
- Mathematical operators: `*`, `/`, `%` > `+`, `-`
- Bitwise operators: no mixing. no chaining for `<<` and `>>`

## Credits

- [Operator precedence is broken](https://www.foonathan.net/2017/07/operator-precedence/)
- [연산자 우선순위](https://hut.mearie.org/operator-precedence/)
- [Operator Precedence: We can do better](https://blog.adamant-lang.org/2019/operator-precedence/)
