#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STAGES_DIR="$SCRIPT_DIR/stages"
REFS_DIR="$SCRIPT_DIR/references"
TEMPLATES_DIR="$SCRIPT_DIR/templates/project"
SYSTEM_PROMPT="$SCRIPT_DIR/system.md"

CLAUDE_MODEL="${RS_MODEL:-sonnet}"
CLAUDE_BINARY="${RS_CLAUDE:-claude}"

usage() {
    cat <<'EOF'
rs - Research Skills CLI

Usage:
  rs <stage> <project_dir> "task"        Interactive: Claude works + asks you questions
  rs <stage> -p <project_dir> "task"     Batch: Claude runs to completion, no questions
  rs next <project_dir>                  Route: what stage should I run next?
  rs catchup <project_dir>               Catch up: structured briefing on project state
  rs init <project_dir> "topic"          Initialize a new project
  rs list                                List available stages
  rs refs                                List available references
  rs status <project_dir>                Show project status

Stages: scout, idea, experiment, write, review, rebuttal, decision

Flags:
  -p, --batch     Run in batch mode (no interactive questions)
  -i, --interactive  Run in interactive mode (default)

Examples:
  rs write ~/my-paper "Complete the Method section"
  rs write -p ~/my-paper "Complete the Method section"    # no questions, just do it
  rs next ~/my-paper
  rs init ~/new-project "Multi-neuron splitting for BaB verification"

Environment:
  RS_MODEL    Claude model (default: sonnet)
  RS_CLAUDE   Claude binary path (default: claude)
  RS_REFS     Comma-separated reference files to include
EOF
    exit 0
}

cmd_list() {
    echo "Available stages:"
    for f in "$STAGES_DIR"/*.md; do
        name=$(basename "$f" .md)
        desc=$(head -3 "$f" | tail -1)
        printf "  %-14s %s\n" "$name" "$desc"
    done
}

cmd_refs() {
    echo "Available references:"
    for f in "$REFS_DIR"/*.md; do
        name=$(basename "$f" .md)
        desc=$(head -3 "$f" | tail -1)
        printf "  %-30s %s\n" "$name" "$desc"
    done
}

cmd_init() {
    local project_dir="$1"
    local topic="${2:-Research project}"

    mkdir -p "$project_dir/knowledge" "$project_dir/paper" "$project_dir/literature" \
             "$project_dir/experiments" "$project_dir/figures"

    for f in brief.md plan.md status.md workspaces.md; do
        if [ ! -f "$project_dir/$f" ]; then
            cp "$TEMPLATES_DIR/$f" "$project_dir/$f"
        fi
    done

    if [ ! -f "$project_dir/experiments/runs.md" ]; then
        cp "$TEMPLATES_DIR/../project/experiments/runs.md" "$project_dir/experiments/runs.md" 2>/dev/null || true
    fi

    if grep -q '\[One sentence' "$project_dir/brief.md" 2>/dev/null; then
        sed -i "s|\[One sentence: what is the research objective?\]|$topic|" "$project_dir/brief.md"
    fi

    echo "Project initialized at $project_dir"
    echo "Next: edit brief.md, plan.md, then run: rs scout $project_dir \"survey the field\""
}

cmd_status() {
    local project_dir="$1"
    echo "=== $(basename "$project_dir") ==="
    echo ""
    if [ -f "$project_dir/status.md" ]; then
        cat "$project_dir/status.md"
    else
        echo "No status.md found"
    fi
    echo ""
    echo "--- Plan ---"
    if [ -f "$project_dir/plan.md" ]; then
        grep -E '^\- \[[ x]\]' "$project_dir/plan.md" 2>/dev/null || echo "(no tasks)"
    fi
    echo ""
    echo "--- Knowledge cards ---"
    ls "$project_dir/knowledge/"*.md 2>/dev/null | wc -l | xargs -I{} echo "{} cards"
}

build_context() {
    local project_dir="$1"
    local context=""

    for doc in brief.md plan.md status.md workspaces.md; do
        if [ -f "$project_dir/$doc" ]; then
            context+="
--- $doc ---
$(cat "$project_dir/$doc")
"
        fi
    done

    if ls "$project_dir/knowledge/"*.md &>/dev/null; then
        context+="
--- Knowledge Cards ---"
        for card in "$project_dir/knowledge/"*.md; do
            context+="
## $(basename "$card" .md)
$(cat "$card")
"
        done
    fi

    echo "$context"
}

build_system() {
    local stage="$1"
    local stage_file="$STAGES_DIR/$stage.md"

    local system_content
    system_content="$(cat "$SYSTEM_PROMPT")

$(cat "$stage_file")"

    if [ -n "${RS_REFS:-}" ]; then
        IFS=',' read -ra REF_NAMES <<< "$RS_REFS"
        for ref_name in "${REF_NAMES[@]}"; do
            local ref_file="$REFS_DIR/${ref_name}.md"
            if [ -f "$ref_file" ]; then
                system_content+="

$(cat "$ref_file")"
            fi
        done
    fi

    echo "$system_content"
}

cmd_run() {
    local mode="$1"
    local stage="$2"
    local project_dir="$3"
    local task="$4"
    local stage_file="$STAGES_DIR/$stage.md"

    if [ ! -f "$stage_file" ]; then
        echo "Error: Unknown stage '$stage'. Run 'rs list' to see available stages."
        exit 1
    fi
    if [ ! -d "$project_dir" ]; then
        echo "Error: Project directory '$project_dir' does not exist."
        exit 1
    fi

    local context
    context="$(build_context "$project_dir")"
    local system_content
    system_content="$(build_system "$stage")"

    local user_prompt="Project directory: $project_dir

$context

---
Task: $task

Work in the project directory. Edit files directly. Read any additional project files (paper/*.tex, etc.) as needed."

    echo "[$mode] stage=$stage project=$(basename "$project_dir") model=$CLAUDE_MODEL"
    echo "Task: $task"
    echo "---"

    if [ "$mode" = "interactive" ]; then
        "$CLAUDE_BINARY" \
            --model "$CLAUDE_MODEL" \
            --dangerously-skip-permissions \
            --system-prompt "$system_content" \
            -p "$user_prompt" \
            --resume
    else
        "$CLAUDE_BINARY" -p \
            --model "$CLAUDE_MODEL" \
            --dangerously-skip-permissions \
            --system-prompt "$system_content" \
            "$user_prompt"
    fi
}

# Parse mode flag from args
parse_and_dispatch() {
    local stage="${1:-}"
    shift || true

    local mode="interactive"
    if [ "${1:-}" = "-p" ] || [ "${1:-}" = "--batch" ]; then
        mode="batch"
        shift
    elif [ "${1:-}" = "-i" ] || [ "${1:-}" = "--interactive" ]; then
        mode="interactive"
        shift
    fi

    local project_dir="${1:?Usage: rs <stage> <project_dir> \"task\"}"
    local task="${2:?Usage: rs <stage> <project_dir> \"task\"}"

    cmd_run "$mode" "$stage" "$project_dir" "$task"
}

case "${1:-}" in
    help|--help|-h|"")
        usage
        ;;
    list)
        cmd_list
        ;;
    refs)
        cmd_refs
        ;;
    init)
        cmd_init "${2:?Usage: rs init <project_dir> [topic]}" "${3:-Research project}"
        ;;
    status)
        cmd_status "${2:?Usage: rs status <project_dir>}"
        ;;
    catchup)
        cmd_run "interactive" "decision" "${2:?Usage: rs catchup <project_dir>}" "I'm returning to this project after a break. Give me a structured catch-up:

1. **Where we are**: Current BaB level, active branch, confidence
2. **What happened**: Summarize the Session Log (last 3-5 entries)
3. **Key decisions**: Most important choices made and why
4. **Pruned branches**: What was tried and abandoned
5. **Evidence state**: What's proven, what's missing
6. **Blockers**: Anything unresolved
7. **Recommended next action**: What to do now, as a concrete rs command

Read status.md (especially Session Log and BaB Tree Snapshot), brief.md, plan.md, and knowledge/ cards. Be concise — this is a catch-up briefing, not a re-analysis."
        ;;
    next)
        cmd_run "interactive" "decision" "${2:?Usage: rs next <project_dir>}" "Read status.md and all project context. Assess current evidence state. Recommend the next stage and specific task. When ready, output a concrete command: rs <stage> <project_dir> \"task\""
        ;;
    *)
        parse_and_dispatch "$@"
        ;;
esac
