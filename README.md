# Parser
This parser is inspired by the [mixfix](https://agda.readthedocs.io/en/v2.5.2/language/mixfix-operators.html) operators used within the Agda programming language. These operators are defined by using underscores `_` is place of argument parameters. For example, with the ternary operator `condition ? ifTrue : ifFalse`, its mixfix definition would be `_?_:_` since underscores are in place of each argument. Another example would be bracket operators such as functions or the [inner product](http://mathworld.wolfram.com/InnerProduct.html) of two vectors `v` and `w`: `<v, w>`, whose mixfix definition would be `<_,_>`.

This feature gives you a lot of flexibility and allows you define custom syntax. However, my parser does not support mixed types; I don't feel like changing that because this was an exercise.

The parser should be able to express any fixed-type operations, such as: floating point arithmetic, boolean algebras, and vector space.