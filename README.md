# Parser
This parser is heavily inspired by the [mixfix](https://agda.readthedocs.io/en/v2.5.2/language/mixfix-operators.html) operators within Agda, allowing you to define weird and wonderful syntax such as `if true then 3 else 1` where the mixfix definition of the function is:
```
if_then_else_ : (A : Set) -> Boolean -> A -> A -> A
if true then x else _ = x
if false then _ else y = y
```
However, my parser is very limited and does not support mixed types; I don't intend to change this because I don't care.

In its current state the parser should be able to express floating point arithmetic as well as simple two-value boolean algebra.