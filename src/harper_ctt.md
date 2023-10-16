> # _Computational Type Theory_
>
> Bob Harper, OPLSS 2018

## [Lecture 1][1]

<iframe width="560" height="315" src="https://www.youtube.com/embed/LE0SSLizYUI?si=YSw_ufskYlinuv9M" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

### Type theory from a computational perspective

---

References

- Martin-Löf, _Constructive Mathematics and Computer Programming_
- Constable et al., _NuPRL System & Semantics_

The plan is to develop type theory starting with computation, and developing a
_theory of truth_ based on proofs. This contrasts with formalisms (e.g. theories
of proofs, formal derivation, etc.). Not just playing the Coq/Agda/Lean video
game!

### Start with a programming language

---

The language has a deterministic semantics (via its transition system).

- Forms of expression $E$
- Two judgement forms
  - $E\ val$ meaning $E$ is fully evaluated
  - $E \mapsto E'$ meaning one simplification of $E$
- Derived notion $E \Downarrow E_{\circ}$ meaning $E \mapsto ^{*} E_{\circ}
  \
  val$

For example: $if(E_1;E_2)(E)$

### Operational semantics for binary decision diagrams

---

### $\frac{E\ \longmapsto\ E'}{if(E_1;E_2)(E)\ \longmapsto\ if(E_1;E_2)(E')}$

$\frac{}{\begin{cases} if(E_1;E_2)(true)\ \longmapsto\ E_1 \\
if(E_1;E_2)(false)\
\longmapsto\ E_2 \end{cases}}$

### Principle: types are specifications of program _behavior_

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

---

$seq(n)\ type$ when $n\ \in\ Nat$

$n:\ Nat\ \gg\ seq\ type$

That is, a family of types indexed by a type.

Another way to phrase it, which emphasizes the _indexing_ of a sequence by $n\
\in\ Nat$ is

$\forall n\ \exists\ seq(n)$ where $seq(n) \doteq [0..n-1]$

In NuPRL notation: $f\ \in\ n: Nat \rightarrow\ Seq(n)$, which in the literature
may also be represented as $\Pi n: Nat\ Seq(n)$

### Functionality

---

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

---

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

### A computational semantics

---

$A \doteq A'$ means $ \begin{cases} A \Downarrow A_{\circ} \\ A' \Downarrow
A'_{\circ} \\ A_{\circ} \doteq_{\circ} A'_{\circ} \end{cases}$

$A_{\circ}$ and $A'_{\circ}$ are equal _type-values_, or what Martin-Löf called
_canonical types_.

$M \doteq M' \in A$, where $A$ is a type. A type, again, is a program which
evaluates to equal type-values.

### A few core ideas

---

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

---

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
- $\blacksquare$

## [Lecture 2][2]

<iframe width="560" height="315" src="https://www.youtube.com/embed/1U4w0159-Ls?si=tmRfnko1dvSBDNx4" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

### A small recap of lecture 1

---

A type system consists of $A \doteq A$ with $A\ type\ iff\ A \doteq A$

$M \doteq M$ with $M \in A\ iff\ M \doteq M \in A$

Which is both _symmetric_ and _transitive_, that is

If $A \doteq A'$ and $M \doteq M' \in A$ then $M \doteq M' \in A$

**Hypotheticals express _functionality_**

$a: A \gg B\ type$ means $B$ is a family of types that depends functionally on
$a: A$.

$M \doteq M' \in A$ implies $B[M/a] \doteq B[M'/a]$

$a: A \gg N \in B$ means $B$ is a family of _elements_. A family of elements is
a mapping.

$M \doteq M' \in A$ implies $N[M/a] \doteq N[M'/a] \in B[M/a] \doteq B[M'/a]$

Similarly for $B \doteq B'$, $N \doteq N' \in B$

All of the above constitutes a type system defined in terms of evaluation (hence
computational) using "certain constructions".

### There exists a type system containing a type `bool`

---

- $Bool \doteq Bool$

- $M \doteq M' \in Bool$ iff either ${M \Downarrow true \choose M' \Downarrow
  true }$ or ${M \Downarrow false \choose M' \Downarrow false}$

**Fact**

If $a: Bool \gg B\ type$ and $M_1 \in B[true/a]$ and $M_2 \in B[false/a]$ and $M
\in Bool$

Then $if(M_1;M_2)(M) \in B[M/a]$

**Proof**

- Either $M \Downarrow true$

  - $\therefore M \doteq true \in Bool$ by head expansion (reverse execution)

- _or_ $M \Downarrow false$

  - $\therefore M \doteq false \in Bool$ by head expansion (reverse execution)

- $\blacksquare$

**An example**

Given $M_1 \in B[true/a]$

- $if(M_1;M_2)(M) \doteq if(M_1;M_2)(true) \doteq M_1 \in B[true/a] \doteq
  B[M/a]$ ✅

This can be generalized by _Shannon expansion_.

If $a: Bool \gg P \in B$ then $P[M/a] \doteq if(P[true/a];P[false/a])(M)$

Which can be seen as a "pivot on $M$". Binary decision diagrams are created by
choosing pivots in a way that minimizes the size of conditionals.

- The given facts look like definitions in a formal type system, and this is
  intentional.
- Semantics define what _is true_, and formalisms that follow are a pale
  approximation which are useful for impl.
- These "little lemmas" are the method of _logical relations_; type-indexed
  information determines what equality means.

### There exists a type system containing a type `Nat`

---

$Nat \doteq Nat$

$M \doteq M' \in Nat$ is the extremal (strongest/smallest) statement s.t.

- Either ${M \Downarrow 0 \choose M' \Downarrow 0}$
- Or ${M \Downarrow succ(N) \choose M' \Downarrow succ(N')}$ $N \doteq N' \in
  Nat$
- The extremal clause provides a morally-equivalent notion of an induction
  principle.

**Consider the following**

Given some assumptions

### $\frac{}{0\ val}$

### $\frac{}{succ(M)\ val}$

We can define the Y-combinator

$fix(a.succ(a)) \mapsto \omega \coloneqq succ(fix\ a.succ(a))$

$\omega \in CoNat$ is thus the _greatest_ solution to the specification, $\omega
\notin Nat$

We may now define a recursor $R$.

$R \coloneqq rec(M_{\circ};a,b.M_1)(M) \mapsto rec(M_{\circ};a,b.M_1)(M')$ if $M
\mapsto M'$

- $R(0) \mapsto M_{\circ}$
- $R(succ(m)) \mapsto M_1[M, R(M)/a,b]$

**Fact**

- $a: Nat \gg B\ type$
- $M_{\circ} \in B[0/a]$
- $a: Nat, b: B \gg M_1 \in B[succ(a)/a]$

If $M \in Nat$ then $R(M) \in B[M/a]$

**Proof**

Case for $0$:

- $M \Downarrow 0$
- $M \doteq 0 \in Nat$
- $M_{\circ} \in B[0/a] \doteq B[m/a]$
- $R(M) \doteq R(0) \doteq M_{\circ}$
- $R(M) \in B[M/a]$
- $\blacksquare$

Case for $succ(N)$

- $M \Downarrow succ(N)$
- Given an inductive hypothesis $R(N) \in B[N/a]$ ... (the proof is unfinished)

These ($Bool$,$Nat$) are representative examples of _inductive types_ (least
extremal clauses that satisfy some conditions). They are not representative
examples of _coinductive_ types (most extremal clauses that imply/are consistent
with some conditions).

### There exists a type system containing a type `Prod`

---

- $(A_1 \times A_2) \doteq (A'_1 \times A'_2)$ iff $A_1 \doteq A'_1$ and $A_2
  \doteq A'_2$

- $M \doteq M' \in (A_1 \times A_2)$ iff $ \begin{cases} M \Downarrow \langle
  M_1, M_2 \rangle \\ M' \Downarrow \langle M'_1, M'_2 \rangle \end{cases}$

where $M_n \doteq M'_n \in A_n$, that is, that the values for $M_n$s are
evaluated under the conditions in $A_n$

**Fact**

If $M \in A_1 \times A_2$ then $M.0 \in A_1$ and $M.1 \in A_2$ (one could sub
other notation here, e.g. `fst`, etc.)

where

## $\frac{M \mapsto M'}{M.i_1 \mapsto M.i_2}$ $\frac{}{\langle M_1, M_2 \rangle.i \mapsto M_i}\ $

and $(i=1,2)$

A note about logical relations as a tactic: membership in product types is
defined in terms of equality in each of the component types. Constituent types
are, in a sense, already given, and then we speak about the composite types.

Going further...

If $M_1 \in A_1$, then $\langle M_1, M_2 \rangle.1 \doteq M_1 \in A_1$, which
has _no requirement_ on $M_2$.

Recall that by head-expansion (a.k.a reverse-execution) $\langle M_1, M_2
\rangle.1 \mapsto M_1$.

This may seem like a "technical anomaly", but is an important insight into how
computational type theory relies on _specifications_ as opposed to a grammar for
writing down well-formed things according to syntactic rules that impose
additional requirements beyond those described by this fact. _Formalisms are
about obeying protocols_.

- Completing the proof of fact w.r.t. `Prod` from the assumptions (in similar
  vein to the earlier examples) is left as an exercise.

### There exists a type system containing a type `Fn`

---

- $A_1 \rightarrow A_2 \doteq A'_1 \rightarrow A'_2$ iff $A_1 \doteq A'_1$
- $A_2 \doteq A'_2$

- $M \doteq M' \in A_1 \rightarrow A_2$ iff $M \Downarrow \lambda a.M_2$
- $M'\Downarrow \lambda a.M'_2$

where

$a: A \gg M_2 \doteq M\_2 \in A_2$

Given some assumptions

### $\frac{}{\lambda a.M\ val}$

### $\frac{M \mapsto M'}{ap(M,M1) \mapsto ap(M', M_1)}$

### $\frac{}{ap(\lambda a.M_2,M1) \mapsto M_2[M_1/a]}$

The latter is a kind of $\beta$-reduction, or at least a well-specified
_reduction strategy_.

**Fact**

If $M \in A_1 \rightarrow A_2$ and $M_1 \in A_1$ then $ap(M, M_1) \in A_2$.

We can translate this fact into a usual type-theoretic notation like so

### $\frac{\Gamma \vdash M: A_1 \rightarrow A_2\ \Gamma \vdash M_1: A_1}{\Gamma \vdash ap(M,M_1): A_2}$

and the inductive definition is the "protocol" or "syntax" of the aforementioned
fact.

_Observe_: what is the quantifier complexity of $M \doteq M' \in Nat \rightarrow
Nat$?

In an informal sense, one can say

$\forall M_1 \doteq M'_1 \in Nat\ \exists P_1 \doteq P'_1 \in Nat$ such that
$ap(M,M_1) \doteq ap(M',M'_1) \in Nat$

This is the reason that starting with the formal syntax is inadequate, because
an induction rule like

### $\frac{}{\Gamma \vdash M \equiv M': Nat \rightarrow Nat}$

is a derivation tree that results in a quantifier complexity of "there exists
something". But $\forall\ \exists$ _cannot be captured by $\exists$ alone_!

**Fact**

If $M,M' \in A_1 \rightarrow A_2$ and $a: A_1 \gg ap(M,a) \doteq ap(M',a)$ then
$M \doteq M' \in A_1 \rightarrow A_2$.

One may call this "function extensionality"

- Completing the proof of facts w.r.t. `Fn` from the assumptions (in similar
  vein to the earlier examples) is left as an exercise.

This is profound: One _cannot axiomatize equality_ in $Nat \rightarrow Nat$ by a
deeper understanding of Gödel's theorem.

### There exists a type system containing a type `Dependent Prod`

---

- $a: A_1 \times A_2 \doteq a: A'_1 \times A'_2$ iff $A_1 \doteq A'_1$
- $a: A_1 \gg A_2 \doteq A'_2$
- $M \doteq M' \in (A_1 \times A_2)$ iff $ \begin{cases} M \Downarrow \langle
  M_1, M_2 \rangle \\ M' \Downarrow \langle M'_1, M'_2 \rangle \end{cases}$

where

$M_1 \doteq M'_1 \in A_1$ and, different from `Prod`, $M_2 \doteq M'_2 \in
A_2[M_1/a] \doteq A_2[M'_1/a]$

which encodes the _dependency_ between $A_1$ and $A_2$.

### There exists a type system containing a type `Dependent Fn`

- $a: A_1 \rightarrow A_2 \doteq a: A'_1 \rightarrow A'_2$ iff $A_1 \doteq A'_1$
- $a: A_1 \gg A_2 \doteq A'_2$

From the definition, it's easy enough to look back at the definitions / initial
transitions for `Fn` and subsitute the following, as we did for `Prod`

$a: A_1 \gg M_2 \doteq M'_2 \in A_2(a)$

The meaning of the change is

- If $M_1 \doteq M'_1 \in A_1$ then $M_2[M_1/a] \doteq M_2[M'_1/a] \in
  A_2[M_1/a] \doteq A_2[M'_1/a]$

**Fact**

1. If $M \in a: A_1 \times A_2$ then $M.0 \in A_1$ and $M.1 \in A_2[M.0/a]$

2. If $M \in a: A_1 \rightarrow A_2$ and $M_1 \in A_1$ then $ap(M,M_1) \in
   A_2[M_1/a]$

- Completing the proof of fact w.r.t. `Dependent Fn` from the assumptions (in
  similar vein to the earlier examples) is left as an exercise.

### In summary, so far

This is a development of a simple, _inherently computational_ dependent type
system

- $\tau \coloneqq Bool\ |\ Nat\ |\ a_1:A_1 \times A_2\ |\ a_1:A_1 \rightarrow
  A_2$

which can be used to prove an earlier claim

- $if(17;true)(M) \in if(Nat,Bool)(M)$

## [Lecture 3][3]

<iframe width="560" height="315" src="https://www.youtube.com/embed/GzPMZ6RsihU?si=DRNb42qoCfiUqsg6" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

## [Lecture 4][4]

<iframe width="560" height="315" src="https://www.youtube.com/embed/pfOQ97iCIsk?si=3r0neFvVaQiNiC6h" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

## [Lecture 5][5]

<iframe width="560" height="315" src="https://www.youtube.com/embed/RhDuRmg-SdA?si=IAec8tT6aNKafyqy" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

[1]: https://youtu.be/LE0SSLizYUI?si=nFIdHEnJ2zHYoIcw
[2]: https://youtu.be/1U4w0159-Ls?si=tmRfnko1dvSBDNx4
[3]: https://youtu.be/GzPMZ6RsihU?si=j0CPmIx0Guf0cQq6
[4]: https://youtu.be/pfOQ97iCIsk?si=BDnawE4phRxcntnY
[5]: https://youtu.be/RhDuRmg-SdA?si=PjaOP-qiifzFye_H
