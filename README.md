# rs — Research Skills CLI

Turn Claude into your research co-pilot. One shell command per research stage, interactive by default, no infrastructure needed.

## Prerequisites

- [Claude CLI](https://docs.anthropic.com/en/docs/claude-cli) installed and logged in (`claude --version`)

That's it. No Node.js, no Python, no daemon, no web server.

## Quick Start

```bash
# 1. Initialize a project
./rs init ~/my-paper "Learned multi-neuron branching for BaB verification"

# 2. Edit brief.md with your research context
vim ~/my-paper/brief.md

# 3. Start working — Claude asks you questions along the way
./rs scout ~/my-paper "Survey related work and identify baselines"
```

## Commands

| Command | What it does |
|---------|-------------|
| `rs <stage> <dir> "task"` | Run a stage interactively (default) |
| `rs <stage> -p <dir> "task"` | Run in batch mode (no questions) |
| `rs next <dir>` | Ask: what should I do next? |
| `rs catchup <dir>` | Structured briefing after a break |
| `rs status <dir>` | Quick view of current state |
| `rs init <dir> "topic"` | Create a new project |
| `rs list` | Show available stages |

## Stages

```
scout    Survey literature, identify baselines and evaluation protocol
idea     Generate hypotheses, select direction with abandonment conditions
experiment  Run baselines, main experiments, ablations
write    Draft paper sections — edits .tex/.md directly
review   Adversarial self-review before submission
rebuttal Map reviewer feedback into action + response letter
decision Route: go / stop / pivot / backtrack (the BaB bound evaluation)
```

## How It Works

### Research as Branch-and-Bound

The entire research process is hierarchical BaB — from picking a direction down to writing a sentence:

```
L0  Direction    "What problem?"         ← branch / bound / prune
L1  Approach     "Which method family?"  ← branch / bound / prune
L2  Design       "Architecture, loss?"   ← branch / bound / prune
L3  Implementation "Hyperparams?"        ← branch / bound / prune
L4  Writing      "Narrative, framing?"   ← branch / bound / prune
```

At every level, Claude presents candidates, you pick, Claude deepens. If something breaks at L3, you backtrack to L2. All decisions are recorded in `status.md`.

### Interactive by Default

Claude pauses at every decision point and asks you:

```
At Level 2 (design):

Candidates I see:
1. GNN — captures network structure, slower inference
2. Attention — models pairwise interactions, fits non-additivity argument
3. MLP — fast baseline, ignores structure

Should I evaluate all three, or do you already know which to pursue?
> 2
```

Use `-p` for batch mode when the task is unambiguous.

### Files Are the State

```
~/my-paper/
├── brief.md            What this research is about
├── plan.md             What's done and what's next
├── status.md           BaB state + session log + decisions
├── workspaces.md       Repos, data, environments registry
├── knowledge/          Design decisions, related work notes
├── paper/              Your .tex files
├── literature/         .bib, PDFs
├── experiments/
│   └── runs.md         Append-only experiment log (repo + commit + command + result)
└── figures/
```

No database. No daemon state. `status.md` IS the state machine. `workspaces.md` IS the repo registry. `experiments/runs.md` IS the experiment tracker.

### Multi-Repo Projects

Register all codebases in `workspaces.md`:

```markdown
| ID | Role | Path | Branch | Notes |
|----|------|------|--------|-------|
| `abcrown` | baseline-framework | ~/repos/alpha-beta-CROWN | main | BaB verifier |
| `multisplit` | our-method | ~/repos/multisplit | dev | predictor impl |
| `vnncomp` | data | ~/data/vnncomp2024 | main | benchmarks |
```

Every experiment run in `experiments/runs.md` links back to a repo + commit:

```markdown
### run-003: MultiSplit vs FSB on ACAS Xu
- **Repo**: `abcrown` @ `dev-multisplit` @ `a3f2c1d`
- **Command**: `python abcrown.py --config exp/acasxu_multisplit.yaml`
- **Result**: verified 42/45 in 180s (FSB: 42/45 in 340s)
```

## Typical Workflows

### Writing a paper (you have an idea, need to write + experiment)

```bash
./rs scout ~/paper "Survey multi-neuron branching methods"
./rs idea ~/paper "Formalize the predictor architecture"
./rs write ~/paper "Draft Method and Training sections"
./rs experiment ~/paper "Run main comparison on VNN-COMP"
./rs write ~/paper "Fill in Experiment section with results"
./rs review ~/paper "Full review before NeurIPS submission"
```

### Picking up after a break

```bash
./rs catchup ~/paper           # "Here's where you left off..."
./rs next ~/paper              # "I recommend: rs write ..."
```

### Responding to reviews

```bash
./rs rebuttal ~/paper "Map R1-R3 feedback into action items"
./rs experiment ~/paper "Run ablation requested by R2"
./rs write ~/paper "Revise Method section per R1 feedback"
./rs rebuttal ~/paper "Draft response letter"
```

## Adding Context

Put extra reference guides in the prompt with `RS_REFS`:

```bash
RS_REFS="reviewer-first-writing,paper-section-playbook" ./rs write ~/paper "Draft introduction"
```

Available references: `rs refs`

## Configuration

| Env var | Default | What |
|---------|---------|------|
| `RS_MODEL` | `sonnet` | Claude model |
| `RS_CLAUDE` | `claude` | Claude binary path |
| `RS_REFS` | (none) | Comma-separated reference files to include |

## Deeper Docs

- [WORKFLOW.md](WORKFLOW.md) — BaB research model, stage transitions, common patterns
- [DESIGN.md](DESIGN.md) — Architecture and design decisions
