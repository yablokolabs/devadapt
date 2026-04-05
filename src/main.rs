use anyhow::Result;
use clap::{Parser, Subcommand};
use devadapt::data::{load_dataset, summarize_dataset};
use devadapt::recommend::recommend_from_examples;

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
    },
    Recommend {
        #[arg(long)]
        task: String,
        #[arg(long)]
        workspace: String,
        #[arg(long, default_value = "examples/devadapt-sample.json")]
        dataset: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Train { dataset, epochs } => train_cmd(&dataset, epochs),
        Command::Recommend {
            task,
            workspace,
            dataset,
        } => recommend_cmd(&dataset, &task, &workspace),
    }
}

fn train_cmd(dataset: &str, epochs: usize) -> Result<()> {
    let examples = load_dataset(dataset)?;
    let summary = summarize_dataset(&examples);
    println!("dataset_examples={}", summary.examples);
    println!("epochs={epochs}");
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
    println!("status=bootstrap-ready");
    Ok(())
}

fn recommend_cmd(dataset: &str, task: &str, workspace: &str) -> Result<()> {
    let examples = load_dataset(dataset)?;
    let recommendation = recommend_from_examples(&examples, task, workspace);
    println!("{}", serde_json::to_string_pretty(&recommendation)?);
    Ok(())
}
