# Mixfix Expressions

This repository contains the source code to my first ever [Rust](https://www.rust-lang.org/) project.

## Inspiration
This parser is inspired by the [mixfix](https://agda.readthedocs.io/en/v2.5.2/language/mixfix-operators.html) operators used within [Agda](https://en.wikipedia.org/wiki/Agda_(programming_language)).

This feature lets you define where functions accept arguments.

For example, if you define a function `plus`, maybe you would want it to read as `a plus b` instead of `plus a b`; this feature lets you do that! by using `_`s you can tell the compiler where the arguments are supposed to be, so our function `plus` becomes `_plus_`, and then we can use it infix like we want.

Another example would be the ternary operator `?:` in most C-style languages: you could create a function called `_?_:_` which accepts three arguments, and then use it as `condition ? a : b` since the `_`s tell you where it expects arguments to be.

## Limitations
My parser does not support mixed types, and I don't feel like changing this because the project was an exercise.