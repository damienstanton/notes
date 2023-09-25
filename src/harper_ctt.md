# Based on Bob Harper's OPLSS 2018 five-session seminar, _Computational Type Theory_

## [Lecture 1][1]

<iframe width="560" height="315" src="https://www.youtube.com/embed/LE0SSLizYUI?si=YSw_ufskYlinuv9M" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

### Type theory from a computational perspective

References

- Martin-Löf, _Constructive Mathematics and Computer Programming_
- Constable et al., _NuPRL System & Semantics_

The plan is to develop type theory starting with computation, and developing a
_theory of truth_ based on proofs. This contrasts with formalisms (e.g. theories
of proofs, formal derivation, etc.). Not just playing the Coq/Agda/Lean video
game!

### Start with a programming language

The language has a deterministic semantics (via its transition system).

- Forms of expression $E$
- Two judgement forms
  - $E\ val$ meaning $E$ is fully evaluated
  - $E \mapsto E'$ meaning one simplification of $E$
- Derived notion $E \Downarrow E_{\circ}$ meaning $E \mapsto ^{*} E_{\circ}
  \
  val$

For example: $if(E_1;E_2)(E)$

### Operational semantics for binary decision diagrams follow.

### $\frac{E\ \longmapsto\ E'}{if(E_1;E_2)(E)\ \longmapsto\ if(E_1;E_2)(E')}$

$\frac{}{\begin{cases} if(E_1;E_2)(true)\ \longmapsto\ E_1 \\
if(E_1;E_2)(false)\
\longmapsto\ E_2 \end{cases}}$

### Principle: Types are specifications of program _behavior_

Judgements are _expressions of knowledge_, in the intuitionistic sense
(Brouwer), based on the premise that mathematics is a human activity, thus
finite, and that the only way to constrain facts about infinite infinities
(Gödel, Russell?) is via algorithms.

E.g. there is no way to convey infinitely many utterly disconnected facts about
$Nat$ other than to have a uniformity expressed as an algorithm. We agree on
such uniformity solely because of the fundamental faculties of the human mind
w.r.t. computation.

$M$ and $A$ here are _programs_, and are behavioral, not structural.
$\begin{cases} A\
type \\ M\ \in\ A \end{cases}$

For example: $if(17;\_)(true)\ \in\ Nat$, why? Because the simplification step
entails $17\ \in Nat$

Further, $if(Nat;Bool)(M)$ is a type _precisely when_ $M\ \in\ Bool$

This applies to type expressions as well, e.g.

$if(17;true)(M)\ \in\ if(Nat;Bool)(M)$

This helps explain why a deterministic operational semantics is required,
because it is _the same $M$_ in the simplification step. Types/specifications
**are programs**.

### Type-indexed families of types a.k.a. dependent types

$seq(n)\ type$ when $n\ \in\ Nat$

$n:\ Nat\ \gg\ seq\ type$

That is, a family of types indexed by a type.

Another way to phrase it, which emphasizes the _indexing_ of a sequence by $n\
\in\ Nat$ is

$\forall n\ \exists\ seq(n)$ where $seq(n) \doteq [0..n-1]$

In NuPRL notation: $f\ \in\ n: Nat \rightarrow\ Seq(n)$, which in the literature
may also be represented as $\Pi n: Nat\ Seq(n)$

### Functionality

Families (of types, of elements) must respect **equality of indices**. _So what
is equality?_

Trivially, $seq(2+2)$ is "the same as" $seq(4)$, or

$seq(if(17;18)(M))$ is "the same as" $if(seq(17);seq(18))(M)$

Which can be clarified with a slight change in notation, substituting $a: Bool$
for $M$

As a type structure gets richer, _when two things are equal is a property of the
type they inhabit_, and it can be arbitrarily complicated. The situation about
what is true has a much higher -- in fact -- **unbounded** quantifier of
complexity, whereas the question of what is written down in any formalism is
always relentlessly recursively enumerable. This means that formalisms can never
approach equality-based truth! This standpoint is validated by Gödel's theorem.

Indeed, the entire endeavour of higher-dimensional type theory arises from
investigating when two types are the same.

### Judgements

When $A$ is a type, $A \doteq A'$, exact equality of types

When $M \in A$, $M \doteq M' \in A$, exact equality of elements

Thus, equal indices deterministically produce equal results (this is also why a
deterministic operational semantics is a necessity). A term for this is
**equisatisfaction**.

A simple example is

$2 \doteq 4 \in Nat$ cannot be true...

However, $2 \doteq 4 \in Nat/2\ (evens)$ _is true!_. It always depends on type
inhabitants.

This is in contrast to traditional formalized type theory, where equality is
axiomatized as being type-independent, which is a fallacy.

Another possible notation for $M \doteq M' \in A$ is $M \doteq_{A} M'$.

### A Computational Semantics

$A \doteq A'$ means $ \begin{cases} A \Downarrow A_{\circ} \\ A' \Downarrow
A'_{\circ} \\ A_{\circ} \doteq_{\circ} A'_{\circ} \end{cases}$

$A_{\circ}$ and $A'_{\circ}$ are equal _type-values_, or what Martin-Löf called
_canonical types_.

$M \doteq M' \in A$, where $A$ is a type. A type, again, is a program which
evaluates to equal type-values.

### A few core ideas

Given that $A \Downarrow A_{\circ}$ $A_{\circ} \doteq_{\circ} A_{\circ}$,

we can say

$\begin{cases} M \Downarrow M_{\circ}\ &\&\ \\ M' \Downarrow M'_{\circ}\
&\&\
\\ M_{\circ} \doteq_{\circ} M'_{\circ} \in A_{\circ} \end{cases}$

$a: A \gg B \doteq B'$ means

$if\ (M \doteq M' \in A)\ then\ (B[M/a] \doteq B'[M'/a])$

This induces a certain implication about type indexing, called _functionality_:

Check that $ \begin{cases} a: A \gg B\ \text{where}\ B &\doteq\ B \\ M' &\doteq
M' \in A\ \\ \text{implies}\ B[M/a] &\doteq B[M'/a] \end{cases} $

A final example in the general case

$a: A \gg N \doteq N'$ means

$if\ M \doteq M' \in A\ then\ (N[M/a] \doteq N'[M'a] \in B[M/a])$

assuming that $a \gg B \doteq B$

### A specific example by deriving the Boolean type

1. $Bool \doteq_{\circ} Bool$, i.e. $Bool$ _names_ a type-value (hence the $\
   _{\circ}$).

2. What type does it name? The membership relation for canonical elements is

$M_{\circ} \doteq M'_{\circ} \in_{\circ} Bool$ is the _strongest_ relation $\R$
(though some literature calls this _least_, the extremal nature is what is
important)

such that $\R \subseteq (Exp\ \times Exp) \begin{cases} (true\ \doteq_{\circ}\
true\ \in Bool)\ i.e.\ true\
\in_{\circ} Bool\\ (false\ \doteq_{\circ}\ false\ \in Bool)\ i.e.\ false\
\in_{\circ} Bool \end{cases} $

The extremal clause is that

1. The stated conditions hold.
2. Nothing else!

### A proposition/fact/claim

- If $M \in Bool$ and $A type$ and $M_{1} \in A$ and $M_{2} \in A$, then
  $if(M_{1};M_{2})(M) \in A$

**Proof**

- $\_ \in Bool$ is given by a universal property, the _least containing_ $
  \begin{cases} true\ \in Bool \\ false\ \in Bool \end{cases} $
- Fix some type $A$, $M_{1} \in A$, $M_{2} \in A$.
- If $M \in Bool$ then $if(M_{1};M_{2})(M) \in A$
- Thus, $M \in Bool$ means $M \Downarrow M_{\circ}$ and either $\begin{cases}
  M_{0} &\doteq\ true \\ M_{0} &\doteq\ false \end{cases}$
- It suffices to show that $\begin{cases} if(M_{1};M_{2})(true)\in A \\
  if(M_{1};M_{2})(false)\in A \end{cases}$
- $if$ evaluates its principal arguments (via a transition step). Typing is
  closed under "head-expansion" or alternatively "reverse execution".
- $∎$

[1]: https://youtu.be/LE0SSLizYUI?si=nFIdHEnJ2zHYoIcw
