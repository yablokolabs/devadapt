use std::fs;

use anyhow::Result;
use clap::{Parser, Subcommand};
use devadapt::data::{load_dataset, load_skill_config, scan_skill_sources, summarize_dataset};
use devadapt::recommend::{build_trained_profile, recommend_with_skills};

#[derive(Debug, Parser)]
#[command(name = "devadapt")]
#[command(about = "Developer adaptation model for skill and workflow recommendation")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Train {
        #[arg(long)]
        dataset: String,
        #[arg(long, default_value_t = 10)]
        epochs: usize,
        #[arg(long, default_value = "devadapt.yaml")]
        config: String,
        #[arg(long, default_value = "devadapt-profile.json")]
        output: String,
    },
    ScanSkills {
        #[arg(long, default_value = "devadapt.yaml")]
        config: String,
    },
    Recommend {
        #[arg(long)]
        task: String,
        #[arg(long)]
        workspace: String,
        #[arg(long, default_value = "examples/devadapt-sample.json")]
        dataset: String,
        #[arg(long, default_value = "devadapt.yaml")]
        config: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Train {
            dataset,
            epochs,
            config,
            output,
        } => train_cmd(&dataset, epochs, &config, &output),
        Command::ScanSkills { config } => scan_skills_cmd(&config),
        Command::Recommend {
            task,
            workspace,
            dataset,
            config,
        } => recommend_cmd(&dataset, &config, &task, &workspace),
    }
}

fn train_cmd(dataset: &str, epochs: usize, config: &str, output: &str) -> Result<()> {
    let examples = load_dataset(dataset)?;
    let summary = summarize_dataset(&examples);
    let skill_config = load_skill_config(config)?;
    let scanned_skills = scan_skill_sources(&skill_config);
    let profile = build_trained_profile(&examples, &scanned_skills, epochs);
    fs::write(output, serde_json::to_string_pretty(&profile)?)?;

    println!("dataset_examples={}", summary.examples);
    println!("epochs={epochs}");
    println!("scanned_skills={}", scanned_skills.len());
    println!(
        "skills={}",
        serde_json::to_string_pretty(&summary.unique_skills)?
    );
    println!(
        "workflows={}",
        serde_json::to_string_pretty(&summary.workflows)?
    );
    println!(
        "workspaces={}",
        serde_json::to_string_pretty(&summary.workspaces)?
    );
    println!("saved_profile={output}");
    println!("status=v0.2-profile-ready");
    Ok(())
}

fn scan_skills_cmd(config: &str) -> Result<()> {
    let config = load_skill_config(config)?;
    let skills = scan_skill_sources(&config);
    println!("{}", serde_json::to_string_pretty(&skills)?);
    Ok(())
}

fn recommend_cmd(dataset: &str, config: &str, task: &str, workspace: &str) -> Result<()> {
    let examples = load_dataset(dataset)?;
    let skill_config = load_skill_config(config)?;
    let scanned_skills = scan_skill_sources(&skill_config);
    let recommendation = recommend_with_skills(&examples, &scanned_skills, task, workspace);
    println!("{}", serde_json::to_string_pretty(&recommendation)?);
    Ok(())
}
