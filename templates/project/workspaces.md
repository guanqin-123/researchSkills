# Workspaces

Registry of all codebases, tools, and data involved in this project.

## Repos

<!-- Each repo this project touches. Path = local clone location. -->

| ID | Role | Path | Upstream | Branch | Notes |
|----|------|------|----------|--------|-------|
| `paper` | paper | ./paper | — | main | LaTeX source |

<!-- Example entries:
| `abcrown` | baseline-framework | ~/repos/alpha-beta-CROWN | https://github.com/Verified-Intelligence/alpha-beta-CROWN | main | BaB verification framework |
| `multisplit` | our-method | ~/repos/multisplit | (private) | dev | MultiSplit predictor implementation |
| `vnncomp` | data | ~/data/vnncomp2024 | https://github.com/ChristopherBrix/vnncomp2024_benchmarks | main | Benchmark models + specs |
-->

## Environments

<!-- How to set up each repo. One entry per distinct environment. -->

| Repo ID | Setup command | Python | GPU required | Notes |
|---------|-------------|--------|-------------|-------|

<!-- Example:
| `abcrown` | `cd ~/repos/alpha-beta-CROWN && pip install -e .` | 3.10 | yes | needs CUDA 12+ |
| `multisplit` | `cd ~/repos/multisplit && uv sync` | 3.11 | yes | |
-->

## Data & Assets

<!-- Datasets, pretrained models, checkpoints shared across repos. -->

| ID | Type | Path | Source | Size | Notes |
|----|------|------|--------|------|-------|

<!-- Example:
| `acasxu` | benchmark | ~/data/vnncomp/acasxu | VNN-COMP 2024 | 50MB | 45 networks, 10 properties |
| `mnist-fc` | benchmark | ~/data/vnncomp/mnist_fc | VNN-COMP 2024 | 200MB | |
| `predictor-ckpt` | checkpoint | ./experiments/checkpoints/ | trained locally | — | MultiSplit predictor weights |
-->

## Cross-Repo Dependencies

<!-- How repos depend on each other. -->

```
paper ← results from experiments
  ↑
experiments (run in abcrown + multisplit)
  ↑
multisplit → patches/integrates into abcrown
  ↑
abcrown ← vnncomp benchmarks as data
```
