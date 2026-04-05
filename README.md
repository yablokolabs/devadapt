# devadapt

A Burn-powered developer adaptation model for skill and workflow recommendation.

## What it is
`devadapt` helps a developer-facing agent learn which skills, workflows, and routing choices are most useful for a specific developer across tools such as:
- Claude
- Codex
- Cursor
- OpenClaw
- custom local agents

## Core idea
Instead of building another general coding assistant, `devadapt` focuses on the decision layer:
- which skills should be activated?
- which workflow mode fits this task?
- should the system clarify, plan, execute directly, or parallelize?
- which developer patterns are emerging over time?

## Practical usefulness
This can help:
- developers with growing skill libraries
- teams using multiple agent tools
- agent runtimes that need better skill/workflow selection
- systems trying to avoid context saturation from irrelevant skills

## What it learns from
- task text
- workspace tags
- available skills and their descriptions
- prior successful recommendations
- developer corrections and preferences

## Initial scope
v0.1 focuses on:
- skill recommendation
- workflow mode recommendation
- JSON dataset format
- train/eval/recommend CLI
- Rust-native model implementation with Burn

## Influence
This repo is informed by learnings captured in:
- `bots_soul/shared-docs-reference.md`
- shared docs around workflow modes, routing, memory, context engineering, skill selection, and evaluation loops

## Example workflow
### Train from a small dataset
```bash
cargo run -- train --dataset examples/devadapt-sample.json --epochs 10
```

### Recommend skills for a task
```bash
cargo run -- recommend \
  --task "Review a GitHub PR and check failing CI" \
  --workspace watchtower-backend
```

## Honest scope
This is not a general LLM. It is a focused recommendation model for developer-agent adaptation.
