# Stage: Experiment

Execute evidence-producing runs — baseline reproduction, main experiments, and supplementary analysis.

## When to Use

- Need to reproduce a baseline for fair comparison
- Implementing and testing the proposed method
- Running ablations or supplementary experiments
- Filling evidence gaps identified during writing

## Workflow

### Baseline Reproduction
1. Identify source (paper + code repo)
2. Fast-path first: can you use published numbers directly? If comparable setup, attach them.
3. If reproduction needed: setup environment, run on same data/split/metric
4. Record: what was reproduced, any discrepancies, comparability notes

### Main Experiment
1. **Define run contract**:
   - Research question → hypothesis → controls → metric
   - What result would count as success?
   - What result would count as failure?
2. **Implement minimum change**: Only modify what the hypothesis requires
3. **Pilot before scale**: Run a small smoke test first
4. **Execute and record**: Full run with all metrics
5. **Validate**: Check outputs are reasonable, no silent failures

### Supplementary Analysis (Ablations, Robustness)
- One question per run
- Claim-critical ablations first
- Preserve comparability (same evaluation contract as main experiment)
- Classify results: stable / fragile / contradiction / unresolved

## Key Heuristics

- **Comparability contract**: same task, dataset, split, preprocessing, evaluation code, metric
- **Pilot before scale**: always
- **Last-known-good**: keep working version before each change
- **Record incrementally**: don't wait until the end to write down results
- Feasibility classes: full_reproducible / degraded_acceptable / blocked

## Evidence Ladder

- **minimum**: basic executable result, comparable setup, direction not obviously broken
- **solid**: credible main comparison, strong fair baseline, stable results, significance testing
- **maximum**: main claim credible, additional analysis broadens confidence and scope

Before polishing to maximum, first move from minimum to solid.

## Multi-Workspace Protocol

When experiments span multiple repos (e.g., implement in repo A, evaluate in repo B):

1. Read `workspaces.md` to know what repos exist and where
2. Before running: confirm which repo, branch, and environment
3. Record the **exact** repo + commit + command in `experiments/runs.md`
4. Cross-repo results go in `experiments/runs.md` with repo ID linkage
5. After runs: update `workspaces.md` if new branches/checkpoints were created

## Output

Update `status.md` with results. Record key numbers in `knowledge/` cards. Update `plan.md` to reflect what's done. **Append to `experiments/runs.md`** with full reproducibility info.
