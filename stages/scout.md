# Stage: Scout

Establish stable research framing through literature survey, dataset/metric clarification, and baseline discovery.

## When to Use

- Starting a new research direction
- Unclear what baselines exist
- Need to understand the competitive landscape
- Dataset or evaluation protocol is underspecified

## Workflow

1. **Frame the question**: What exactly is the research gap? What would a solution look like?
2. **Bounded literature search**: Find 5-10 key papers. Use layered search:
   - Direct: papers solving the same problem
   - Mechanism: papers using similar techniques in other domains
   - Bottleneck: papers analyzing why current methods fail
   - Adjacent: papers in neighboring subfields that might transfer
3. **For each paper, extract**:
   - What it changes and why
   - What assumptions must hold
   - What evidence supports the claim
   - What weakness remains
4. **Identify baselines**: Which existing methods are the strongest fair comparators?
5. **Clarify evaluation**: What dataset, split, metric, and protocol will be used?
6. **Produce a baseline shortlist**: 2-3 strongest baselines with reproduction feasibility assessment

## Key Heuristics

- Search for **disconfirming** evidence, not just supportive
- Stop on clarity, not exhaustion — if you have 5 solid papers and a clear gap, that's enough
- Prefer papers with code over papers without
- Read with a candidate claim in mind — avoid passive reading
- Classify baselines as: full_reproducible / degraded_acceptable / blocked

## Output

Update `brief.md` with research context. Update `plan.md` with baseline + evaluation details. Create `knowledge/` cards for key papers and design decisions.
