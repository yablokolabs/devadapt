# devadapt

[![crates.io](https://img.shields.io/crates/v/devadapt.svg)](https://crates.io/crates/devadapt)

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
- skills scanned from known directories or YAML-configured sources
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
This repo is informed by broader learnings around:
- workflow modes
- routing
- memory and context engineering
- skill selection
- evaluation loops

## How usage works
Yes — the developer should first give `devadapt` some training data.

At this stage, training is lightweight and dataset-driven:
1. create or extend a JSON dataset of developer tasks
2. include the workspace, selected skills, and chosen workflow for each example
3. run `train` to inspect and prepare the dataset
4. run `recommend` to get skill/workflow suggestions for a new task

In the current bootstrap version, `train` is a preparation/summary step rather than a full learned checkpointing pipeline. That will evolve in later versions.

## Detailed usage steps
### 1. Start from the sample dataset
See:
- `examples/devadapt-sample.json`

Each entry looks like:
```json
{
  "task": "Choose the best skills for debugging a backend service issue",
  "workspace": "backend-service",
  "skills": ["session-logs", "skill-selection"],
  "workflow": "plan"
}
```

### 2. Add your own developer examples
Add more records that reflect real usage patterns:
- task description
- workspace or project label
- skills that were actually useful
- workflow mode that worked best

Examples of workflow values:
- `clarify`
- `plan`
- `execute`
- `parallel`
- `review`

### 3. Scan skill directories automatically
By default, `devadapt` can use `devadapt.yaml` to discover skills from configured locations.

Example config:
```yaml
skill_sources:
  - path: ~/.claude/skills
    runtime: claude
    scope: global
  - path: ~/.openclaw/shared
    runtime: openclaw
    scope: shared
  - path: ./skills
    runtime: local
    scope: workspace
```

Run:
```bash
cargo run -- scan-skills --config devadapt.yaml
```

This builds a normalized skill inventory with:
- skill name
- runtime source
- scope
- path
- lightweight description if available

### 4. Run training/dataset preparation
```bash
cargo run -- train --dataset examples/devadapt-sample.json --epochs 10
```

What this currently does:
- loads the dataset
- summarizes the available skills/workflows/workspaces
- prepares the bootstrap training flow

### 5. Ask for a recommendation
```bash
cargo run -- recommend \
  --task "Recommend the right skills for a multi-file backend debugging task" \
  --workspace backend-service
```

Example output:
```json
{
  "skills": ["session-logs", "skill-selection", "tmux"],
  "workflow": "plan"
}
```

### 6. Improve it over time
As the developer adds more examples, `devadapt` can evolve with:
- new skills
- new workflows
- new project types
- personal usage preferences

That is the intended long-term value: the model becomes more useful as the developer’s skill ecosystem grows.

## Dataset schema
Each training example currently uses this shape:

```json
{
  "task": "string",
  "workspace": "string",
  "skills": ["skill-a", "skill-b"],
  "workflow": "review"
}
```

### Field meanings
- `task`: natural-language description of the developer task
- `workspace`: project/repo/context label
- `skills`: skills or capabilities that were actually useful
- `workflow`: the mode that worked best for the task

## Bootstrap vs future roadmap
### Current bootstrap version
- dataset-driven
- summary-based `train` step
- skill-source scanning via YAML-configured directories
- recommendation via lightweight matching/ranking
- Burn model module included as the foundation for the next stage

### Planned next stage
- real learned training loop with Burn
- saved model artifacts/checkpoints
- tighter fusion of scanned skill metadata with usage history
- better feature encoding for tasks/workspaces/skills
- confidence scoring and recommendation explanations
- adaptation from a growing developer history

## Example dataset template
You can start a real dataset with entries like:

```json
[
  {
    "task": "Select the best skills for a backend debugging task",
    "workspace": "backend-service",
    "skills": ["session-logs", "skill-selection"],
    "workflow": "plan"
  },
  {
    "task": "Plan a multi-step repo refactor before implementation",
    "workspace": "frontend-app",
    "skills": ["skill-selection"],
    "workflow": "plan"
  }
]
```

## Library usage
`devadapt` can be used as both:
- a CLI tool
- a Rust library

Example library usage:
```rust
use devadapt::{load_dataset, recommend_from_examples};

let examples = load_dataset("examples/devadapt-sample.json")?;
let recommendation = recommend_from_examples(
    &examples,
    "Review a GitHub PR and inspect failing CI",
    "backend-service",
);
println!("{:?}", recommendation);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Honest scope
This is not a general LLM. It is a focused recommendation model for developer-agent adaptation.
