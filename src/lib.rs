//! `devadapt` is a developer adaptation library focused on recommending skills and workflow modes.
//!
//! It is designed for developer-agent ecosystems where the useful skill set evolves over time.
//!
//! # Example
//! ```no_run
//! use devadapt::data::load_dataset;
//! use devadapt::recommend::recommend_from_examples;
//!
//! let examples = load_dataset("examples/devadapt-sample.json").unwrap();
//! let recommendation = recommend_from_examples(
//!     &examples,
//!     "Review a GitHub PR and inspect failing CI",
//!     "backend-service",
//! );
//! assert!(!recommendation.skills.is_empty());
//! ```

pub mod data;
pub mod model;
pub mod recommend;

pub use data::{
    DatasetSummary, SkillConfig, SkillRecord, SkillSource, TrainingExample, load_dataset,
    load_skill_config, scan_skill_sources, summarize_dataset,
};
pub use recommend::{Recommendation, recommend_from_examples};
