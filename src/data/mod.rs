use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillSource {
    pub path: String,
    pub runtime: String,
    pub scope: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillConfig {
    pub skill_sources: Vec<SkillSource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillRecord {
    pub name: String,
    pub runtime: String,
    pub scope: String,
    pub path: String,
    pub description: Option<String>,
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

pub fn load_skill_config(path: &str) -> Result<SkillConfig> {
    let text = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&text)?)
}

pub fn scan_skill_sources(config: &SkillConfig) -> Vec<SkillRecord> {
    let mut out = Vec::new();
    for source in &config.skill_sources {
        let expanded = expand_tilde(&source.path);
        let root = PathBuf::from(expanded);
        if !root.exists() || !root.is_dir() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(&root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let name = path
                    .file_name()
                    .and_then(|x| x.to_str())
                    .unwrap_or("unknown-skill")
                    .to_string();
                let description = extract_skill_description(&path);
                out.push(SkillRecord {
                    name,
                    runtime: source.runtime.clone(),
                    scope: source.scope.clone(),
                    path: path.display().to_string(),
                    description,
                });
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name).then_with(|| a.runtime.cmp(&b.runtime)));
    out
}

fn expand_tilde(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{home}/{rest}");
        }
    }
    path.to_string()
}

fn extract_skill_description(path: &Path) -> Option<String> {
    for candidate in ["SKILL.md", "README.md", "readme.md"] {
        let file = path.join(candidate);
        if !file.exists() {
            continue;
        }
        if let Ok(text) = fs::read_to_string(file) {
            for line in text.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    return Some(trimmed.to_string());
                }
            }
        }
    }
    None
}
