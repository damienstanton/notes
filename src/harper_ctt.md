## Based on Bob Harper's OPLSS 2018 five-session seminar, _Computational Type Theory_

### [Lecture 1][1]

<iframe width="560" height="315" src="https://www.youtube.com/embed/LE0SSLizYUI?si=YSw_ufskYlinuv9M" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

#### Type theory from a computational perspective

References

- Martin-Löf, _Constructive Mathematics and Computer Programming_
- Constable et al., _NuPRL System & Semantics_

The plan is to develop type theory starting with computation, and developing a
_theory of truth_ based on proofs. This contrasts with formalisms (e.g. theories
of proofs, formal derivation, etc.). Not just playing the Coq/Agda/Lean video
game!

#### Start with a programming language

The language has a deterministic semantics (via its transition system).

- Forms of expression $E$
- Two judgement forms
  - $E\ val$ meaning $E$ is fully evaluated
  - $E \mapsto E'$ meaning one simplification of $E$
- Derived notion $E \Downarrow E_{\circ}$ meaning $E \mapsto ^{*} E_{\circ}
  \
  val$

For example: $if(E_1;E_2)(E)$

Operational semantics for binary decision diagrams follow.

## $\frac{E\ \longmapsto\ E'}{if(E_1;E_2)(E)\ \longmapsto\ if(E_1;E_2)(E')}$

$\frac{}{\begin{cases} if(E_1;E_2)(true)\ \longmapsto\ E_1 \\
if(E_1;E_2)(false)\
\longmapsto\ E_2 \end{cases}}$

#### Principle: Types are specifications of program _behavior_

Judgements are _expressions of knowledge_, in the intuitionistic sense
(Brouwer), based on the premise that mathematics is a human activity, thus
finite, and that the only way to constrain facts about infinite infinities
(Gödel, Russel?) is via algorithms.

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

#### Type-indexed families of types a.k.a. dependent types

$seq(n)\ type$ when $n\ \in\ Nat$

$n:\ Nat\ \gg\ seq\ type$

That is, a family of types indexed by a type.

Another way to phrase it, which emphasizes the _indexing_ of a sequence by $n\
\in\ Nat$

$\forall n\ \exists\ seq(n)$ where $seq(n) \doteq [0..n-1]$

#### TODO: ~ 31:10, function $f$

[1]: https://youtu.be/LE0SSLizYUI?si=nFIdHEnJ2zHYoIcw
