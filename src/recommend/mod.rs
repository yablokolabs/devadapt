use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::{SkillRecord, TrainingExample};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recommendation {
    pub skills: Vec<String>,
    pub workflow: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainedRecommendation {
    pub recommendation: Recommendation,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainedProfile {
    pub skill_scores: Vec<(String, usize)>,
    pub workflow_scores: Vec<(String, usize)>,
    pub known_workspaces: Vec<String>,
    pub scanned_skills: Vec<SkillRecord>,
    pub epochs: usize,
}

pub fn recommend_from_examples(
    examples: &[TrainingExample],
    task: &str,
    workspace: &str,
) -> Recommendation {
    recommend_with_skills(examples, &[], task, workspace).recommendation
}

pub fn recommend_with_skills(
    examples: &[TrainingExample],
    scanned_skills: &[SkillRecord],
    task: &str,
    workspace: &str,
) -> ExplainedRecommendation {
    let task_lc = task.to_lowercase();
    let workspace_lc = workspace.to_lowercase();

    let mut skill_scores: HashMap<String, usize> = HashMap::new();
    let mut workflow_scores: HashMap<String, usize> = HashMap::new();
    let mut reasons = Vec::new();

    for ex in examples {
        let mut score = 0usize;
        if ex.workspace.to_lowercase() == workspace_lc {
            score += 3;
            reasons.push(format!("matched prior workspace pattern: {}", ex.workspace));
        }
        for token in ex.task.to_lowercase().split_whitespace() {
            if task_lc.contains(token) {
                score += 1;
            }
        }
        if score == 0 {
            continue;
        }
        for skill in &ex.skills {
            *skill_scores.entry(skill.clone()).or_default() += score;
        }
        *workflow_scores.entry(ex.workflow.clone()).or_default() += score;
    }

    for scanned in scanned_skills {
        let mut score = 0usize;
        if task_lc.contains(&scanned.name.to_lowercase().replace('-', " "))
            || task_lc.contains(&scanned.name.to_lowercase())
        {
            score += 2;
        }
        if let Some(description) = &scanned.description {
            for token in description.to_lowercase().split_whitespace().take(20) {
                if token.len() > 4 && task_lc.contains(token) {
                    score += 1;
                }
            }
        }
        if score > 0 {
            *skill_scores.entry(scanned.name.clone()).or_default() += score;
            reasons.push(format!(
                "matched scanned skill metadata: {} ({})",
                scanned.name, scanned.runtime
            ));
        }
    }

    let mut ranked_skills: Vec<(String, usize)> = skill_scores.into_iter().collect();
    ranked_skills.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut ranked_workflows: Vec<(String, usize)> = workflow_scores.into_iter().collect();
    ranked_workflows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let workflow = ranked_workflows
        .first()
        .map(|x| x.0.clone())
        .unwrap_or_else(|| "execute".to_string());

    if reasons.is_empty() {
        reasons.push(
            "fell back to default lightweight ranking with limited matching signal".to_string(),
        );
    }

    ExplainedRecommendation {
        recommendation: Recommendation {
            skills: ranked_skills.into_iter().take(3).map(|x| x.0).collect(),
            workflow,
        },
        reasons,
    }
}

pub fn build_trained_profile(
    examples: &[TrainingExample],
    scanned_skills: &[SkillRecord],
    epochs: usize,
) -> TrainedProfile {
    let mut skill_scores: HashMap<String, usize> = HashMap::new();
    let mut workflow_scores: HashMap<String, usize> = HashMap::new();
    let mut workspaces = Vec::new();

    for ex in examples {
        if !workspaces.contains(&ex.workspace) {
            workspaces.push(ex.workspace.clone());
        }
        for skill in &ex.skills {
            *skill_scores.entry(skill.clone()).or_default() += 1;
        }
        *workflow_scores.entry(ex.workflow.clone()).or_default() += 1;
    }

    let mut skill_scores: Vec<(String, usize)> = skill_scores.into_iter().collect();
    skill_scores.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let mut workflow_scores: Vec<(String, usize)> = workflow_scores.into_iter().collect();
    workflow_scores.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    workspaces.sort();

    TrainedProfile {
        skill_scores,
        workflow_scores,
        known_workspaces: workspaces,
        scanned_skills: scanned_skills.to_vec(),
        epochs,
    }
}
