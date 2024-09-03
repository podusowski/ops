mod command;
mod plan;
mod run;

use clap::{Parser, Subcommand};
use plan::Plan;

#[derive(Parser)]
#[command(version, about, long_about = None, infer_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Enter the shell.
    Shell,
    /// Execute all missions.
    Execute {
        /// Filter missions by pattern.
        pattern: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let plan = Plan::from_file("cio.yaml")?;

    match cli.command {
        Commands::Execute { pattern } => execute(plan, pattern),
        Commands::Shell => shell(plan),
    }
}

fn execute(plan: Plan, pattern: Option<String>) -> anyhow::Result<()> {
    let mut failed = Vec::new();

    let missions = plan.missions.into_iter().filter(|(name, _)| {
        if let Some(pattern) = &pattern {
            name.contains(pattern)
        } else {
            true
        }
    });

    for (name, mission) in missions {
        println!("Launching '{}'", name);
        let status = run::execute(mission)?;

        if !status.success() {
            println!("Mission '{}' failed with status: {:?}", name, status.code());
            failed.push(name);
        }
    }

    if failed.is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Some missions have failed: {:?}", failed))
    }
}

fn shell(plan: Plan) -> anyhow::Result<()> {
    run::shell(plan.shell.ok_or(anyhow::anyhow!(
        "missing 'shell' definition in your Ops.yaml"
    ))?)?;
    Ok(())
}
