use std::collections::BTreeSet;
use std::fs;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainingExample {
    pub task: String,
    pub workspace: String,
    pub skills: Vec<String>,
    pub workflow: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetSummary {
    pub examples: usize,
    pub unique_skills: Vec<String>,
    pub workflows: Vec<String>,
    pub workspaces: Vec<String>,
}

pub fn load_dataset(path: &str) -> Result<Vec<TrainingExample>> {
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn summarize_dataset(examples: &[TrainingExample]) -> DatasetSummary {
    let mut skills = BTreeSet::new();
    let mut workflows = BTreeSet::new();
    let mut workspaces = BTreeSet::new();

    for example in examples {
        workspaces.insert(example.workspace.clone());
        workflows.insert(example.workflow.clone());
        for skill in &example.skills {
            skills.insert(skill.clone());
        }
    }

    DatasetSummary {
        examples: examples.len(),
        unique_skills: skills.into_iter().collect(),
        workflows: workflows.into_iter().collect(),
        workspaces: workspaces.into_iter().collect(),
    }
}
