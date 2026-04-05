use devadapt::data::{TrainingExample, load_skill_config, scan_skill_sources, summarize_dataset};
use devadapt::recommend::recommend_from_examples;

#[test]
fn recommendation_returns_a_workflow() {
    let examples = vec![
        TrainingExample {
            task: "Review a GitHub PR and inspect CI failures".into(),
            workspace: "backend-service".into(),
            skills: vec!["github".into(), "session-logs".into()],
            workflow: "review".into(),
        },
        TrainingExample {
            task: "Plan a multi-step refactor before coding".into(),
            workspace: "frontend-app".into(),
            skills: vec!["skill-selection".into()],
            workflow: "plan".into(),
        },
    ];

    let rec = recommend_from_examples(&examples, "Review PR and failing CI", "backend-service");
    assert_eq!(rec.workflow, "review");
    assert!(!rec.skills.is_empty());
}

#[test]
fn dataset_summary_collects_unique_values() {
    let examples = vec![
        TrainingExample {
            task: "a".into(),
            workspace: "x".into(),
            skills: vec!["github".into()],
            workflow: "review".into(),
        },
        TrainingExample {
            task: "b".into(),
            workspace: "y".into(),
            skills: vec!["tmux".into(), "github".into()],
            workflow: "parallel".into(),
        },
    ];
    let summary = summarize_dataset(&examples);
    assert_eq!(summary.examples, 2);
    assert!(summary.unique_skills.contains(&"github".to_string()));
    assert!(summary.workflows.contains(&"review".to_string()));
}

#[test]
fn skill_config_scans_without_crashing() {
    let config = load_skill_config("devadapt.yaml").unwrap();
    let _skills = scan_skill_sources(&config);
}
