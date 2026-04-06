# Stage: Write

Draft or complete paper sections by editing files directly. No intermediate outline/bundle pipeline — work on the actual .tex or .md.

## When to Use

- Paper has incomplete sections that need writing
- New results need to be incorporated into the manuscript
- Related work section needs drafting
- Figures or tables need to be added

## Workflow

1. **Read everything first**:
   - All `knowledge/` cards (design decisions, related work notes, constraints)
   - The actual paper file (.tex)
   - Current `plan.md` to know what's done and what's not
   - The bibliography file
2. **Identify what to write**: Which sections are incomplete? What evidence supports them?
3. **Edit the file directly**: Insert LaTeX content into the .tex file at the right location
4. **For each section, answer the reader's question**:
   - Introduction: Why does this problem matter? What's the gap? What did we do?
   - Related Work: What's closest? What assumption/mechanism/scope differs?
   - Method: What changed? (faithful to implementation, no fictional components)
   - Experiments: Setup → main comparison → ablations → error analysis → limitations
   - Conclusion: What was actually shown (not the highest hope)
5. **Citation discipline**: Only cite papers that exist in the .bib file. If a citation is needed, add it to .bib first.
6. **Update plan.md**: Mark completed sections.

## Key Heuristics

- **Underclaim in prose, overdeliver in evidence**
- **A figure/table is an argument**, not decoration
- **Front-load value**: title → abstract → intro opening → first figure → main result
- **Equations**: define symbols, tie to implementation, no decorative math
- **Reviewer-first**: simulate a 5-minute scan — does the main claim survive?
- **Limitations are concrete**: what was not tested, what remained unstable, which claims were downgraded
- **No fiction**: if evidence doesn't exist, say "to be filled after experiments" — don't make up numbers

## Section Contracts

### Introduction
Answer: why now, what baseline does, what bottleneck remains, what we changed, what evidence supports it, contribution scope.

### Related Work
Show: which prior routes are closest, which assumption/mechanism/scope differs, why this isn't a rename.

### Method
Keep faithful to implementation. No omitted caveats. If equations used: define symbols, tie back to code.

### Experiments
Flow: setup/contract → main comparison → ablations → robustness/error analysis → limitations from results. All numbers point to specific tables/figures.

### Conclusion
Summarize what was actually shown. Not a restatement of the introduction.

## Output

The paper file (.tex/.md) with completed sections. Updated `plan.md` marking what's done.
