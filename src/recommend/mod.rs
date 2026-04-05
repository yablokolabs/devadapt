use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::TrainingExample;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Recommendation {
    pub skills: Vec<String>,
    pub workflow: String,
}

pub fn recommend_from_examples(
    examples: &[TrainingExample],
    task: &str,
    workspace: &str,
) -> Recommendation {
    let task_lc = task.to_lowercase();
    let workspace_lc = workspace.to_lowercase();

    let mut skill_scores: HashMap<String, usize> = HashMap::new();
    let mut workflow_scores: HashMap<String, usize> = HashMap::new();

    for ex in examples {
        let mut score = 0usize;
        if ex.workspace.to_lowercase() == workspace_lc {
            score += 3;
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

    let mut ranked_skills: Vec<(String, usize)> = skill_scores.into_iter().collect();
    ranked_skills.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let workflow = workflow_scores
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)))
        .map(|x| x.0)
        .unwrap_or_else(|| "execute".to_string());

    Recommendation {
        skills: ranked_skills.into_iter().take(3).map(|x| x.0).collect(),
        workflow,
    }
}
