# Parser
This parser is inspired by the [mixfix](https://agda.readthedocs.io/en/v2.5.2/language/mixfix-operators.html) operators used within the Agda programming language.

## Explaination
This feature gives you a lot of flexibility and allows you define custom syntax; they are defined by inserting underscores `_` in place of argument parameters.

For example: with the ternary operator `condition ? ifTrue : ifFalse`, its mixfix definition would be `_?_:_` since the underscores show the positions of each argument. Another example would be bracket operators such as functions, or the [inner product](http://mathworld.wolfram.com/InnerProduct.html) of two vectors `v` and `w`: `<v, w>`, whose mixfix definition would be `<_,_>`.

## Limitations
My parser does not support mixed types. I don't feel like changing this because the project was an exercise. Therefore, the parser should be able to express any fixed-type operations, including: floating point arithmetic, boolean algebras, and vector spaces.
