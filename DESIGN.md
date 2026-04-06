# researchSkills: Distilled Research Workflow for CLI

Distilled from DeepScientist's 13 skills + 56 reference files + 730-line system prompt into a lean, CLI-first research workflow that runs directly with `claude -p`.

## Philosophy

- **No daemon, no web UI, no MCP servers** — just Claude CLI + well-crafted prompts
- **Files are the state** — markdown docs in your project directory ARE the state machine
- **You control the loop** — invoke stages manually when ready, not auto-scheduled
- **Prompts are composable** — system prompt + stage skill + project context = one Claude invocation

## Architecture

```
researchSkills/
├── DESIGN.md                    # This file
├── system.md                    # Core system prompt (~100 lines)
├── stages/                      # Research stage skills (7 distilled from 13)
│   ├── scout.md                 # Literature survey + baseline identification
│   ├── idea.md                  # Hypothesis generation + selection
│   ├── experiment.md            # Run design + execution + recording
│   ├── write.md                 # Paper drafting (edit existing .tex directly)
│   ├── review.md                # Self-review as adversarial reviewer
│   ├── rebuttal.md              # Reviewer response mapping
│   └── decision.md              # Go/stop/branch/pivot routing
├── references/                  # Reusable research heuristics (standalone)
│   ├── paper-section-playbook.md
│   ├── reviewer-first-writing.md
│   ├── idea-thinking-flow.md
│   ├── evidence-ladder.md
│   ├── comparability-contract.md
│   └── section-contracts.md
├── templates/                   # New project scaffolding
│   └── project/
│       ├── brief.md             # Research brief template
│       ├── plan.md              # Plan template
│       ├── status.md            # Status template
│       └── knowledge/           # Knowledge cards directory
└── rs                           # CLI helper script
```

## Distillation Decisions

### What was kept (research methodology gold)
- Evidence-first discipline (never write claims without evidence)
- Bounded search heuristic (stop on clarity, not exhaustion)
- Incumbent/frontier selection (2-3 serious candidates, not brainstorm lists)
- Comparability contract (same task/dataset/split/metric)
- Adversarial self-review (read as skeptical reviewer)
- Downgrade discipline (preserve negative evidence, narrow claims)
- Paper section playbook (what each section must answer)
- Reviewer-first writing (5-minute pass simulation)
- Idea thinking flow (limitation-first, not method-first)
- Evidence ladder (minimum → solid → maximum)
- Cold start / curriculum strategies

### What was dropped (DS infrastructure)
- 730-line system prompt → ~100 lines
- MCP tool contracts (memory, artifact, bash_exec)
- Artifact state machine (paper_contract_health, bundle, outline manifests)
- Daemon scheduling (baseline_gate, active_anchor, skill dispatch)
- Interaction protocol (artifact.interact, reply_mode, dedupe_key)
- Paper line / branch / worktree management
- Connector delivery (QQ, Telegram, WeChat)
- Process lifecycle protocol (detach/await/kill)

### What was merged
- baseline + experiment → `experiment.md` (baseline is just the first experiment)
- optimize + idea → `idea.md` (optimization is iterative ideation)
- analysis-campaign → folded into `experiment.md` (supplementary runs)
- intake-audit → folded into `decision.md` (assess existing state)
- finalize → folded into `review.md` (final check before submission)
- figure-polish → folded into `write.md` (figures are part of writing)

## Usage

### One-shot stage invocation
```bash
# Run a specific stage on your project
./rs write ~/my-project "Complete the Method section"

# Which expands to:
claude -p --model sonnet --dangerously-skip-permissions \
  --system-prompt "$(cat system.md)\n\n$(cat stages/write.md)" \
  "Project context:\n$(cat ~/my-project/brief.md)\n$(cat ~/my-project/plan.md)\n\nKnowledge:\n$(cat ~/my-project/knowledge/*.md)\n\nTask: Complete the Method section"
```

### Canonical research flow
```
scout → idea → experiment → write → review → submit
                  ↑                    |
                  └── decision ────────┘
```

Each stage reads your project's `brief.md`, `plan.md`, `status.md`, and `knowledge/*.md` as context. You invoke stages manually when ready.

### Start a new project
```bash
./rs init ~/my-new-project "Research topic description"
```

## Key Design Difference from DeepScientist

| Aspect | DeepScientist | researchSkills |
|--------|---------------|----------------|
| Invocation | `ds run write --quest-id X` | `./rs write ~/project "task"` |
| State | daemon + artifact state machine | files in project directory |
| Prompt | 730-line system + skill + context | ~100-line system + skill + context |
| Paper editing | outline → draft.md → LaTeX bundle | edit .tex directly |
| Loop control | daemon auto-schedules skills | you invoke manually |
| Dependencies | Node.js + Python + uv + npm | just `claude` CLI |
