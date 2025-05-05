#![allow(unused)]
/// # Knapsack as an ILP (conceptual encoding)
///
/// The goal is to maximize the total value of picked items,
/// subject to the total weight not exceeding the budget `W`.
///
/// We represent:
/// - `x[i]`: decision variable (0 or 1) indicating whether item `i` is picked.
/// - Each item `i` has weight `w[i]` and value `v[i]`.
/// - The constraint is Σ (w[i] * x[i]) ≤ W.
///
/// This doctest sets up a toy instance of the knapsack problem
/// with a small number of items (and small weight budget)
/// to illustrate how we might encode the constraints
/// and check feasibility.
///
/// ```
/// // We define a simple struct to hold the item data.
/// #[derive(Debug, Clone, Copy)]
/// struct Item {
///     value: i32,
///     weight: i32,
/// }
///
/// // This function will build the decision variables and check feasibility.
/// fn setup_knapsack_ilp(items: &[Item], budget: i32) -> (Vec<i32>, i32) {
///     // x[i] is the decision variable for item i, ∈ {0,1}.
///     // For demonstration, let's do a trivial "pick if possible" strategy,
///     // just to illustrate storing x[i]. We do NOT guarantee optimality here.
///
///     let mut x = vec![0; items.len()];
///     let mut total_weight = 0;
///     let mut total_value = 0;
///
///     for (i, item) in items.iter().enumerate() {
///         // If picking doesn't exceed budget, pick it (x[i]=1).
///         if total_weight + item.weight <= budget {
///             x[i] = 1;
///             total_weight += item.weight;
///             total_value += item.value;
///         }
///     }
///
///     // Return the chosen decision variables and the total value.
///     (x, total_value)
/// }
///
/// fn main() {
///     // Let's define some items and a budget.
///     let items = vec![
///         Item { value: 10, weight: 6 },
///         Item { value:  7, weight: 5 },
///         Item { value: 12, weight: 7 },
///         Item { value:  5, weight: 3 },
///     ];
///     let budget = 15;
///
///     // Set up the "ILP" by building the x[i] decision variables.
///     let (x, total_value) = setup_knapsack_ilp(&items, budget);
///
///     // Print out the decision variables to confirm the picks.
///     // Note: In a real ILP solver, we would systematically find the optimal solution.
///     println!("Decision variables x[i]: {:?}", x);
///     println!("Computed total value: {}", total_value);
///
///     // Let's just ensure the chosen items do not exceed budget.
///     let total_weight: i32 = items.iter().zip(&x).map(|(it, &picked)| it.weight * picked).sum();
///     assert!(total_weight <= budget, "Exceeded budget");
/// }
/// ```
pub fn knapsack_example_for_doctest() {}

/// # Verifying the decision variable (indicator variable) concept
///
/// Suppose we define `x_n` as:
/// ```text
/// x_n = if item_n_is_picked { 1 } else { 0 };
/// ```
///
/// This doctest just confirms the idea of "indicator" variables.
/// It demonstrates toggling an integer based on a boolean condition.
///
/// ```
/// fn indicator_var_example(picked: bool) -> i32 {
///     // x_n = 1 if picked, else 0
///     if picked { 1 } else { 0 }
/// }
///
/// fn main() {
///     assert_eq!(indicator_var_example(true), 1);
///     assert_eq!(indicator_var_example(false), 0);
/// }
/// ```
pub fn indicator_var_for_doctest() {}

/// # Illustrating the weight-sum constraint
///
/// For the knapsack ILP, the core constraint is:
/// ```text
/// (w_1 * x_1) + (w_2 * x_2) + ... + (w_n * x_n) <= W
/// ```
///
/// We'll show how one might accumulate weights and check that it is ≤ W.
///
/// ```
/// #[derive(Debug, Clone, Copy)]
/// struct Item {
///     w: i32, // weight
/// }
///
/// fn weight_sum_constraint_example(items: &[Item], x: &[i32], budget: i32) -> bool {
///     let total_weight: i32 = items.iter()
///         .zip(x.iter())
///         .map(|(item, &picked)| item.w * picked)
///         .sum();
///     total_weight <= budget
/// }
///
/// fn main() {
///     let items = vec![
///         Item { w: 5 },
///         Item { w: 6 },
///         Item { w: 2 },
///     ];
///
///     // Suppose we pick the 1st and 3rd items => x = [1, 0, 1].
///     let x = vec![1, 0, 1];
///     let budget = 10;
///
///     // Check the constraint
///     let feasible = weight_sum_constraint_example(&items, &x, budget);
///     assert_eq!(feasible, true, "Should be feasible, total weight = 7");
///
///     // Another pick: x = [1, 1, 1], total weight = 13 => not feasible.
///     let x2 = vec![1, 1, 1];
///     let feasible2 = weight_sum_constraint_example(&items, &x2, budget);
///     assert!(!feasible2, "Should not be feasible, total weight = 13");
/// }
/// ```
pub fn weight_sum_constraint_for_doctest() {}

/// # SAT as ILP
///
/// In the notes, we see that each proposition `p_i` can be turned into a binary variable
/// `x_i ∈ {0,1}`. A clause can be turned into constraints ensuring at least one literal is 1,
/// etc. We'll just illustrate in code how you might represent the truth assignment
/// as a vector of 0/1 variables, then see if a set of clauses is satisfied.
///
/// ```
/// /// A "literal" can be positive (just x_i) or negative (¬x_i).
/// /// We'll encode negative as `is_negated = true`.
/// #[derive(Debug, Clone, Copy)]
/// struct Literal {
///     index: usize,     // which variable x_i is referred to
///     is_negated: bool, // whether it is x_i or ¬x_i
/// }
///
/// /// A clause is a disjunction (OR) of literals.
/// struct Clause {
///     literals: Vec<Literal>,
/// }
///
/// /// Given an assignment vector x (0 or 1), evaluate a single literal.
/// fn eval_literal(lit: &Literal, x: &[i32]) -> bool {
///     let val = x[lit.index];
///     if lit.is_negated {
///         val == 0 // ¬x_i is satisfied if x_i == 0
///     } else {
///         val == 1 // x_i is satisfied if x_i == 1
///     }
/// }
///
/// /// Evaluate the entire clause by checking if *any* literal is satisfied.
/// fn eval_clause(clause: &Clause, x: &[i32]) -> bool {
///     clause.literals.iter().any(|lit| eval_literal(lit, x))
/// }
///
/// /// Evaluate a formula (conjunction of clauses).
/// fn eval_formula(clauses: &[Clause], x: &[i32]) -> bool {
///     // All clauses must be satisfied.
///     clauses.iter().all(|c| eval_clause(c, x))
/// }
///
/// fn main() {
///     // Suppose we have a formula in 3-CNF, e.g., (x_0 ∨ ¬x_1) ∧ (x_1 ∨ x_2).
///
///     // We'll encode them in a vector of clauses:
///     let clause1 = Clause {
///         literals: vec![
///             Literal { index: 0, is_negated: false }, // x_0
///             Literal { index: 1, is_negated: true  }, // ¬x_1
///         ]
///     };
///     let clause2 = Clause {
///         literals: vec![
///             Literal { index: 1, is_negated: false }, // x_1
///             Literal { index: 2, is_negated: false }, // x_2
///         ]
///     };
///
///     let clauses = vec![clause1, clause2];
///
///     // We'll try a couple of assignments to x = [x_0, x_1, x_2].
///
///     // x = [1, 0, 1] => x_0 = 1, x_1 = 0, x_2 = 1
///     //   Clause1: (x_0 ∨ ¬x_1) => (1 ∨ ¬0) => (1 ∨ 1) => true
///     //   Clause2: (x_1 ∨ x_2) => (0 ∨ 1) => true
///     // => formula satisfied
///     let x_good = vec![1, 0, 1];
///     assert!(eval_formula(&clauses, &x_good));
///
///     // x = [0, 1, 0] => x_0 = 0, x_1 = 1, x_2 = 0
///     //   Clause1: (x_0 ∨ ¬x_1) => (0 ∨ ¬1) => (0 ∨ 0) => false
///     // => formula NOT satisfied
///     let x_bad = vec![0, 1, 0];
///     assert!(!eval_formula(&clauses, &x_bad));
/// }
/// ```
pub fn sat_as_ilp_for_doctest() {}

/// # Observing the "Integrality Gap"
///
/// Although we won't solve or demonstrate the gap fully here,
/// we illustrate the difference between integer constraints and
/// relaxing them to real-valued constraints. (In practice you'd
/// use an ILP solver to see the difference.)
///
/// ```
/// fn integrality_gap_demo() {
///     // Pretend we have a fractional pick of 0.5 on an item whose weight=10, value=50.
///     // As a real LP, that might be feasible. But in ILP, x must be 0 or 1.
///     // So the integer solution might pick 0 units or 1 unit.
///     // The "gap" is the difference in the objective between
///     // the fractional solution vs. the best integer solution.
///     
///     let fractional_pick = 0.5; // Not valid in ILP, only in LP relaxation.
///     let item_value = 50;
///     let item_weight = 10;
///     let budget = 5; // forced to choose partial if it were a real relaxation
///
///     // For demonstration, let's compute the "value" if partial picks were allowed:
///     let partial_value = fractional_pick * (item_value as f64);
///     
///     // For an ILP with a budget less than the item weight, we can't pick it at all,
///     // so best integer pick => x=0 => 0 value. Fractional => 25.0
///     let gap = partial_value - 0.0;
///     
///     // This gap illustrates that relaxing integrality constraints can yield
///     // a "better" objective value in the LP sense, but that solution is invalid
///     // when we restore the integrality constraint.
///
///     assert!((gap - 25.0).abs() < f64::EPSILON);
/// }
///
/// fn main() {
///     integrality_gap_demo();
/// }
/// ```
pub fn integrality_gap_for_doctest() {}
