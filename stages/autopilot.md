# Stage: Autopilot

Autonomous experiment loop. Modify code, run experiments, measure results, keep or discard — repeat forever until a strategic decision is needed.

## When to Use

- You have a clear quantitative metric to optimize
- The experimental setup is stable (data, evaluation, baseline all working)
- You want unattended overnight or long-running experiment search
- L0-L2 decisions (direction, approach, design) are already settled

## Before Starting

1. Read `autopilot.md` — metric config, run command, thresholds
2. Read `status.md` — current BaB state, what's been tried
3. Read `experiments/results.tsv` — past results and current best
4. Read `workspaces.md` — which repo to modify
5. Read the actual code you'll be editing
6. If `results.tsv` has no data rows, your first run is the **baseline**: run the code as-is, record the result as `keep baseline`

## The Loop

**LOOP FOREVER** — do not stop unless you hit a strategic decision.

```
1. READ    — status.md, experiments/results.tsv (know current best metric)
2. THINK   — Pick ONE change to try (single variable, informed by past results)
3. MODIFY  — Edit the code (minimum viable change)
4. COMMIT  — git add <files> && git commit -m "autopilot: <description>"
5. RUN     — Execute the run command from autopilot.md config
6. MEASURE — Extract metric using grep_pattern from log file
7. EVALUATE:
   - Metric improved >= threshold  →  KEEP
   - Metric not improved           →  DISCARD: git reset --hard HEAD~1
   - Crash (trivial)               →  Fix and retry (counts as same round)
   - Crash (fundamental)           →  Log as "crash", git reset --hard HEAD~1, move on
   - Timeout                       →  Kill, log as "timeout", git reset --hard HEAD~1
8. LOG     — Append to experiments/results.tsv AND experiments/runs.md
9. CHECK   — Strategic decision needed?
   - YES → write pending_question.md, EXIT
   - NO  → go to step 2
```

**NEVER STOP**: Do not pause to ask "should I continue?" or "is this a good stopping point?". The user may be asleep or away. You run indefinitely until manually stopped or until a strategic decision is required. If you run out of ideas, think harder — re-read the code, re-read past results, try combining near-misses, try more radical changes at L3.

## Keep / Discard Rules

Read `autopilot.md` for:
- **name**: the metric to optimize
- **direction**: `higher_is_better` or `lower_is_better`
- **threshold**: minimum improvement to count as "keep"
- **grep_pattern**: regex to extract metric value from log output

**Keep**: metric improved by at least `threshold` in the configured direction.
**Discard**: metric unchanged, worsened, or improved less than `threshold`.

After discarding, always `git reset --hard HEAD~1` to revert to the last kept state.

## Logging

### experiments/results.tsv (machine-readable, append-only)

Tab-separated, one row per experiment:

```
run_id	commit	metric_value	prev_best	status	description	timestamp
```

- **run_id**: sequential (001, 002, ...)
- **commit**: short git hash (7 chars) of the experimental commit
- **metric_value**: extracted metric (use 0 for crashes)
- **prev_best**: best metric value before this run
- **status**: `keep`, `discard`, `crash`, or `timeout`
- **description**: short text of what was tried
- **timestamp**: ISO 8601 (YYYY-MM-DDTHH:MM:SS)

### experiments/runs.md (human-readable, append-only)

Use the standard run template for kept experiments. For discarded experiments, a one-line entry is sufficient.

## Crash Handling

- **Trivial** (import error, syntax error, OOM): fix the issue and retry. Reducing batch size for OOM is a valid fix. This counts as the same round, not a new experiment.
- **Fundamental** (missing data, broken evaluation, logic error in experimental setup): log as "crash" in results.tsv, `git reset --hard HEAD~1`, and **escalate** — this likely indicates a problem beyond L3.
- **Timeout** (exceeds configured timeout): kill the process, log as "timeout", revert. Try a smaller/faster configuration next.

If crashes exceed `max_consecutive_failures` from autopilot.md config, **escalate**.

## Simplicity Criterion

When evaluating whether to keep a change, weigh complexity cost against improvement:

- Small improvement + significant added complexity → DISCARD (not worth maintaining)
- Small improvement + deleted code → KEEP (simplification win)
- ~0 improvement but simpler code → KEEP (architectural improvement)
- Large improvement + any complexity → KEEP

## Escalation Rules

### Must escalate (write `pending_question.md` and stop)

- You believe the current L2 design or L1 approach should change
- You want to backtrack to L0 (change research direction entirely)
- You propose stopping this line of research
- Consecutive failures exceed `max_consecutive_failures`
- The search space at L3 is exhausted (all reasonable variants tried, no improvement)
- You discover the experimental setup has a fundamental flaw (wrong data, broken metric)

### Auto-resolve (no escalation, just do it)

- L3 implementation choices: hyperparameters, batch size, learning rate, regularization
- L4 writing/formatting: log format, metric display, code style
- Trivial crash recovery: fix imports, reduce batch size, fix syntax
- Choosing which variable to modify next
- Keep/discard based on metric threshold

When auto-resolving, log your reasoning in the `description` field of results.tsv. Do not stop for confirmation.

## Pending Question Format (for escalation)

When you must escalate, write `pending_question.md` in the project directory:

```markdown
# Pending Question

**Stage**: autopilot
**Level**: L[N]
**Type**: [BACKTRACK / EXHAUSTED / FUNDAMENTAL_ISSUE / STOP]

## Context
[Current autopilot state: rounds completed, best metric, recent trend]

## Question
[The strategic question you need answered]

## Options
1. [Option A] — [tradeoff]
2. [Option B] — [tradeoff]

## Recommendation
[Your recommendation and why]
```

After writing this file, **stop the loop and exit**. The notification system will email the user.

## Status Updates

Every 10 rounds (or on escalation), update `status.md`:
- Append a Session Log entry summarizing the autopilot batch
- Update Evidence State if results warrant it
- Update BaB Tree Snapshot if any L3 decisions were committed

## Output

- `experiments/results.tsv` with all experiment results
- `experiments/runs.md` with detailed entries for kept experiments
- Updated `status.md` with session log
- Git history with descriptive commits for all kept changes
