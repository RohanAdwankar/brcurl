---
name: reflect
description: Use when a user asks the agent to reflect on a mistake, understand what went wrong with recent changes, and record condensed lessons in the nearest AGENTS.md for future turns.
---

# Reflect

Use this skill after a mistake, misread requirement, or avoidable detour.

## Goal

Turn a failure into a small durable behavior change for future agents working in the same repository.

## Workflow

1. Compare the user's request against the changes that were actually made.
2. State the mismatch plainly:
   - what the user asked for
   - what the agent did instead
   - why that difference mattered
3. Revert or narrow the incorrect changes before adding anything new.
4. Extract 1-3 short lessons that are specific, behavioral, and reusable.
5. Write those lessons into the nearest `AGENTS.md` that governs the current workspace.

## Lesson Quality Bar

Good lessons are:

- short enough to scan quickly
- phrased as future operating rules
- tied to the user's preferences, not generic engineering advice

Avoid:

- long retrospectives
- excuses
- project-specific implementation detail that will age badly

## AGENTS.md Update Rules

- Preserve existing instructions.
- Append a small `Reflection Notes` section if one does not exist.
- Add only the minimum bullets needed.
- Prefer rules like `Ask before adding new CLI surface area for test workflows.` over long explanations.

## Example Lessons

- Keep test harnesses out of the main executable unless the user explicitly asks for a product feature.
- Ask before changing CLI shape when multiple interpretations are possible.
