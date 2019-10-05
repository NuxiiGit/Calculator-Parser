# Mixfix Expressions

This repository contains the source code to my first ever [Rust](https://www.rust-lang.org/) project.

## Inspiration
This parser is inspired by the [mixfix](https://agda.readthedocs.io/en/v2.5.2/language/mixfix-operators.html) operators used within [Agda](https://en.wikipedia.org/wiki/Agda_(programming_language)). A feature which lets you define where functions should accept arguments.

For example, if you define a function `plus`, it would usually read as `plus a b`, where `a` and `b` are parameters. However, what if you wanted it to read as `a plus b`? This can be done by renaming `plus` to `_plus_`, where each `_` tells you where the first and second parameters should be.

Another example would be the ternary operator `?:` found in most C-style languages. By creating a function named `_?_:_` which accepts three parameters, we can simulate the ternary operator: `condition ? a : b`.

Additionally, Haskell has similar built-in constructs which act as syntactic sugar for repetitive tasks. These include `if..then..else..` and `case..of..`.

## Limitations
My parser does not support mixed types, and I don't feel like changing this because the project was an exercise.