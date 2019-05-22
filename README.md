# Parser
This parser is heavily inspired by the [mixfix](https://agda.readthedocs.io/en/v2.5.2/language/mixfix-operators.html) operators within Agda, allowing you to define weird and wonderful syntax such as `if true then 3 else 1` or `list [ 3 ]` where the mixfix definition of these functions are:
```
if_then_else_ : {A : Set} -> Bool -> A -> A -> A
if true then x else _ = x
if false then _ else y = y
```
and
```
_[_] : {A : Set} -> List A -> Nat -> Option A
nil [ _ ] = none
(cons x _) [ zero ] = some x
(cons _ xs) [ suc n ] = xs [ n ]
```
However, my parser is very limited and does not support mixed types and I don't intend to change this.

In its current state the parser should be able to express floating point arithmetic as well as simple two-value boolean algebra.