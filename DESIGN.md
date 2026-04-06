# researchSkills: Design Document

A lean, CLI-first research workflow that runs directly with Claude CLI.

## Philosophy

- **Just Claude CLI + well-crafted prompts** — no extra infrastructure
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

### What was deliberately excluded
- Complex state machines and artifact management
- Daemon-based scheduling and auto-dispatch
- Web UI and messaging platform connectors
- Intermediate format pipelines (outline → draft → bundle)

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

## Key Design Choices

| Aspect | Choice | Why |
|--------|--------|-----|
| State management | Plain markdown files | No infrastructure to maintain |
| Paper editing | Edit .tex directly | No intermediate format overhead |
| Loop control | User invokes stages manually | Full control over research direction |
| Dependencies | Just Claude CLI | Minimal setup |
| Prompt size | ~100-line system + stage skill | Maximizes context for actual work |
