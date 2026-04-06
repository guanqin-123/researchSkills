# Stage: Decision

The **bound evaluation** step of the research BaB process. Assesses current evidence, prunes weak branches, and decides whether to deepen, backtrack, or branch at the current level.

## When to Use

- After an experiment: should we continue this direction or pivot?
- Multiple options available: which path to take?
- Results are disappointing: should we stop, narrow scope, or try something else?
- Existing state from prior work: what's worth keeping?

## Workflow

1. **State the question**: What decision needs to be made?
2. **Collect evidence**: What do we know? What results exist?
3. **Identify candidates**: 2-3 concrete options
4. **For each option, assess**:
   - Expected value (what's the upside?)
   - Feasibility (can we actually do it?)
   - Evidence cost (how much work to validate?)
   - Risk (what could go wrong?)
5. **Apply incumbent/frontier heuristic**:
   - Is the current direction still the best given evidence?
   - If not, which alternative has the strongest case?
6. **Decide and record**: State the choice, the reason, and the abandonment condition

## Decision Types

| Decision | Options |
|----------|---------|
| After experiment | continue / iterate / pivot / stop |
| Scope | narrow claims / broaden experiments / hold |
| Baseline | reproduce / use published numbers / skip |
| Writing | draft now / gather more evidence first |
| Submission | submit / revise / target different venue |

## BaB Operations

| Operation | When | Action |
|-----------|------|--------|
| **Deepen** | Current branch is promising | Go to next finer level (L0→L1→L2→...) |
| **Branch** | Need alternatives at current level | Generate 2-3 candidates, evaluate |
| **Prune** | Evidence shows branch is weak | Record why, close the branch |
| **Backtrack** | Fine-grained work reveals coarse-level problem | Go back to coarser level, re-evaluate |
| **Multi-split** | Path is clear across multiple levels | Skip intermediate evaluations, jump ahead |

## Pruning and Abandonment

When evidence weakens a direction:
- State what was pruned and at which level
- State which evidence caused it
- State whether to retry, pivot, narrow scope, or stop
- State what future evidence would reopen this branch

**Record pruned branches in status.md** — failed branches are evidence that guides future decisions.

## Intake Assessment (for existing projects)

When starting from existing work:
- Inventory what exists: code, results, drafts, data
- Trust-rank each asset: trustworthy / needs-verification / unreliable
- Identify gaps: what's missing to proceed?
- Recommend starting point: which stage to enter?

## Output

Updated `plan.md` with the decision and next steps. Knowledge card documenting the decision rationale if it's significant.
