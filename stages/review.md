# Stage: Review

Independent skeptical audit of the paper before submission. Read as if you are a hostile reviewer looking for the strongest rejection reason.

## When to Use

- Draft is "complete" and needs quality check before submission
- After major revisions
- Before final submission

## Workflow

1. **Five-minute pass** (simulate reviewer first impression):
   - What does the paper claim in one sentence?
   - What is the main problem?
   - What does the best prior method fail to do?
   - What does this method change?
   - What is the strongest evidence?
   - Does the main claim survive this quick pass?
2. **Audit each dimension**:
   - Research question/value: Is the problem worth solving?
   - Novelty/positioning: Is this genuinely new or a rename?
   - Method fit: Does the method actually address the stated problem?
   - Evidence sufficiency: Do the experiments support the claims?
   - Experimental validity: Fair baselines? Significance testing? Confounders?
   - Claim scope: Are claims appropriately bounded?
   - Writing defensibility: Would a reviewer find unclear arguments?
3. **Classify each issue found**:
   - **Critical**: blocks acceptance (missing baseline, unsupported claim, flawed experiment)
   - **Major**: weakens the paper significantly (incomplete ablation, unclear method description)
   - **Minor**: polish issues (typos, figure readability, citation format)
4. **For each critical/major issue, prescribe**:
   - What needs to change
   - What evidence would fix it
   - Whether it requires new experiments or just rewriting
5. **Produce verdict**: accept-as-is / minor-revision / major-revision / rethink

## Key Heuristics

- **Adversarial mindset**: look for the strongest rejection reason
- **Prefer narrowing over defending**: if a claim is weak, narrow it rather than defend it
- **Route from evidence, not rhetoric**: a well-written weak paper is still weak
- **Check the gap between abstract claims and actual results**
- **Supported / Partial / Unsupported**: classify every main claim

## Finalization Checklist

- [ ] All claims backed by specific evidence (table/figure/result)
- [ ] No invented or placeholder numbers
- [ ] Bibliography complete and correctly formatted
- [ ] Figures readable, properly labeled, referenced in text
- [ ] Contribution list matches what was actually demonstrated
- [ ] Limitations section is honest
- [ ] Author information correct (for camera-ready)
- [ ] Page limit respected
- [ ] Supplementary materials referenced

## Output

A review report listing issues by severity. Updated paper file with fixes for minor issues. Updated `status.md` with verdict and remaining action items.
