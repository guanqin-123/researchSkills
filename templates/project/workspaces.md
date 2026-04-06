# Workspaces

Registry of all codebases, tools, and data involved in this project.

## Repos

<!-- Each repo this project touches. Path = local clone location. -->

| ID | Role | Path | Upstream | Branch | Notes |
|----|------|------|----------|--------|-------|
| `paper` | paper | ./paper | — | main | LaTeX source |

<!-- Example entries:
| `baseline` | baseline-framework | ~/repos/baseline | https://github.com/org/baseline | main | reference implementation |
| `ours` | our-method | ~/repos/our-method | (private) | dev | proposed approach |
| `data` | data | ~/data/benchmarks | https://github.com/org/benchmarks | main | evaluation datasets |
-->

## Environments

<!-- How to set up each repo. One entry per distinct environment. -->

| Repo ID | Setup command | Python | GPU required | Notes |
|---------|-------------|--------|-------------|-------|

## Data & Assets

<!-- Datasets, pretrained models, checkpoints shared across repos. -->

| ID | Type | Path | Source | Size | Notes |
|----|------|------|--------|------|-------|

## Cross-Repo Dependencies

<!-- How repos depend on each other. -->

```
paper ← results from experiments
  ↑
experiments (run in baseline + ours)
  ↑
ours → integrates with baseline framework
  ↑
baseline ← benchmark data
```
