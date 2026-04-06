# Unified Research Workflow

A stage-agnostic orchestration protocol. You can enter at any point, and the system routes you forward based on evidence in your project files.

## Foundational Model: Research as Branch-and-Bound

The entire research process — from initial idea to final paper — is a **hierarchical Branch-and-Bound** process. At every granularity level, you:

1. **Branch**: Generate 2-3 candidate options (not an open-ended brainstorm)
2. **Bound**: Quick evaluation — is this candidate worth pursuing? (pilot, literature check, feasibility estimate)
3. **Prune**: Eliminate weak branches with explicit abandonment rationale
4. **Deepen**: Commit to the surviving branch and refine at the next finer granularity

### Granularity Levels

```
Level 0: Research direction    "What problem to work on?"
  └─ Level 1: Approach         "Which method family?"
       └─ Level 2: Design      "Which architecture, loss, features?"
            └─ Level 3: Impl    "Which hyperparams, training schedule?"
                 └─ Level 4: Writing  "Which narrative, framing, claims?"
```

Each level follows the same branch → bound → prune → deepen cycle. The `decision` stage is the **bound evaluation** at every level.

### Cross-level Backtracking

Fine-grained work can reveal that a coarser decision was wrong:

| Signal | Backtrack to |
|--------|-------------|
| Experiment results disappointing | Level 2 (redesign) or Level 1 (new approach) |
| Design doesn't fit the framework | Level 1 (reconsider method family) |
| Writing reveals logic gap in method | Level 2 (fix design) |
| Reviewer rejects core premise | Level 0 (reconsider direction) |

**Rule**: When backtracking, explicitly record what failed and why. Don't silently restart — the failed branch is evidence too.

### Multi-Split Analogy

Just like MultiSplit predicts groups of neurons to split simultaneously in BaB verification, experienced researchers make **multi-level decisions at once** when the path is clear: "I know the approach (L1) AND the architecture (L2) AND the loss function (L2) — let me jump straight to implementation (L3)." This skips intermediate bound evaluations, saving time. But when uncertain, evaluate one level at a time.

## State Machine

**`status.md` is the state machine.** It tracks where you are (which BaB level + which stage), what's done, and what's next. Every stage reads it on entry, updates it on exit. The `decision` stage is the universal router — run it whenever the next step isn't obvious.

## Lifecycle Model

```
          ┌─────────────────────────────────┐
          │           decision               │
          │    (bound evaluation at any       │
          │     level — prune or deepen)      │
          └──┬───┬───┬───┬───┬───┬───┬──────┘
             │   │   │   │   │   │   │
             v   v   v   v   v   v   v
          scout idea exp write review rebuttal
             │   │   │    │     │      │
             └───┴───┴────┴─────┴──────┘
                    all update status.md
```

There is no enforced linear order. The transitions are:

| From | Typical next | When |
|------|-------------|------|
| **scout** | idea or experiment | Landscape is clear, gap identified |
| **idea** | experiment | Direction selected, ready to test |
| **experiment** | decision | Results obtained, need to evaluate |
| **decision** | idea / experiment / write / stop | Based on evidence quality |
| **write** | review | Draft sections complete |
| **review** | write / experiment / rebuttal | Depending on issues found |
| **rebuttal** | experiment / write | Depending on reviewer requests |

## State Protocol

`status.md` is the single source of truth for routing. It must always contain:

```markdown
# Status

**Stage**: [current stage name]
**State**: [one sentence: what's happening now]

## Completed
- [x] [What was done, with date]

## Next Steps
1. [Recommended next action]
2. [Alternative if #1 is blocked]

## Decisions Log
| Date | Decision | Reason | Next Stage |
|------|----------|--------|------------|

## Evidence State
- **Baseline**: [none / attached / reproduced / verified]
- **Main result**: [none / pilot / solid / maximum]
- **Paper**: [no draft / partial / complete / reviewed]
- **Submission**: [not ready / ready / submitted / in revision]
```

## Entry Contract

Every stage starts the same way:

1. Read `status.md` — where are we?
2. Read `brief.md` — what's the goal?
3. Read `plan.md` — what's the plan?
4. Read `knowledge/*.md` — what do we know?
5. Do the stage-specific work
6. Update `status.md` — what changed? What's next?

**Drop-in at any point**: If you're starting from existing work, run `decision` first. It will inventory your assets and recommend an entry stage.

## Exit Contract

Every stage ends by updating `status.md` with:

1. What was completed this run
2. Current evidence state (baseline / main result / paper / submission)
3. Recommended next stage and why
4. Any decisions made (append to Decisions Log)

## The Decision Loop

The fundamental pattern for any research project:

```bash
# 1. Start or assess where you are
./rs decision ~/project "Where are we? What should we do next?"

# 2. Execute the recommended stage
./rs <recommended_stage> ~/project "specific task"

# 3. After significant work, route again
./rs decision ~/project "Evaluate results and decide next step"

# 4. Repeat until done
```

## Common Research Patterns

### Pattern A: Paper-First (theory/method paper)
You have an idea, need to write the paper and then run experiments to fill in results.

```bash
./rs scout ~/project "Survey related work on [topic]"
./rs idea ~/project "Formalize the method and select architecture"
./rs write ~/project "Draft Method and Training sections"
./rs experiment ~/project "Implement and run main comparison"
./rs write ~/project "Fill in Experiment section with actual results"
./rs review ~/project "Full adversarial review before submission"
```

### Pattern B: Experiment-First (empirical paper)
You have a hypothesis, need to validate it first, then write up.

```bash
./rs scout ~/project "Find strongest baselines for [task]"
./rs experiment ~/project "Reproduce baseline on [dataset]"
./rs idea ~/project "Design ablation to test [hypothesis]"
./rs experiment ~/project "Run main experiment + ablations"
./rs decision ~/project "Are results strong enough to write up?"
./rs write ~/project "Draft full paper from results"
./rs review ~/project "Pre-submission check"
```

### Pattern C: Revision (after reviews)
Paper was reviewed, need to respond.

```bash
./rs rebuttal ~/project "Map reviewer feedback into action items"
./rs experiment ~/project "Run additional experiments requested by R2"
./rs write ~/project "Revise paper based on reviewer feedback"
./rs rebuttal ~/project "Draft response letter"
```

### Pattern D: Exploration (new research area)
Don't know what to work on yet.

```bash
./rs scout ~/project "Broad survey of [field]"
./rs decision ~/project "Which direction has the most potential?"
./rs idea ~/project "Develop top candidate into concrete hypothesis"
./rs experiment ~/project "Quick pilot to validate feasibility"
./rs decision ~/project "Is this worth pursuing further?"
```

### Pattern E: Drop-in (existing project)
Picking up from existing work (code, partial results, draft).

```bash
./rs decision ~/project "Assess existing assets and recommend entry point"
# Then follow the recommendation
```

## Stage Compatibility Matrix

Which stages can follow which:

| Stage | Can be followed by |
|-------|-------------------|
| scout | idea, experiment, decision, write |
| idea | experiment, decision, write |
| experiment | decision, write, idea |
| write | review, experiment, decision |
| review | write, experiment, decision |
| rebuttal | experiment, write, decision |
| decision | any stage |

## Evidence Quality Gates

These are soft gates — guidelines for when to transition, not hard blocks:

| Transition | Gate |
|-----------|------|
| scout → idea | Landscape is clear, gap is identified, 5+ key papers read |
| idea → experiment | Direction is concrete, abandonment condition defined |
| experiment → write | At least "solid" evidence (credible comparison, stable results) |
| write → review | All claim-carrying sections have content |
| review → submit | No critical issues, all claims backed by evidence |

If evidence isn't strong enough, the decision stage routes you back to gather more.

## Adding to Reference Context

For stages that benefit from extra guidance, use `RS_REFS`:

```bash
# Writing with reviewer-first perspective
RS_REFS="reviewer-first-writing,paper-section-playbook" ./rs write ~/project "..."

# Ideation with structured thinking flow
RS_REFS="idea-thinking-flow" ./rs idea ~/project "..."

# Experiment with evidence standards
RS_REFS="evidence-ladder,comparability-contract" ./rs experiment ~/project "..."
```

## Two Execution Modes

### Interactive (default) — Claude works + asks you questions

```bash
./rs write ~/project "Complete the Method section"
```

Claude works on your files and **pauses at decision points** to ask you:
- Direction choices (A or B?)
- Scope changes (narrow or proceed?)
- Evidence gaps (placeholder or stop?)
- Design decisions (GNN or Attention?)

You answer in the terminal, Claude continues. This is the **recommended mode** for most work.

### Batch — Claude runs to completion, no questions

```bash
./rs write -p ~/project "Complete the Method section"
```

One-shot execution. Claude does its best without asking. Good for:
- Well-defined tasks with no ambiguity
- Background/nohup execution
- Tasks where you trust Claude's judgment fully

## Integration with Claude CLI

Under the hood:

```
rs <stage> <project> "task"
  ↓
Interactive: claude --system-prompt "..." -p "..." --resume
Batch:       claude -p --system-prompt "..." "..."
  ↓
Claude reads project files, does work, edits files directly
  ↓
Interactive: pauses at checkpoints, asks you, continues
Batch: runs to completion
  ↓
You inspect changes, then run next stage (or rs next)
```

No daemon. No state machine. No MCP. Just files + prompts + Claude.
