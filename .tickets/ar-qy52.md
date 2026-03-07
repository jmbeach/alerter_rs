---
id: ar-qy52
status: closed
deps: [ar-vdmr]
links: []
created: 2026-03-07T22:25:33Z
type: task
priority: 2
parent: ar-540c
tags: [workflow:plan-and-implement-tdd]
---
# [init] Address code review feedback

Scan the codebase for comments containing "AI_REVIEW".
If there are no findings, skip this step.

If there are findings that need to be addressed:

Use subagents (Task tool) — NOT an agent team — to fix findings in parallel.

GROUPING:
- Parse AI_REVIEW findings and group them by file (or group of closely related files).
- Spawn one subagent per group. No two subagents should touch the same file.

SUBAGENT PROMPT (include all of this for every subagent):
- The specific AI_REVIEW comments assigned to this subagent (paste the full text).
- The file paths this subagent is responsible for.
- The instruction: "Address these review findings. Do not modify files outside your assignment."

WORKFLOW:
1. Group findings by file.
2. Spawn all subagents in parallel (multiple Task tool calls in a single message).
3. When all subagents return, review the changes for correctness.

