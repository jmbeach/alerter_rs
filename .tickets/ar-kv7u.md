---
id: ar-kv7u
status: open
deps: [ar-8a8g]
links: []
created: 2026-03-08T00:08:00Z
type: task
priority: 2
parent: ar-s7vk
tags: [workflow:plan-and-implement-tdd]
---
# [example] Run agent team to create the tests

Use tk on the test epic id to see what tests need created.

Create an agent team to write failing tests in parallel. Follow these rules:

TEAM SIZING:
- Count the number of test tickets (tk) under the epic.
- Spawn ceil(ticket_count / 5) teammates, minimum 2, maximum 5.
- Start each teammate with model "opus".

WORK PARTITIONING (file ownership):
- Assign each teammate a disjoint set of test tickets. No two teammates share a ticket.
- Each teammate OWNS the test files for their assigned tickets. No other teammate may edit those files.
- The lead must state the file ownership mapping in each teammate's spawn prompt.

SPAWN PROMPT (include all of this for every teammate):
- The openspec artifact paths for this branch (list them explicitly - the ones with plan file: openspec/changes/example-binary-and-integration-tests/proposal.md).
- The test framework and conventions used in this project (detect from existing tests).
- The specific ticket IDs and descriptions assigned to this teammate.
- The exact file paths this teammate is responsible for creating.
- The instruction: "Write tests that FAIL. Do not write any implementation code."

WORKFLOW:
1. The lead partitions tickets across teammates and spawns them with detailed prompts.
2. Teammates work in parallel, each writing tests only for their assigned tickets/files.
3. After all teammates finish, the lead does a consistency review across all test files
   to check for duplicated setup, inconsistent naming, or missing coverage.
4. The lead marks each ticket as done only after verifying the tests exist and fail.

DO NOT WRITE THE IMPLEMENTATION TO PASS THE TESTS: We'll do that later.

