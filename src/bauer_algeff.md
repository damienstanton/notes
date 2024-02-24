> # _Algebraic Effects and Handlers_
>
> Andrej Bauer

## Basics

Take the operation symbol for $plus$:

$plus: 1 \times \Z^2 \rightarrow \Z$

`3 + (4 + 5)`

```haskell
plus(b:2 if b = 0 then 3
         else plus(c:2 if c = 0 then 4
                       else 5))
```

$Op \colon A^n \rightarrow A$

where $n$ is the _arity_, or number of distinct effects

---

$Op \colon B \times A^n \rightarrow A$

where $B$ is the _parameter type_, a monad

---

$Op \colon B \times A^C \rightarrow A$

where $C$ is an unbounded type; a delimited continuation
