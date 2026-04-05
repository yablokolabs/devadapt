use std::fs;

use devadapt::data::{TrainingExample, load_skill_config, scan_skill_sources, summarize_dataset};
use devadapt::recommend::recommend_from_examples;

#[test]
fn recommendation_returns_a_workflow() {
    let examples = vec![
        TrainingExample {
            task: "Recommend the best skills for backend debugging".into(),
            workspace: "backend-service".into(),
            skills: vec!["session-logs".into(), "skill-selection".into()],
            workflow: "plan".into(),
        },
        TrainingExample {
            task: "Plan a multi-step refactor before coding".into(),
            workspace: "frontend-app".into(),
            skills: vec!["skill-selection".into()],
            workflow: "plan".into(),
        },
    ];

    let rec = recommend_from_examples(
        &examples,
        "Recommend skills for a backend debugging issue",
        "backend-service",
    );
    assert_eq!(rec.workflow, "plan");
    assert!(!rec.skills.is_empty());
}

#[test]
fn dataset_summary_collects_unique_values() {
    let examples = vec![
        TrainingExample {
            task: "a".into(),
            workspace: "x".into(),
            skills: vec!["session-logs".into()],
            workflow: "plan".into(),
        },
        TrainingExample {
            task: "b".into(),
            workspace: "y".into(),
            skills: vec!["tmux".into(), "session-logs".into()],
            workflow: "parallel".into(),
        },
    ];
    let summary = summarize_dataset(&examples);
    assert_eq!(summary.examples, 2);
    assert!(summary.unique_skills.contains(&"session-logs".to_string()));
    assert!(summary.workflows.contains(&"plan".to_string()));
}

#[test]
fn skill_config_scans_without_crashing() {
    let config = load_skill_config("devadapt.yaml").unwrap();
    let _skills = scan_skill_sources(&config);
}

#[test]
fn scan_extracts_frontmatter_description() {
    let temp = tempfile::tempdir().unwrap();
    let skill_dir = temp.path().join("debug-skill");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: Debug Skill\ndescription: Helps debug complex failures quickly\n---\n\n# Debug Skill\n",
    )
    .unwrap();

    let config_path = temp.path().join("devadapt.yaml");
    fs::write(
        &config_path,
        format!(
            "skill_sources:\n  - path: {}\n    runtime: claude\n    scope: global\n",
            temp.path().display()
        ),
    )
    .unwrap();

    let config = load_skill_config(config_path.to_str().unwrap()).unwrap();
    let skills = scan_skill_sources(&config);
    assert_eq!(skills.len(), 1);
    assert_eq!(
        skills[0].description.as_deref(),
        Some("Helps debug complex failures quickly")
    );
}
