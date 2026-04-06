# Stage: Idea

Generate concrete, literature-grounded hypotheses and select the most promising direction.

## When to Use

- Baseline is understood, need to decide what to try
- Existing approach hit a wall, need a new direction
- Multiple possible improvements, need to prioritize

## Workflow

1. **Start from the limitation, not the method**:
   - What exactly fails? Under what condition?
   - Why is that failure important for the target metric?
   - Is this a real pattern or noise?
2. **Separate symptom, mechanism, and consequence**:
   - Symptom: what is observed
   - Mechanism hypothesis: what might cause it
   - Consequence: why it hurts the metric
3. **Build competing hypotheses**: For each limitation, keep 1 main + 2-3 alternatives
4. **Name the lever bucket**: data / model / objective / optimization / inference / evaluation
5. **Literature-informed search**: Every paper should answer one of:
   - Has this been tried?
   - Has this failure been explained?
   - Is there a transferable mechanism from another domain?
   - What is the strongest version of what we might be reinventing?
6. **Bounded divergence**: Generate 6-12 raw ideas, then filter to 2-3 serious candidates
7. **For each candidate**:
   - Two-sentence pitch
   - Strongest objection
   - Minimal experiment to falsify
   - Abandonment condition
8. **Select**: Choose the candidate with the best evidence-per-compute ratio

## Key Heuristics

- Prefer ideas with **fast falsification** — cheap to disprove
- "Why now?" — what changed that makes this tractable now?
- A good idea generates new knowledge even if the result is negative
- Mechanism family first, within-family variants second
- If you can't name the lever bucket, the idea is too fuzzy
- Frontier diversity cap: max 1-2 per mechanism family

## Output

Update `plan.md` with selected direction. Create `knowledge/` card for the idea with rationale, alternatives considered, and abandonment condition.
