---
id: ar-6m5u
status: closed
deps: [ar-ukpf]
links: []
created: 2026-03-08T00:08:00Z
type: task
priority: 2
parent: ar-s7vk
tags: [workflow:plan-and-implement-tdd]
---
# [example] Run an agent team to implement the implementation tickets.

Use tk to find children of <impl-epic-id> to see the tasks hat need implemented.

Create an agent team to implement the feature in parallel. Follow these rules:

TEAM SIZING:
- Count the number of implementation tickets under the epic.
- Spawn ceil(ticket_count / 5) teammates, minimum 2, maximum 5.
- Start each teammate with model "opus".

PLAN APPROVAL (required):
- Spawn teammates with plan approval required.
- Each teammate must submit a plan before writing any code.
- The lead reviews each plan and rejects any that:
  - Modify files owned by another teammate.
  - Don't reference the relevant failing tests.
  - Introduce unnecessary abstractions beyond what the tests require.
- Only after the lead approves a plan does the teammate begin implementation.

WORK PARTITIONING (file ownership):
- Assign each teammate a disjoint set of implementation tickets.
- Each teammate OWNS the source files for their assigned tickets. No two teammates edit the same file.
- If two tickets require changes to the same file, assign them to the same teammate.
- The lead must state the file ownership mapping in each teammate's spawn prompt.

SPAWN PROMPT (include all of this for every teammate):
- The openspec artifact paths for this branch (list them explicitly - the ones with plan file openspec/changes/example-binary-and-integration-tests/proposal.md).
- The paths to the failing test files relevant to this teammate's tickets.
- The specific ticket IDs and descriptions assigned to this teammate.
- The exact source file paths this teammate is responsible for.
- The instruction: "Make the failing tests pass. Do not modify test files."

WORKFLOW:
1. The lead partitions tickets, maps file ownership, and spawns teammates with detailed prompts.
2. Each teammate submits a plan. The lead approves or rejects with feedback.
3. Approved teammates implement in parallel, each only touching their own files.
4. After all teammates finish, the lead runs the full test suite.
5. If tests fail, the lead identifies which teammate's files are involved and
   sends them a message with the failure output to fix.
6. The lead marks each ticket as done only after its tests pass.

