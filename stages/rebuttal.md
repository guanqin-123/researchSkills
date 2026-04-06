# Stage: Rebuttal

Map reviewer feedback into experiments, manuscript changes, and a structured response letter.

## When to Use

- Paper received reviews with requests for revision
- Need to plan rebuttal experiments
- Need to write response letter

## Workflow

1. **Normalize reviews**: Break each review into atomic items (one concern per item)
2. **Classify each item**:
   - `editorial`: typo, formatting, citation fix
   - `text_only`: clarification, better explanation needed
   - `evidence_gap`: missing ablation, comparison, or analysis
   - `experiment_gap`: new experiment required
   - `claim_scope`: claims need narrowing or qualification
3. **Decide stance for each**:
   - `agree`: reviewer is right, we will change
   - `partially_agree`: valid point, but we address it differently
   - `clarify`: misunderstanding we can resolve with better text
   - `respectful_disagree`: we maintain our position with evidence
4. **Route actions**:
   - editorial/text_only → edit paper directly
   - evidence_gap → run supplementary experiment, then edit paper
   - experiment_gap → design new experiment, run it, then edit paper
   - claim_scope → narrow claims in paper
5. **Write response letter**:
   - For each item: restate concern → our response → what changed (with diff pointers)
   - Answer the concern directly in the first 1-2 sentences
   - Be specific about what was added/changed and where

## Key Heuristics

- **Don't argue, demonstrate**: if reviewer wants evidence, provide it
- **Selective concede/clarify/defend**: don't defend everything or concede everything
- **For non-experimental items**: reduce uncertainty with evidence, not rhetoric
- **New experiments get the same rigor**: comparability contract applies
- **Track which reviewer items are addressed**: maintain a checklist

## Response Letter Structure

```
## Reviewer 1

### R1-C1: [Concern summary]
**Stance**: agree / partially agree / clarify / respectful disagree

[Response: 2-4 sentences addressing the concern directly]

**Changes**: [What was modified, with section/line references]

### R1-C2: ...
```

## Output

Response letter draft. Updated paper with revisions. New experiment results if needed. Updated `status.md` with revision status.
